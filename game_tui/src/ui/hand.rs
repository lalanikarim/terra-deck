//! Hand rendering - shows player and opponent cards

use ratatui::{prelude::*, style::Modifier, widgets::*};
use game_core::{Card, Suit, Rank};

/// Render the player's hand with selectable cards
pub fn render_player_hand(frame: &mut Frame, area: Rect, state: &super::AppUiState) {
    let cards = get_player_cards();
    
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
        let card_str = format_card_display(card);
        let display_str = format!("({}) {}", idx + 1, card_str);
        
        let base_style = if is_selected {
            Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        
        lines.push(Line::from(display_str).style(base_style));
    }
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("Your Hand"));
    
    frame.render_widget(paragraph, area);
}

/// Render opponent's hand (hidden cards)
pub fn render_opponent_hand(frame: &mut Frame, area: Rect, _state: &super::AppUiState) {
    let opponent_card_count = get_opponent_card_count();
    
    let display = if opponent_card_count == 0 {
        "No cards"
    } else {
        &format!("[?] × {} (hidden)", opponent_card_count)
    };
    
    let paragraph = Paragraph::new(display)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().title("Opponent's Hand"));
    
    frame.render_widget(paragraph, area);
}

/// Format a card for display
/// Shows: Suit abbreviation + Symbol + Rank + (HP/MaxHP)
fn format_card_display(card: &Card) -> String {
    let (suit_str, suit_char) = match card.suit {
        Suit::Hearts => ("Red", '♥'),
        Suit::Diamonds => ("Ylw", '♦'),
        Suit::Clubs => ("GrY", '♣'),
        Suit::Spades => ("GrY", '♠'),
    };
    
    format!(
        "{} {} {} HP:{}/{}",
        suit_str, suit_char, card.rank, card.hp, card.max_hp
    )
}

/// Get player cards (placeholder)
fn get_player_cards() -> Vec<Card> {
    vec![
        Card::new(Suit::Hearts, Rank::Ten),
        Card::new(Suit::Diamonds, Rank::Five),
        Card::new(Suit::Clubs, Rank::Jack),
        Card::new(Suit::Spades, Rank::Queen),
    ]
}

/// Get opponent card count (placeholder)
fn get_opponent_card_count() -> usize {
    4
}
