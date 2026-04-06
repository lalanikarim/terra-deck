//! Poker Card RPG - TUI Application
//!
//! Main entry point for the terminal user interface using ratatui and crossterm.

use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Write};

use ui::AppUiState;

fn main() {
    // Setup terminal
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    stdout().execute(EnterAlternateScreen).expect("Failed to enter alternate screen");

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    // Initialize app state
    let mut app_state = AppUiState::default();
    app_state.move_selection_right(); // Select first card

    // Main loop
    loop {
        // Draw frame
        terminal
            .draw(|frame| ui::render_game(frame, &app_state))
            .expect("Failed to draw frame");

        // Poll for events
        if event::poll(std::time::Duration::from_millis(100)).expect("Failed to poll events") {
            if let event::Event::Key(key) = event::read().expect("Failed to read event") {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            return; // Quit
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            app_state.move_selection_left();
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            app_state.move_selection_right();
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            play_selected_card(&app_state);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Cleanup
    drop(terminal);
    stdout()
        .execute(LeaveAlternateScreen)
        .expect("Failed to leave alternate screen");
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

/// Handle playing the selected card
fn play_selected_card(state: &AppUiState) {
    if let Some(idx) = state.selected_card {
        eprintln!("Playing card {}", idx);
    }
}

mod ui;
