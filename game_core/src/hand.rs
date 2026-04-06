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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Rank, Suit};

    #[test]
    fn test_hand_empty_initially() {
        let hand = Hand::default();
        assert!(hand.is_empty());
        assert_eq!(hand.len(), 0);
    }

    #[test]
    fn test_hand_add_card() {
        let mut hand = Hand::default();
        let card = Card::new(Suit::Hearts, Rank::Seven);
        hand.add_card(card);
        assert_eq!(hand.len(), 1);
    }

    #[test]
    fn test_hand_add_multiple_cards() {
        let mut hand = Hand::default();

        for rank in [Rank::Two, Rank::Five, Rank::Ten, Rank::King, Rank::Ace].iter() {
            hand.add_card(Card::new(Suit::Clubs, *rank));
            assert_eq!(hand.len(), hand.card_count());
        }
    }

    #[test]
    fn test_hand_play_card() {
        let mut hand = Hand::default();
        hand.add_card(Card::new(Suit::Diamonds, Rank::Queen));
        hand.add_card(Card::new(Suit::Spades, Rank::Jack));
        hand.add_card(Card::new(Suit::Hearts, Rank::Nine));

        let card = hand.play_card(1);
        assert!(card.is_some());
        assert_eq!(card.unwrap().rank, Rank::Jack);
        assert_eq!(hand.len(), 2);
    }

    #[test]
    fn test_hand_play_invalid_index() {
        let mut hand = Hand::default();

        hand.add_card(Card::new(Suit::Clubs, Rank::Four));
        hand.add_card(Card::new(Suit::Hearts, Rank::Eight));

        let card = hand.play_card(10);
        assert!(card.is_none());
        assert_eq!(hand.len(), 2);
    }

    #[test]
    fn test_hand_play_invalid_index_empty_hand() {
        let mut hand = Hand::default();
        let card = hand.play_card(0);
        assert!(card.is_none());
    }

    #[test]
    fn test_hand_play_all_cards() {
        let mut hand = Hand::default();

        for i in 0..5 {
            hand.add_card(Card::new(
                [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades][i % 4],
                [Rank::Two, Rank::Five, Rank::Ten, Rank::King, Rank::Ace][i],
            ));
        }

        assert_eq!(hand.len(), 5);

        for i in 0..5 {
            let card = hand.play_card(0);
            assert!(card.is_some());
            assert_eq!(hand.len(), 5 - i - 1);
        }

        assert!(hand.is_empty());
    }

    #[test]
    fn test_hand_get_card() {
        let mut hand = Hand::default();
        hand.add_card(Card::new(Suit::Diamonds, Rank::King));
        hand.add_card(Card::new(Suit::Spades, Rank::Ace));
        hand.add_card(Card::new(Suit::Clubs, Rank::Seven));

        let card = hand.get(1);
        assert!(card.is_some());
        assert_eq!(card.unwrap().rank, Rank::Ace);
        assert_eq!(card.unwrap().suit, Suit::Spades);
    }

    #[test]
    fn test_hand_get_invalid_index() {
        let mut hand = Hand::default();
        hand.add_card(Card::new(Suit::Hearts, Rank::Six));

        let card = hand.get(5);
        assert!(card.is_none());
    }

    #[test]
    fn test_hand_get_empty_hand() {
        let hand = Hand::default();
        let card = hand.get(0);
        assert!(card.is_none());
    }

    #[test]
    fn test_hand_card_count() {
        let mut hand = Hand::default();
        assert_eq!(hand.card_count(), 0);

        hand.add_card(Card::new(Suit::Diamonds, Rank::Four));
        assert_eq!(hand.card_count(), 1);

        hand.add_card(Card::new(Suit::Spades, Rank::Nine));
        assert_eq!(hand.card_count(), 2);

        hand.play_card(0);
        assert_eq!(hand.card_count(), 1);
    }

    #[test]
    fn test_hand_play_first_last() {
        let mut hand = Hand::default();
        hand.add_card(Card::new(Suit::Hearts, Rank::Two));
        hand.add_card(Card::new(Suit::Diamonds, Rank::Three));
        hand.add_card(Card::new(Suit::Clubs, Rank::Four));

        let first = hand.play_card(0);
        assert_eq!(first.unwrap().rank, Rank::Two);

        let last = hand.play_card(1);
        assert_eq!(last.unwrap().rank, Rank::Four);

        let remaining = hand.play_card(0);
        assert_eq!(remaining.unwrap().rank, Rank::Three);
    }

    #[test]
    fn test_hand_play_empty_hand() {
        let mut hand = Hand::default();
        assert!(hand.is_empty());

        let card = hand.play_card(0);
        assert!(card.is_none());
        assert!(hand.is_empty());
    }
}
