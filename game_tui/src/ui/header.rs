//! Header rendering - shows game state/turn indicator

use ratatui::{prelude::*, widgets::*};
use game_core::{GameState, GameResult};

pub fn render(frame: &mut Frame, area: Rect, state: &super::AppUiState) {
    let title = match state.selected_card {
        Some(idx) => format!("Poker Card RPG - Card {} Selected", idx + 1),
        None => "Poker Card RPG - Terra-Deck".to_string(),
    };

    let paragraph = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center)
        .block(Block::default().title("Poker Card RPG"));

    frame.render_widget(paragraph, area);
}

/// Get the header text based on game state
pub fn get_state_text(game_state: &GameState) -> (String, Style) {
    match game_state {
        GameState::PlayerTurn => ("YOUR TURN - Select a card!".to_string(), Style::default().fg(Color::Green).bold()),
        GameState::OpponentTurn => ("OPPONENT TURN - Waiting...".to_string(), Style::default().fg(Color::Yellow).bold()),
        GameState::CombatResolution => ("⚔️ COMBAT! ⚔️".to_string(), Style::default().fg(Color::Red).bold()),
        GameState::GameOver(result) => {
            let (text, style) = match result {
                GameResult::Won => ("🎉 YOU WON! 🎉".to_string(), Style::default().fg(Color::Green).bold()),
                GameResult::Lost => ("💀 YOU LOST 💀".to_string(), Style::default().fg(Color::Red).bold()),
                GameResult::Draw => ("🤝 DRAW 🤝".to_string(), Style::default().fg(Color::Yellow).bold()),
            };
            (text, style)
        }
    }
}
