//! Core game logic for Poker Card RPG
//! This module contains the ECS components, resources, and systems
//! that define the game mechanics independent of the UI layer.

use bevy::prelude::*;

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

/// Resource representing the game deck
#[derive(Resource)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Creates a new shuffled deck of 52 cards
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        // Generate all combinations of suits and ranks
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

    /// Returns true if the hand is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Gets a reference to a card at the specified index
    pub fn get(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }
}

/// Resource representing the game state (turn management)
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerTurn,
    OpponentTurn,
    CombatResolution,
    GameOver,
}

/// Resource for tracking combat events/log
#[derive(Resource, Default)]
pub struct CombatLog {
    pub entries: Vec<String>,
    pub max_entries: usize,
}

impl CombatLog {
    /// Adds a new entry to the combat log
    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    /// Clears the combat log
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns an iterator over the log entries
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.iter()
    }
}
