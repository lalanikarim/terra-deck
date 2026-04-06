//! Game over screen with full opponent reveal

use ratatui::{prelude::*, widgets::*};

use crate::game_state::FullGameState;

/// Render game over screen
pub fn render_game_over(frame: &mut Frame, area: Rect, game: &FullGameState) {
    let result_text = match game.game_over_result {
        Some(game_core::GameResult::Won) => "🏆 YOU WON! 🏆",
        Some(game_core::GameResult::Lost) => "💀 YOU LOST 💀",
        Some(game_core::GameResult::Draw) => "🤝 DRAW 🤝",
        None => "GAME OVER",
    };

    let title_style = Style::default()
        .fg(if game.game_over_result == Some(game_core::GameResult::Won) {
            Color::Green
        } else {
            Color::Red
        })
        .add_modifier(Modifier::BOLD);

    let mut lines = vec![Line::from(result_text).style(title_style)];
    lines.push(Line::from(""));

    // Show your final hand
    lines.push(Line::from("Your Final Hand:"));
    for (idx, card) in game.player_hand.cards.iter().enumerate() {
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
        lines.push(Line::from(card_str));
    }

    lines.push(Line::from(""));

    // Reveal opponent's hand
    lines.push(Line::from("Opponent's Hand (REVEALED):"));
    let opponent_lines = create_revealed_opponent_lines(game);
    lines.extend(opponent_lines);

    // Instructions
    lines.push(Line::from(""));
    lines.push(Line::from("Press 'R' to restart or 'Q' to quit"));

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("=== GAME OVER ==="));

    frame.render_widget(paragraph, area);
}

/// Create revealed opponent hand lines
fn create_revealed_opponent_lines(game: &FullGameState) -> Vec<Line<'static>> {
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

/// Get suit character
fn get_suit_char(suit: game_core::Suit) -> char {
    match suit {
        game_core::Suit::Hearts => '♥',
        game_core::Suit::Diamonds => '♦',
        game_core::Suit::Clubs => '♣',
        game_core::Suit::Spades => '♠',
    }
}

/// Get rank string
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
use ratatui::widgets::{Paragraph, Line, Block, Style, Color, Modifier};
