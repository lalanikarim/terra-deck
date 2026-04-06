//! Header rendering

use ratatui::{prelude::*, widgets::*};

use crate::game_state::FullGameState;

/// Render game header
pub fn render(frame: &mut Frame, area: Rect, game: &FullGameState) {
    let title = build_header_text(game);

    let paragraph = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(paragraph, area);
}

/// Build header text with game info
fn build_header_text(game: &FullGameState) -> Line<'static> {
    let round = if game.current_combat_round > 0 {
        format!(" - Round {}", game.current_combat_round)
    } else {
        "".to_string()
    };

    Line::from(format!(
        "🎴 TERRA-DECK - Poker Card RPG{}",
        round
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_header_text() {
        let game = FullGameState::new();
        let line = build_header_text(&game);
        assert!(line.to_string().contains("TERRA-DECK"));
    }
}
use ratatui::widgets::{Paragraph, Line, Block, Style, Color, Modifier};
