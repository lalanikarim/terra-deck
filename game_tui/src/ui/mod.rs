//! TUI rendering module

pub mod header;
pub mod hand;
pub mod log;
pub mod footer;

use ratatui::prelude::*;
use ratatui::layout::*;
use game_core::Card;

/// Application state for TUI
#[derive(Default)]
pub struct AppUiState {
    pub selected_card: Option<usize>,
    pub log_scroll_offset: Option<usize>,
}

impl AppUiState {
    pub fn move_selection_left(&mut self) {
        if let Some(idx) = self.selected_card {
            self.selected_card = if idx == 0 {
                None
            } else {
                Some(idx - 1)
            };
        }
    }

    pub fn move_selection_right(&mut self) {
        match self.selected_card {
            None => self.selected_card = Some(0),
            Some(idx) => self.selected_card = Some(idx + 1),
        }
    }
}

/// Main render function
pub fn render_game(frame: &mut Frame, state: &AppUiState, player_cards: &[Card]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(frame.area());

    header::render(frame, chunks[0], state);
    hand::render_player_hand(frame, chunks[1], state, player_cards);
    log::render(frame, chunks[2], state, &get_log_entries());
    hand::render_opponent_hand(frame, chunks[3], state, 5);
    footer::render(frame, chunks[4], state);
}

/// Get log entries
fn get_log_entries() -> Vec<String> {
    vec![
        "Game started!".to_string(),
        "Player selected card 1".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_state_default() {
        let state = AppUiState::default();
        assert_eq!(state.selected_card, None);
    }

    #[test]
    fn test_move_selection_left_from_none() {
        let mut state = AppUiState::default();
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
    fn test_move_selection_right_from_none() {
        let mut state = AppUiState::default();
        state.move_selection_right();
        assert_eq!(state.selected_card, Some(0));
    }

    #[test]
    fn test_move_selection_right() {
        let mut state = AppUiState {
            selected_card: Some(2),
            ..Default::default()
        };
        state.move_selection_right();
        assert_eq!(state.selected_card, Some(3));
    }
}
