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
