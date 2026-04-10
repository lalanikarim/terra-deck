//! Hand rendering - shows player cards with selection

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};

use crate::game_state::GameSession;

/// Render the player's hand with selectable cards
pub fn render_player_hand(frame: &mut Frame, area: Rect, game: &GameSession) {
    if game.player_hand.is_empty() {
        let placeholder = Paragraph::new("No cards in hand - Game Over")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(Block::default().title("Your Hand"));
        frame.render_widget(placeholder, area);
        return;
    }

    let mut lines = Vec::new();

    for (idx, card) in game.player_hand.cards.iter().enumerate() {
        let is_selected = game.selected_player_card.index == Some(idx);
        let card_line = create_card_line(card, idx + 1, is_selected);
        lines.push(card_line);
    }

    let paragraph = Paragraph::new(lines).block(Block::default().title("Your Hand"));
    frame.render_widget(paragraph, area);
}

/// Create a single card line for display
fn create_card_line(
    card: &game_core::Card,
    display_idx: usize,
    is_selected: bool,
) -> Line<'static> {
    let selection_marker = if is_selected { "← " } else { "  " };
    let card_str = format!(
        "{} {} HP:{}/{}",
        get_suit_char(card.suit),
        get_rank_str(card.rank),
        card.hp,
        card.max_hp
    );

    let line_str = format!("({}) {} {}", display_idx, card_str, selection_marker);

    let style = if is_selected {
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    Line::from(line_str).style(style)
}

/// Get suit character for display
fn get_suit_char(suit: game_core::Suit) -> char {
    match suit {
        game_core::Suit::Hearts => '♥',
        game_core::Suit::Diamonds => '♦',
        game_core::Suit::Clubs => '♣',
        game_core::Suit::Spades => '♠',
    }
}

/// Get rank string for display
fn get_rank_str(rank: game_core::Rank) -> String {
    match rank {
        game_core::Rank::Two => "2".to_string(),
        game_core::Rank::Three => "3".to_string(),
        game_core::Rank::Four => "4".to_string(),
        game_core::Rank::Five => "5".to_string(),
        game_core::Rank::Six => "6".to_string(),
        game_core::Rank::Seven => "7".to_string(),
        game_core::Rank::Eight => "8".to_string(),
        game_core::Rank::Nine => "9".to_string(),
        game_core::Rank::Ten => "10".to_string(),
        game_core::Rank::Jack => "J".to_string(),
        game_core::Rank::Queen => "Q".to_string(),
        game_core::Rank::King => "K".to_string(),
        game_core::Rank::Ace => "A".to_string(),
    }
}
