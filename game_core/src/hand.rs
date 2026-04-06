//! Hand implementation

use crate::Card;
use bevy::prelude::*;

/// Resource representing a player's hand
#[derive(Resource, Default)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    /// Adds a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Removes and returns a card at the specified index
    pub fn play_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    /// Returns the number of cards in the hand
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns the number of cards in the hand (alias for len)
    pub fn card_count(&self) -> usize {
        self.len()
    }

    /// Returns true if the hand is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Gets a reference to a card at the specified index
    pub fn get(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }
}
