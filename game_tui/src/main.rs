//! Poker Card RPG - TUI Application

use bevy::prelude::*;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use ratatui::{backend::CrosstermBackend, Terminal};

use game_core::*;

mod ui;
use ui::{AppUiState, render_game};

fn main() {
    // Setup terminal
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    stdout()
        .execute(EnterAlternateScreen)
        .expect("Failed to enter alternate screen");

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    // Initialize game state with placeholder cards
    let mut tui_state = AppUiState::default();
    tui_state.selected_card = Some(0);

    // Example cards for display
    let player_cards = create_example_player_hand();
    let opponent_card_count = create_example_opponent_hand().len();

    info!("🎴 Poker Card RPG - Terra-Deck");
    info!("Player has {} cards, Opponent has {} cards", 
          player_cards.len(), opponent_card_count);

    // Main game loop
    loop {
        // Draw frame with current state
        terminal
            .draw(|frame| render_game(frame, &tui_state, &player_cards))
            .expect("Failed to draw frame");

        // Poll for input events
        if event::poll(std::time::Duration::from_millis(100))
            .expect("Failed to poll events")
        {
            if let event::Event::Key(key) = event::read().expect("Failed to read event") {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            info!("Quitting game...");
                            break;
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            tui_state.move_selection_left();
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            tui_state.move_selection_right();
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            if let Some(idx) = tui_state.selected_card {
                                info!("→ Playing card #{} ({} of {})", 
                                     idx + 1, 
                                     get_card_display_name(&player_cards[idx]),
                                     player_cards[idx].suit);
                                // TODO: Trigger combat system
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Cleanup terminal
    drop(terminal);
    stdout()
        .execute(LeaveAlternateScreen)
        .expect("Failed to leave alternate screen");
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

/// Create example player hand for demo
fn create_example_player_hand() -> Vec<Card> {
    vec![
        Card::new(game_core::Suit::Hearts, game_core::Rank::Ten),
        Card::new(game_core::Suit::Diamonds, game_core::Rank::Five),
        Card::new(game_core::Suit::Clubs, game_core::Rank::Jack),
        Card::new(game_core::Suit::Spades, game_core::Rank::Queen),
        Card::new(game_core::Suit::Hearts, game_core::Rank::Ace),
    ]
}

/// Create example opponent hand (hidden)
fn create_example_opponent_hand() -> Vec<Card> {
    vec![
        Card::new(game_core::Suit::Diamonds, game_core::Rank::King),
        Card::new(game_core::Suit::Clubs, game_core::Rank::Eight),
        Card::new(game_core::Suit::Spades, game_core::Rank::Nine),
        Card::new(game_core::Suit::Hearts, game_core::Rank::Four),
        Card::new(game_core::Suit::Spades, game_core::Rank::Ace),
    ]
}

/// Get a friendly name for a card
fn get_card_display_name(card: &Card) -> String {
    format!("{} {}", card.rank, card.suit)
}
