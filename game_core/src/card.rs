//! Card entity and implementation

use crate::types::{Rank, Suit};
use bevy::prelude::*;

/// A card entity in the game
#[derive(Debug, Clone, Component)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    /// Current hit points (starts as rank value, can be modified by damage)
    pub hp: u8,
    /// Maximum hit points (based on rank)
    pub max_hp: u8,
}

impl Card {
    /// Creates a new card with the given suit and rank
    pub fn new(suit: Suit, rank: Rank) -> Self {
        let hp = rank as u8;
        Card {
            suit,
            rank,
            hp,
            max_hp: hp,
        }
    }

    /// Returns true if the card is still alive (has HP > 0)
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Applies damage to the card, returning the actual damage dealt
    pub fn take_damage(&mut self, damage: u8) -> u8 {
        let actual_damage = damage.min(self.hp);
        self.hp -= actual_damage;
        actual_damage
    }

    /// Heals the card, up to its maximum HP
    pub fn heal(&mut self, amount: u8) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test healing increases HP up to max
    #[test]
    fn test_card_heal() {
        let mut card = Card::new(Suit::Spades, Rank::Ace);
        card.take_damage(5);
        assert_eq!(card.hp, 9);

        card.heal(3);
        assert_eq!(card.hp, 12);
        assert_eq!(card.max_hp, 14);
    }

    /// Test healing cannot exceed max HP
    #[test]
    fn test_card_heal_over_max() {
        let mut card = Card::new(Suit::Hearts, Rank::King);
        card.take_damage(1);
        assert_eq!(card.hp, 12);

        card.heal(10);
        assert_eq!(card.hp, 13);
        assert_eq!(card.max_hp, 13);
    }

    /// Test healing with zero does nothing
    #[test]
    fn test_card_heal_zero() {
        let mut card = Card::new(Suit::Diamonds, Rank::Jack);
        let hp_before = card.hp;
        card.heal(0);
        assert_eq!(card.hp, hp_before);
    }

    /// Test damage of 1
    #[test]
    fn test_card_take_damage_one_hit() {
        let mut card = Card::new(Suit::Clubs, Rank::Four);
        let damage = card.take_damage(1);
        assert_eq!(damage, 1);
        assert_eq!(card.hp, 3);
    }

    /// Full damage sequence: take damage and heal
    #[test]
    fn test_card_damage_then_heal() {
        let mut card = Card::new(Suit::Spades, Rank::Nine);
        assert_eq!(card.hp, 9);

        card.take_damage(4);
        assert_eq!(card.hp, 5);

        card.heal(3);
        assert_eq!(card.hp, 8);

        card.take_damage(8);
        assert!(!card.is_alive());
    }

    /// Test card reaches zero HP
    #[test]
    fn test_card_hp_zero() {
        let mut card = Card::new(Suit::Hearts, Rank::Two);
        assert_eq!(card.hp, 2);
        card.take_damage(2);
        assert_eq!(card.hp, 0);
        assert!(!card.is_alive());
    }

    /// Test exact damage then full heal
    #[test]
    fn test_card_exact_damage_then_full_heal() {
        let mut card = Card::new(Suit::Spades, Rank::Ace);
        card.take_damage(14);
        assert_eq!(card.hp, 0);
        card.heal(14);
        assert_eq!(card.hp, 14);
    }
}
