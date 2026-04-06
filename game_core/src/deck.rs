//! Deck implementation

use crate::types::{Rank, Suit};
use crate::Card;
use bevy::prelude::*;

/// Resource representing the game deck
#[derive(Resource)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Create a new deck with all 52 cards in standard order
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        // Add one of each suit/rank combination
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades].iter() {
            for rank in [
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ]
            .iter()
            {
                cards.push(Card::new(*suit, *rank));
            }
        }

        Deck { cards }
    }

    /// Draws a card from the deck, returning None if empty
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Returns the number of cards remaining in the deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns true if the deck is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_generation() {
        let deck = Deck::new();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_deck_contains_all_suits() {
        let deck = Deck::new();
        let mut suits_count = std::collections::HashMap::new();

        for card in &deck.cards {
            *suits_count.entry(card.suit).or_insert(0) += 1;
        }

        assert_eq!(suits_count.len(), 4);
        for (_, count) in suits_count {
            assert_eq!(count, 13);
        }
    }

    #[test]
    fn test_deck_contains_all_ranks() {
        let deck = Deck::new();
        let mut ranks_count = std::collections::HashMap::new();

        for card in &deck.cards {
            *ranks_count.entry(card.rank).or_insert(0) += 1;
        }

        for rank in [
            Rank::Two,
            Rank::Five,
            Rank::Ten,
            Rank::Jack,
            Rank::King,
            Rank::Ace,
        ]
        .iter()
        {
            assert_eq!(ranks_count.get(rank), Some(&4));
        }
    }

    #[test]
    fn test_deck_draw() {
        let mut deck = Deck::new();
        let card = deck.draw();
        assert!(card.is_some());
        assert_eq!(deck.len(), 51);
    }

    #[test]
    fn test_deck_draw_multiple() {
        let mut deck = Deck::new();

        for i in 0..10 {
            let card = deck.draw();
            assert!(card.is_some());
            assert_eq!(deck.len(), 51 - i);
        }
    }

    #[test]
    fn test_deck_draw_empty() {
        let mut deck = Deck::new();

        // Drain the deck
        while deck.len() > 0 {
            deck.draw();
        }

        let card = deck.draw();
        assert!(card.is_none());
    }

    #[test]
    fn test_deck_is_empty_false() {
        let deck = Deck::new();
        assert!(!deck.is_empty());
    }

    #[test]
    fn test_deck_is_empty_true() {
        let mut deck = Deck::new();
        while !deck.is_empty() {
            deck.draw();
        }
        assert!(deck.is_empty());
    }

    #[test]
    fn test_deck_draw_then_empty() {
        let mut deck = Deck::new();
        let initial_len = deck.len();

        for _ in 0..initial_len {
            deck.draw();
        }

        assert!(deck.is_empty());
        assert_eq!(deck.len(), 0);
    }

    #[test]
    fn test_deck_draw_until_empty() {
        let mut deck = Deck::new();
        let mut drawn_cards = Vec::new();

        while let Some(card) = deck.draw() {
            drawn_cards.push(card);
        }

        assert_eq!(drawn_cards.len(), 52);
        assert!(deck.is_empty());
    }

    #[test]
    fn test_multiple_decks_independent() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();

        let card1 = deck1.draw();
        let card2 = deck2.draw();

        assert!(card1.is_some());
        assert!(card2.is_some());

        // Second deck should still be intact
        assert_eq!(deck2.len(), 51);
    }
}
