//! TUI rendering module with full game state integration

pub mod footer;
pub mod game_over;
pub mod hand;
pub mod header;
pub mod log;
pub mod opponent;

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use crate::game_state::GameSession;

/// Main render function with full game state
pub fn render_game(
    frame: &mut Frame,
    game: &GameSession,
    _is_player_turn: bool,
    is_opponent_turn: bool,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(7), // Player hand (5 cards + title + spacing)
            Constraint::Min(6),    // Combat log
            Constraint::Length(7), // Opponent hand (5 cards + title + spacing)
            Constraint::Length(2), // Footer
            Constraint::Length(1), // Status
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
fn render_status(frame: &mut Frame, area: Rect, game: &GameSession, is_opponent_turn: bool) {
    let status = match game.loop_state {
        crate::game_state::GameStateLoop::Start => "Game Start".to_string(),
        crate::game_state::GameStateLoop::SelectPlayerCard => {
            "Select your card with ←→, then Enter".to_string()
        }
        crate::game_state::GameStateLoop::SelectOpponentTarget => {
            "Select target with ←→, then Enter to preview".to_string()
        }
        crate::game_state::GameStateLoop::ConfirmAttack => {
            "Press Y to confirm or N to cancel".to_string()
        }
        crate::game_state::GameStateLoop::ResolvingCombat => "Resolving combat...".to_string(),
        crate::game_state::GameStateLoop::WaitingForOpponent => {
            "Waiting for opponent...".to_string()
        }
        crate::game_state::GameStateLoop::OpponentSelectingTarget => {
            "Opponent selecting...".to_string()
        }
        crate::game_state::GameStateLoop::OpponentAttackResolving => {
            "Opponent attacking...".to_string()
        }
        crate::game_state::GameStateLoop::GameOver => match game.game_over_result {
            Some(game_core::GameResult::Won) => {
                "YOU WON! Press R to restart or Q to quit".to_string()
            }
            Some(game_core::GameResult::Lost) => {
                "YOU LOST! Press R to restart or Q to quit".to_string()
            }
            _ => "Game Over".to_string(),
        },
        crate::game_state::GameStateLoop::Quit => "Quitting...".to_string(),
    };

    let style = if is_opponent_turn {
        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White).bg(Color::Blue)
    };

    let mut text_lines: Vec<Line> = Vec::new();
    let words: Vec<&str> = status.split(' ').collect();
    let mut current_line = String::new();

    for word in words {
        if (current_line.len() + word.len() + 1) > area.width as usize - 2 {
            if !current_line.is_empty() {
                text_lines.push(Line::from(current_line));
            }
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        text_lines.push(Line::from(current_line));
    }

    let paragraph = Paragraph::new(text_lines).style(style);

    frame.render_widget(paragraph, area);
}
