//! Opponent hand rendering - shows hidden cards with alive/dead indicators

use ratatui::{prelude::*, widgets::*};

use crate::game_state::FullGameState;

/// Render opponent's hand (all cards hidden with alive indicators)
pub fn render_opponent_hand(frame: &mut Frame, area: Rect, game: &FullGameState) {
    if game.opponent_hand.is_empty() {
        let paragraph = Paragraph::new("Opponent has no cards")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(Block::default().title("Opponent's Hand"));
        frame.render_widget(paragraph, area);
        return;
    }

    let lines = create_opponent_hand_lines(game);
    let paragraph = Paragraph::new(lines).block(Block::default().title("Opponent's Hand"));
    frame.render_widget(paragraph, area);
}

/// Create opponent hand lines (all cards hidden)
fn create_opponent_hand_lines(game: &FullGameState) -> Vec<Line<'static>> {
    (0..game.opponent_hand.len())
        .map(|idx| {
            let is_selected = game.selected_opponent_card == Some(idx);
            create_opponent_card_line(idx + 1, is_selected)
        })
        .collect()
}

/// Create a single hidden opponent card line
fn create_opponent_card_line(display_idx: usize, is_selected: bool) -> Line<'static> {
    let selection_marker = if is_selected { "←" } else { "" };
    
    let line_str = format!(
        "[?]   ● (alive) {}",
        selection_marker
    );

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    Line::from(line_str).style(style)
}

/// Create revealed opponent hand lines (shown at game over)
pub fn create_revealed_opponent_lines(game: &FullGameState) -> Vec<Line<'static>> {
    game.opponent_hand.cards
        .iter()
        .enumerate()
        .map(|(idx, card)| {
            let status = if card.hp == 0 { "(dead)" } else { "(alive)" };
            let card_str = format!(
                "  {}. {} {} HP:{}/{} {}",
                idx + 1,
                get_suit_char(card.suit),
                get_rank_str(card.rank),
                card.hp,
                card.max_hp,
                status
            );
            Line::from(card_str).style(Style::default().fg(Color::Cyan))
        })
        .collect()
}

/// Get suit character (same as player hand)
fn get_suit_char(suit: game_core::Suit) -> char {
    match suit {
        game_core::Suit::Hearts => '♥',
        game_core::Suit::Diamonds => '♦',
        game_core::Suit::Clubs => '♣',
        game_core::Suit::Spades => '♠',
    }
}

/// Get rank string (same as player hand)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opponent_card_line_hidden() {
        let line = create_opponent_card_line(1, false);
        assert!(line.to_string().contains("[?]"));
        assert!(line.to_string().contains("alive"));
    }

    #[test]
    fn test_opponent_card_line_selected() {
        let line = create_opponent_card_line(1, true);
        assert!(line.to_string().contains("←"));
    }
}
use ratatui::widgets::{Paragraph, Line, Block, Style, Color, Modifier};
