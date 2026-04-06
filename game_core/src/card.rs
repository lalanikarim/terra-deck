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
