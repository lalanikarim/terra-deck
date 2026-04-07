//! Header rendering

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};

use crate::game_state::FullGameState;

/// Render game header
pub fn render(frame: &mut Frame, area: Rect, game: &FullGameState) {
    let title = build_header_text(game);

    let paragraph = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
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
    Line::from(format!("🎴 TERRA-DECK - Poker Card RPG{}", round))
}
