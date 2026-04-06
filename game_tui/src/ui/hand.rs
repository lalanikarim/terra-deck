//! Hand rendering - shows player and opponent cards

use ratatui::{prelude::*, style::Modifier, widgets::*};

/// Render the player's hand with selectable cards
pub fn render_player_hand(frame: &mut Frame, area: Rect, state: &super::AppUiState, cards: &[game_core::Card]) {
    if cards.is_empty() {
        let placeholder = Paragraph::new("No cards in hand")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(Block::default().title("Your Hand"));
        frame.render_widget(placeholder, area);
        return;
    }

    let mut lines = Vec::new();

    for (idx, card) in cards.iter().enumerate() {
        let is_selected = state.selected_card == Some(idx);
        let display_str = format!(
            "({}) {} {} {} HP:{}/{}",
            idx + 1,
            get_suit_str(card.suit),
            get_suit_char(card.suit),
            card.rank,
            card.hp,
            card.max_hp
        );

        let final_str = if is_selected {
            format!("{} ←", display_str)
        } else {
            display_str
        };

        let base_style = if is_selected {
            Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(final_str).style(base_style));
    }

    let paragraph = Paragraph::new(lines).block(Block::default().title("Your Hand"));
    frame.render_widget(paragraph, area);
}

fn get_suit_str(suit: game_core::Suit) -> &'static str {
    match suit {
        game_core::Suit::Hearts => "Red",
        game_core::Suit::Diamonds => "Ylw",
        game_core::Suit::Clubs => "GrY",
        game_core::Suit::Spades => "GrY",
    }
}

fn get_suit_char(suit: game_core::Suit) -> char {
    match suit {
        game_core::Suit::Hearts => '♥',
        game_core::Suit::Diamonds => '♦',
        game_core::Suit::Clubs => '♣',
        game_core::Suit::Spades => '♠',
    }
}

pub fn render_opponent_hand(frame: &mut Frame, area: Rect, _state: &super::AppUiState, card_count: usize) {
    let display = if card_count == 0 {
        "No cards"
    } else {
        &format!("[?] × {} (hidden)", card_count)
    };

    let paragraph = Paragraph::new(display)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().title("Opponent's Hand"));

    frame.render_widget(paragraph, area);
}
