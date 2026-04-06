//! Core types and enums for the Poker Card RPG game

use bevy::prelude::Component;
use std::fmt::Display;

/// Represents the combat archetype relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Archetype {
    Rock,
    Paper,
    Scissors,
    Infantry,
}

impl Display for Archetype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Archetype::Rock => "Rock",
                Archetype::Paper => "Paper",
                Archetype::Scissors => "Scissors",
                Archetype::Infantry => "Infantry",
            }
        )
    }
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

impl Display for Suit {
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

impl Display for Rank {
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

#[cfg(test)]
mod tests {
    use super::*;

    // === Archetype Tests ===

    #[test]
    fn test_suit_archetype_heart_is_rock() {
        let suit = Suit::Hearts;
        assert_eq!(suit.archetype(), Archetype::Rock);
    }

    #[test]
    fn test_suit_archetype_diamond_is_paper() {
        let suit = Suit::Diamonds;
        assert_eq!(suit.archetype(), Archetype::Paper);
    }

    #[test]
    fn test_suit_archetype_club_is_scissors() {
        let suit = Suit::Clubs;
        assert_eq!(suit.archetype(), Archetype::Scissors);
    }

    #[test]
    fn test_suit_archetype_spade_is_infantry() {
        let suit = Suit::Spades;
        assert_eq!(suit.archetype(), Archetype::Infantry);
    }

    // === Suit Display Tests ===

    #[test]
    fn test_suit_display() {
        assert_eq!(format!("{}", Suit::Hearts), "♥");
        assert_eq!(format!("{}", Suit::Diamonds), "♦");
        assert_eq!(format!("{}", Suit::Clubs), "♣");
        assert_eq!(format!("{}", Suit::Spades), "♠");
    }

    // === Rank Tests ===

    #[test]
    fn test_rank_values() {
        assert_eq!(Rank::Two as u8, 2);
        assert_eq!(Rank::Five as u8, 5);
        assert_eq!(Rank::Ten as u8, 10);
        assert_eq!(Rank::Jack as u8, 11);
        assert_eq!(Rank::Queen as u8, 12);
        assert_eq!(Rank::King as u8, 13);
        assert_eq!(Rank::Ace as u8, 14);
    }

    #[test]
    fn test_rank_display() {
        assert_eq!(format!("{}", Rank::Two), "2");
        assert_eq!(format!("{}", Rank::Ten), "10");
        assert_eq!(format!("{}", Rank::Jack), "J");
        assert_eq!(format!("{}", Rank::Queen), "Q");
        assert_eq!(format!("{}", Rank::King), "K");
        assert_eq!(format!("{}", Rank::Ace), "A");
    }

    #[test]
    fn test_rank_ord() {
        assert!(Rank::Two < Rank::Five);
        assert!(Rank::Ten < Rank::Jack);
        assert!(Rank::King < Rank::Ace);
        assert!(Rank::Five >= Rank::Five);
    }

    #[test]
    fn test_rank_partial_ord() {
        let rank1 = Rank::Seven;
        let rank2 = Rank::Seven;
        assert!(rank1.partial_cmp(&rank2).unwrap().is_eq());
        let rank3 = Rank::King;
        assert!(rank3.partial_cmp(&rank1).unwrap().is_gt());
    }

    #[test]
    fn test_rank_eq() {
        assert_eq!(Rank::King, Rank::King);
        assert_ne!(Rank::Ace, Rank::King);
    }

    #[test]
    fn test_rank_clone_copy() {
        let rank = Rank::Queen;
        let rank_copy = rank;
        assert_eq!(rank, rank_copy);
    }

    #[test]
    fn test_archetype_display() {
        assert_eq!(format!("{}", Archetype::Rock), "Rock");
        assert_eq!(format!("{}", Archetype::Paper), "Paper");
        assert_eq!(format!("{}", Archetype::Scissors), "Scissors");
        assert_eq!(format!("{}", Archetype::Infantry), "Infantry");
    }
}
