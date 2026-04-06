//! TUI rendering module with full game state integration

pub mod header;
pub mod hand;
pub mod log;
pub mod footer;
pub mod opponent;
pub mod game_over;

use ratatui::prelude::*;
use ratatui::layout::*;

use crate::game_state::FullGameState;

/// Main render function with full game state
pub fn render_game(
    frame: &mut Frame,
    game: &FullGameState,
    is_player_turn: bool,
    is_opponent_turn: bool,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),       // Header
            Constraint::Length(4),       // Player hand
            Constraint::Min(8),          // Combat log
            Constraint::Length(4),       // Opponent hand
            Constraint::Length(2),       // Footer
            Constraint::Length(1),       // Status
        ])
        .split(frame.area());

    header::render(frame, chunks[0], game);
    hand::render_player_hand(frame, chunks[1], game);
    log::render(frame, chunks[2], &game.combat_log);
    opponent::render_opponent_hand(frame, chunks[3], game);
    footer::render(frame, chunks[4], game);
    render_status(frame, chunks[5], game, is_opponent_turn);
}

/// Render status bar showing game state
fn render_status(
    frame: &mut Frame,
    area: Rect,
    game: &FullGameState,
    is_opponent_turn: bool,
) {
    let status = match game.loop_state {
        crate::game_state::GameStateLoop::Start => "Game Start".to_string(),
        crate::game_state::GameStateLoop::SelectPlayerCard => "Select your card with ←→, then Enter".to_string(),
        crate::game_state::GameStateLoop::SelectOpponentTarget => "Select target with ←→, then Enter to preview".to_string(),
        crate::game_state::GameStateLoop::ConfirmAttack => "Press Y to confirm or N to cancel".to_string(),
        crate::game_state::GameStateLoop::ResolvingCombat => "Resolving combat...".to_string(),
        crate::game_state::GameStateLoop::WaitingForOpponent => "Waiting for opponent...".to_string(),
        crate::game_state::GameStateLoop::OpponentSelectingTarget => "Opponent selecting...".to_string(),
        crate::game_state::GameStateLoop::OpponentAttackResolving => "Opponent attacking...".to_string(),
        crate::game_state::GameStateLoop::GameOver => match game.game_over_result {
            Some(game_core::GameResult::Won) => "YOU WON! Press R to restart or Q to quit".to_string(),
            Some(game_core::GameResult::Lost) => "YOU LOST! Press R to restart or Q to quit".to_string(),
            _ => "Game Over".to_string(),
        }
        crate::game_state::GameStateLoop::Quit => "Quitting...".to_string(),
    };

    let style = if is_opponent_turn {
        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White).bg(Color::Blue)
    };

    let paragraph = Paragraph::new(status)
        .style(style)
        .wrap(ratatui::layout::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_initialization() {
        let game = FullGameState::new();
        assert!(game.player_hand.is_empty());
        assert!(game.opponent_hand.is_empty());
    }

    #[test]
    fn test_move_selection_left_from_none() {
        let mut game = FullGameState::new();
        game.loop_state = crate::game_state::GameStateLoop::SelectPlayerCard;
        game.selected_player_card = None;
        super::move_selection_left(&mut game);
        assert_eq!(game.selected_player_card, None);
    }

    #[test]
    fn test_move_selection_left() {
        let mut game = FullGameState::new();
        game.loop_state = crate::game_state::GameStateLoop::SelectPlayerCard;
        game.selected_player_card = Some(2);
        super::move_selection_left(&mut game);
        assert_eq!(game.selected_player_card, Some(1));
    }

    #[test]
    fn test_move_selection_right_from_none() {
        let mut game = FullGameState::new();
        game.loop_state = crate::game_state::GameStateLoop::SelectPlayerCard;
        game.selected_player_card = None;
        super::move_selection_right(&mut game);
        assert_eq!(game.selected_player_card, Some(1));
    }

    #[test]
    fn test_move_selection_right() {
        let mut game = FullGameState::new();
        game.loop_state = crate::game_state::GameStateLoop::SelectPlayerCard;
        game.selected_player_card = Some(2);
        super::move_selection_right(&mut game);
        assert_eq!(game.selected_player_card, Some(3));
    }
}
use ratatui::widgets::{Paragraph, Line, Block, Style, Color, Modifier};
