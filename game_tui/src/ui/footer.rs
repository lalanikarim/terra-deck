//! Footer rendering with help text

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};

use crate::game_state::GameSession;

/// Render footer with current controls
pub fn render(frame: &mut Frame, area: Rect, game: &GameSession) {
    let help_text = get_help_text_for_state(game);

    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default());

    frame.render_widget(paragraph, area);
}

/// Get help text based on game state
fn get_help_text_for_state(game: &GameSession) -> Line<'static> {
    match game.loop_state {
        crate::game_state::GameStateLoop::SelectPlayerCard => {
            Line::from("←→ Move selection | Enter Select card | q Quit")
        }
        crate::game_state::GameStateLoop::SelectOpponentTarget => {
            Line::from("←→ Choose target | Enter Preview | Esc Cancel")
        }
        crate::game_state::GameStateLoop::ConfirmAttack => {
            Line::from("Y Confirm attack | N Cancel")
        }
        crate::game_state::GameStateLoop::ResolvingCombat |
        crate::game_state::GameStateLoop::WaitingForOpponent => {
            Line::from("Waiting for combat resolution...")
        }
        crate::game_state::GameStateLoop::GameOver => {
            Line::from("R Restart game | Q Quit")
        }
        _ => {
            Line::from("←→ Navigate | Enter Confirm | q Quit")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::GameStateLoop;

    #[test]
    fn test_get_help_text_select_player() {
        let mut game = GameSession::new();
        game.loop_state = GameStateLoop::SelectPlayerCard;
        let text = get_help_text_for_state(&game);
        assert!(text.to_string().contains("←→"));
    }

    #[test]
    fn test_get_help_text_confirm() {
        let mut game = GameSession::new();
        game.loop_state = GameStateLoop::ConfirmAttack;
        let text = get_help_text_for_state(&game);
        assert!(text.to_string().contains("Y"));
    }

    #[test]
    fn test_get_help_text_game_over() {
        let mut game = GameSession::new();
        game.loop_state = GameStateLoop::GameOver;
        let text = get_help_text_for_state(&game);
        assert!(text.to_string().contains("R"));
    }
}
