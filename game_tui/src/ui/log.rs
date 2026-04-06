//! Combat log rendering - scrollable text area for game events

use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect, state: &super::AppUiState) {
    let log_entries = get_combat_log_entries();
    let max_visible = area.height as usize - 2;

    let start = state.log_scroll_offset;
    let end = std::cmp::min(start + max_visible, log_entries.len());

    let visible_entries: Vec<Line> = log_entries[start..end]
        .iter()
        .map(|entry| Line::from(entry.clone()).style(style_log_entry(entry)))
        .collect();

    let paragraph = if visible_entries.is_empty() {
        Paragraph::new("No combat yet...")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
    } else {
        Paragraph::new(visible_entries)
    };

    frame.render_widget(
        paragraph.block(Block::default().title("Combat Log")),
        area,
    );
}

/// Style log entries based on content
fn style_log_entry(entry: &str) -> Style {
    if entry.contains("Won") || entry.contains("Won!") {
        Style::default().fg(Color::Green).bold()
    } else if entry.contains("Lost") || entry.contains("Lost!") {
        Style::default().fg(Color::Red).bold()
    } else if entry.contains("Critical") || entry.contains("crit") {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Gray)
    }
}

/// Get combat log entries (placeholder)
fn get_combat_log_entries() -> Vec<String> {
    vec![
        "Game started!".to_string(),
        "Player selected card 1".to_string(),
        "Opponent selected card 2".to_string(),
    ]
}
