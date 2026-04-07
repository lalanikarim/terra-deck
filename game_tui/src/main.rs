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
    let mut game = FullGameState::new();
    game.start_new_game();

    println!("🎴 Poker Card RPG - Terra-Deck");
    println!("Game initialized with {} player cards and {} opponent cards",
          game.player_hand.len(), game.opponent_hand.len());

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
                match handle_key(&game, key) {
                    Some(next_game) => {
                        game = next_game;
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
fn handle_key(game: &FullGameState, key: KeyEvent) -> Option<FullGameState> {
    // If opponent turn or resolving combat, ignore player input
    if game.is_opponent_turn() || game.is_resolving() || game.loop_state == GameStateLoop::GameOver {
        return None;
    }

    // Only handle key press events
    match key.kind {
        crossterm::event::KeyEventKind::Press => {}
        _ => return None,
    }

    let mut new_game = game.clone();

    match key.code {
        KeyCode::Char('q') => {
            println!("Quitting game...");
            new_game.loop_state = GameStateLoop::Quit;
            Some(new_game)
        }
        KeyCode::Left | KeyCode::Char('h') => {
            move_selection_left(&mut new_game);
            Some(new_game)
        }
        KeyCode::Right | KeyCode::Char('l') => {
            move_selection_right(&mut new_game);
            Some(new_game)
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            handle_enter(&mut new_game);
            Some(new_game)
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            confirm_attack(&mut new_game);
            Some(new_game)
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            cancel_attack(&mut new_game);
            Some(new_game)
        }
        KeyCode::Esc => {
            cancel_attack(&mut new_game);
            Some(new_game)
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if new_game.loop_state == GameStateLoop::GameOver {
                new_game.start_new_game();
            }
            Some(new_game)
        }
        _ => None,
    }
}

/// Move selection left
fn move_selection_left(game: &mut FullGameState) {
    if game.loop_state == GameStateLoop::SelectPlayerCard {
        if let Some(idx) = game.selected_player_card {
            if game.selected_player_card.is_some() && game.selected_player_card.unwrap() < game.player_hand.len() {
                game.selected_player_card = Some(if idx == 0 { game.player_hand.len() - 1 } else { idx - 1 });
            }
        }
    } else if game.loop_state == GameStateLoop::SelectOpponentTarget {
        if let Some(idx) = game.selected_opponent_card {
            if game.selected_opponent_card.is_some() && game.selected_opponent_card.unwrap() < game.opponent_hand.len() {
                game.selected_opponent_card = Some(if idx == 0 { game.opponent_hand.len() - 1 } else { idx - 1 });
            }
        }
    }
}

/// Move selection right
fn move_selection_right(game: &mut FullGameState) {
    if game.loop_state == GameStateLoop::SelectPlayerCard {
        game.selected_player_card = Some(game.selected_player_card.unwrap_or(0));
        if let Some(idx) = game.selected_player_card {
            game.selected_player_card = Some((idx + 1) % game.player_hand.len());
        }
    } else if game.loop_state == GameStateLoop::SelectOpponentTarget {
        game.selected_opponent_card = Some(game.selected_opponent_card.unwrap_or(0));
        if let Some(idx) = game.selected_opponent_card {
            game.selected_opponent_card = Some((idx + 1) % game.opponent_hand.len());
        }
    }
}

/// Handle Enter key (advance to next step)
fn handle_enter(game: &mut FullGameState) {
    match game.loop_state {
        GameStateLoop::SelectPlayerCard => {
            if game.selected_player_card.is_some() && game.selected_player_card.unwrap() < game.player_hand.len() {
                game.loop_state = GameStateLoop::SelectOpponentTarget;
                game.selected_opponent_card = Some(0);
            }
        }
        GameStateLoop::SelectOpponentTarget => {
            if game.selected_opponent_card.is_some() && game.selected_opponent_card.unwrap() < game.opponent_hand.len() {
                game.loop_state = GameStateLoop::ConfirmAttack;
            }
        }
        GameStateLoop::ConfirmAttack => {
            confirm_attack(game);
        }
        _ => {}
    }
}

/// Confirm attack and resolve combat
fn confirm_attack(game: &mut FullGameState) {
    if let (Some(player_idx), Some(opponent_idx)) = 
        (game.selected_player_card, game.selected_opponent_card) {
        
        // Ensure indices are valid
        let valid_player_idx = player_idx.min(game.player_hand.len() - 1);
        let valid_opponent_idx = opponent_idx.min(game.opponent_hand.len() - 1);

        let result = game.resolve_player_attack(valid_player_idx, valid_opponent_idx);
        println!("Combat: You dealt {} damage, took {} damage", 
              result.player_dmg, result.opponent_dmg);

        // Reset for next turn
        game.loop_state = GameStateLoop::SelectPlayerCard;
        game.selected_player_card = Some(0);
        game.selected_opponent_card = None;
    }
}

/// Cancel attack and go back
fn cancel_attack(game: &mut FullGameState) {
    match game.loop_state {
        GameStateLoop::SelectOpponentTarget | GameStateLoop::ConfirmAttack => {
            game.loop_state = GameStateLoop::SelectPlayerCard;
            game.selected_opponent_card = None;
        }
        _ => {}
    }
}
