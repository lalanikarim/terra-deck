//! Core types and enums for the Poker Card RPG game

use bevy::prelude::Component;

/// Represents the combat archetype relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Archetype {
    Rock,
    Paper,
    Scissors,
    Infantry,
}

/// Represents the four suits in a deck of cards, each with a combat archetype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Suit {
    Hearts,   // Rock archetype
    Diamonds, // Paper archetype
    Clubs,    // Scissors archetype
    Spades,   // Infantry archetype
}

impl Suit {
    /// Returns the combat archetype for this suit
    pub fn archetype(&self) -> Archetype {
        match self {
            Suit::Hearts => Archetype::Rock,
            Suit::Diamonds => Archetype::Paper,
            Suit::Clubs => Archetype::Scissors,
            Suit::Spades => Archetype::Infantry,
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Hearts => "♥",
                Suit::Diamonds => "♦",
                Suit::Clubs => "♣",
                Suit::Spades => "♠",
            }
        )
    }
}

/// Represents the rank/value of a card (2-10, J, Q, K, A)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            }
        )
    }
}
