//! Combat log rendering

use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect, state: &super::AppUiState, log_entries: &[String]) {
    let max_visible = (area.height as usize - 2).max(1);

    let start = state.log_scroll_offset.unwrap_or(0);
    let _end = std::cmp::min(start + max_visible, log_entries.len());

    let visible_entries: Vec<Line> = log_entries
        .iter()
        .enumerate()
        .filter_map(|(i, entry)| {
            let relative_idx = i - start;
            if relative_idx < max_visible {
                Some(Line::from(entry.clone()).style(style_log_entry(entry)))
            } else {
                None
            }
        })
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
