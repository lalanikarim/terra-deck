//! Combat log rendering

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};

use game_core::CombatLog;

/// Render combat log
pub fn render(frame: &mut Frame, area: Rect, combat_log: &CombatLog) {
    let max_visible = (area.height as usize - 2).max(1);

    let entries: Vec<&String> = combat_log.iter().collect();
    let start = entries.len().saturating_sub(max_visible);
    let visible: Vec<Line> = entries[start..]
        .iter()
        .map(|entry| Line::from(entry.as_str()).style(style_log_entry(entry)))
        .collect();

    let paragraph = if visible.is_empty() {
        Paragraph::new("Combat log is empty...")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
    } else {
        Paragraph::new(visible)
    };

    frame.render_widget(
        paragraph.block(Block::default().title("Combat Log")),
        area,
    );
}

/// Style log entries based on content
fn style_log_entry(entry: &str) -> Style {
    if entry.contains("YOU WON") {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else if entry.contains("YOU LOST") {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else if entry.contains("CRITICAL") {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else if entry.contains("died") || entry.contains("destroyed") {
        Style::default().fg(Color::Red)
    } else if entry.contains("Game started") {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    }
}