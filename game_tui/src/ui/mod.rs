//! TUI rendering module using ratatui
//!
//! This module provides terminal rendering for the Poker Card RPG.

pub mod header;
pub mod hand;
pub mod log;
pub mod footer;

use ratatui::{prelude::*, layout::*};
use game_core::{GameState, GameResult, CombatStats, CombatLog, Hand, SelectedCard};

/// Application state for the TUI
#[derive(Default)]
pub struct AppUiState {
    pub selected_card: Option<usize>,
    pub log_scroll_offset: usize,
}

impl AppUiState {
    /// Creates a new AppUiState
    pub fn new() -> Self {
        AppUiState {
            selected_card: None,
            log_scroll_offset: 0,
        }
    }

    /// Updates app state from Bevy resources
    pub fn update_from_resources(
        &mut self,
        selected_card: &SelectedCard,
        combat_log: &CombatLog,
        _player_hand: &Hand,
    ) {
        self.selected_card = selected_card.index;
        
        // Auto-scroll log if new entries added
        let max_visible = 10;
        let log_entries = combat_log.iter().collect::<Vec<_>>();
        let log_len = log_entries.len();
        if log_len > max_visible && self.log_scroll_offset + max_visible < log_len {
            self.log_scroll_offset = log_len - max_visible;
        }
    }
    
    /// Move selection left
    pub fn move_selection_left(&mut self) {
        if let Some(idx) = self.selected_card {
            self.selected_card = if idx == 0 { None } else { Some(idx - 1) };
        }
    }
    
    /// Move selection right
    pub fn move_selection_right(&mut self) {
        match self.selected_card {
            None => self.selected_card = Some(0),
            Some(idx) => self.selected_card = Some(idx + 1),
        }
    }
}

/// Main render function called from main.rs
pub fn render_game(frame: &mut Frame, state: &AppUiState) {
    render(frame, state);
}

/// Renders the full UI frame
fn render(frame: &mut Frame, state: &AppUiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(4),  // Player hand
            Constraint::Min(10),    // Combat log
            Constraint::Length(3),  // Opponent hand
            Constraint::Length(2),  // Footer
        ])
        .split(frame.area());

    header::render(frame, chunks[0], state);
    hand::render_player_hand(frame, chunks[1], state);
    log::render(frame, chunks[2], state);
    hand::render_opponent_hand(frame, chunks[3], state);
    footer::render(frame, chunks[4], state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_state_default() {
        let state = AppUiState::default();
        assert_eq!(state.selected_card, None);
        assert_eq!(state.log_scroll_offset, 0);
    }
    
    #[test]
    fn test_ui_state_new() {
        let state = AppUiState::new();
        assert_eq!(state.selected_card, None);
        assert_eq!(state.log_scroll_offset, 0);
    }
    
    #[test]
    fn test_move_selection_left_from_none() {
        let mut state = AppUiState::default();
        state.move_selection_left();
        assert_eq!(state.selected_card, None);
    }
    
    #[test]
    fn test_move_selection_left_from_zero() {
        let mut state = AppUiState {
            selected_card: Some(0),
            ..Default::default()
        };
        state.move_selection_left();
        assert_eq!(state.selected_card, None);
    }
    
    #[test]
    fn test_move_selection_left() {
        let mut state = AppUiState {
            selected_card: Some(2),
            ..Default::default()
        };
        state.move_selection_left();
        assert_eq!(state.selected_card, Some(1));
    }
    
    #[test]
    fn test_move_selection_right() {
        let mut state = AppUiState::default();
        state.move_selection_right();
        assert_eq!(state.selected_card, Some(0));
    }
}
