//! Footer rendering - help text and input hints

use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, area: Rect, _state: &super::AppUiState) {
    let help_text = vec![
        Line::from("Controls:← →: Navigate | Space: Play | q: Quit"),
    ];
    
    let paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    
    frame.render_widget(paragraph, area);
}

/// Return the help text as a vector of lines
pub fn get_help_text() -> Vec<Line<'static>> {
    vec![
        Line::from("Controls:← →: Navigate | Space: Play | q: Quit"),
    ]
}
