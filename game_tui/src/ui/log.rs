//! Combat log rendering

use ratatui::{prelude::*, widgets::*};

use game_core::CombatLog;

/// Render combat log
pub fn render(frame: &mut Frame, area: Rect, combat_log: &CombatLog) {
    let max_visible = (area.height as usize - 2).max(1);

    let entries: Vec<&String> = combat_log.iter().collect();
    let start = entries.len().saturating_sub(max_visible);
    let visible: Vec<Line> = entries[start..]
        .iter()
        .map(|entry| Line::from((*entry).as_str()).style(style_log_entry(entry)))
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
        Style::default().fg(Color::Green).bold()
    } else if entry.contains("YOU LOST") {
        Style::default().fg(Color::Red).bold()
    } else if entry.contains("CRITICAL") {
        Style::default().fg(Color::Yellow).bold()
    } else if entry.contains("died") || entry.contains("destroyed") {
        Style::default().fg(Color::Red)
    } else if entry.contains("Game started") {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_log_entry_won() {
        let style = style_log_entry("=== YOU WON! ===");
        assert_eq!(style.foreground, Some(ratatui::style::Color::Green));
    }

    #[test]
    fn test_style_log_entry_lost() {
        let style = style_log_entry("=== YOU LOST ===");
        assert_eq!(style.foreground, Some(ratatui::style::Color::Red));
    }

    #[test]
    fn test_style_log_entry_critical() {
        let style = style_log_entry("CRITICAL! dealt 10 damage");
        assert_eq!(style.foreground, Some(ratatui::style::Color::Yellow));
    }
}
use ratatui::widgets::{Paragraph, Line, Block, Style, Color, Modifier};
