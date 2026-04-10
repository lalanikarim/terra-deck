//! Poker Card RPG - TUI Application
//! Full game integration with combat loop, deck management, and turns

use std::io::stdout;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod game_state;
mod ui;

use game_state::*;
use ui::*;

fn main() {
    // Setup terminal for TUI
    setup_terminal().expect("Failed to setup terminal");

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    // Initialize game
    let mut game = GameSession::new();
    game.start_new_game();

    println!("🎴 Poker Card RPG - Terra-Deck");
    println!(
        "Game initialized with {} player cards and {} opponent cards",
        game.player_hand.len(),
        game.opponent_hand.len()
    );

    // Main game loop
    loop {
        // Draw frame
        terminal
            .draw(|frame| {
                render_game(frame, &game, game.is_player_turn(), game.is_opponent_turn());
            })
            .expect("Failed to draw frame");

        // Poll for input events
        if event::poll(Duration::from_millis(50)).expect("Failed to poll events") {
            if let std::io::Result::Ok(Event::Key(key)) = event::read() {
                match handle_key(&mut game, key) {
                    Some(should_quit) => {
                        if should_quit {
                            break;
                        }
                    }
                    None => {}
                }
            }
        }

        // Check if we should quit
        if game.loop_state == GameStateLoop::Quit {
            break;
        }
    }

    // Cleanup terminal
    cleanup_terminal().expect("Failed to cleanup terminal");
}

/// Setup terminal for raw mode
fn setup_terminal() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

/// Cleanup terminal
fn cleanup_terminal() -> std::io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Handle key input based on current game state
/// Returns Some(true) if should quit, Some(false) or None otherwise
fn handle_key(game: &mut GameSession, key: KeyEvent) -> Option<bool> {
    // If opponent turn or resolving combat, ignore player input
    if game.is_opponent_turn() || game.is_resolving() {
        return None;
    }

    // Game over - only handle restart or quit
    if game.is_game_over() {
        match key.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                game.start_new_game();
                println!("Game restarted!");
                return Some(false);
            }
            KeyCode::Char('q') => {
                println!("Quitting game...");
                return Some(true);
            }
            _ => return None,
        }
    }

    // Only handle key press events
    match key.kind {
        crossterm::event::KeyEventKind::Press => {}
        _ => return None,
    }

    match key.code {
        KeyCode::Char('q') => {
            println!("Quitting game...");
            return Some(true);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            move_selection_left(game);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            move_selection_right(game);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            handle_enter(game);
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            confirm_attack(game);
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            cancel_attack(game);
        }
        _ => {}
    }

    None
}

/// Move selection left
fn move_selection_left(game: &mut GameSession) {
    if game.loop_state == GameStateLoop::SelectPlayerCard {
        if let Some(_idx) = game.selected_player_card.index {
            game.selected_player_card
                .previous(game.player_hand.len().saturating_sub(1));
        }
    } else if game.loop_state == GameStateLoop::SelectOpponentTarget {
        if let Some(_idx) = game.selected_opponent_card.index {
            game.selected_opponent_card
                .previous(game.opponent_hand.len().saturating_sub(1));
        }
    }
}

/// Move selection right
fn move_selection_right(game: &mut GameSession) {
    if game.loop_state == GameStateLoop::SelectPlayerCard {
        if game.player_hand.is_empty() {
            return;
        }
        if game.selected_player_card.index.is_none() {
            game.selected_player_card = game_core::turn_state::SelectedCard::new(0);
        }
        game.selected_player_card
            .next(game.player_hand.len().saturating_sub(1));
    } else if game.loop_state == GameStateLoop::SelectOpponentTarget {
        if game.opponent_hand.is_empty() {
            return;
        }
        if game.selected_opponent_card.index.is_none() {
            game.selected_opponent_card = game_core::turn_state::SelectedCard::new(0);
        }
        game.selected_opponent_card
            .next(game.opponent_hand.len().saturating_sub(1));
    }
}

/// Handle Enter key (advance to next step)
fn handle_enter(game: &mut GameSession) {
    match game.loop_state {
        GameStateLoop::SelectPlayerCard => {
            if game.player_hand.is_empty() {
                return;
            }
            if let Some(player_idx) = game.selected_player_card.index {
                if player_idx < game.player_hand.len() {
                    game.loop_state = game.loop_state.advance_after_player_card_selected();
                    if !game.opponent_hand.is_empty() {
                        game.selected_opponent_card = game_core::turn_state::SelectedCard::new(0);
                    }
                }
            }
        }
        GameStateLoop::SelectOpponentTarget => {
            if game.selected_opponent_card.index.is_some() {
                if let Some(opponent_idx) = game.selected_opponent_card.index {
                    if opponent_idx < game.opponent_hand.len() {
                        game.loop_state = game.loop_state.advance_after_target_selected();
                    }
                }
            }
        }
        GameStateLoop::ConfirmAttack => {
            confirm_attack(game);
        }
        _ => {}
    }
}

/// Confirm attack and resolve combat
fn confirm_attack(game: &mut GameSession) {
    if let (Some(player_idx), Some(opponent_idx)) = (
        game.selected_player_card.index,
        game.selected_opponent_card.index,
    ) {
        // Ensure indices are valid
        let valid_player_idx = player_idx.min(game.player_hand.len().saturating_sub(1));
        let valid_opponent_idx = opponent_idx.min(game.opponent_hand.len().saturating_sub(1));

        let result = game.resolve_player_attack(valid_player_idx, valid_opponent_idx);
        println!(
            "Combat: You dealt {} damage, took {} damage",
            result.player_dmg, result.opponent_dmg
        );

        // Reset for next turn
        game.loop_state = game.loop_state.reset_to_player_turn();
        if !game.player_hand.is_empty() {
            game.selected_player_card = game_core::turn_state::SelectedCard::new(0);
        }
        game.selected_opponent_card = game_core::turn_state::SelectedCard::none();
        game.current_combat_round += 1;
    }
}

/// Cancel attack and go back
fn cancel_attack(game: &mut GameSession) {
    game.loop_state = game.loop_state.cancel_target_selection();
    game.selected_opponent_card = game_core::turn_state::SelectedCard::none();
}
