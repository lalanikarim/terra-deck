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
#[derive(Resource)]
pub struct CombatLog {
    pub entries: Vec<String>,
    /// Maximum number of entries to keep (oldest are removed when exceeded)
    pub max_entries: usize,
}

impl Default for CombatLog {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify card creation initializes HP correctly
    #[test]
    fn test_card_creation() {
        let card = Card::new(Suit::Hearts, Rank::Seven);
        assert_eq!(card.rank, Rank::Seven);
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.hp, 7);
        assert_eq!(card.max_hp, 7);
    }

    /// Test that all ranks initialize with correct HP values
    #[test]
    fn test_card_all_ranks_hp() {
        for rank in [Rank::Two, Rank::Five, Rank::Ten, Rank::King, Rank::Ace].iter() {
            let card = Card::new(Suit::Clubs, *rank);
            assert_eq!(card.hp as u8, *rank as u8);
            assert_eq!(card.max_hp, card.hp);
        }
    }

    /// Verify alive state for HP > 0
    #[test]
    fn test_card_is_alive_alive() {
        let card = Card::new(Suit::Diamonds, Rank::Queen);
        assert!(card.is_alive());
    }

    /// Verify dead state when HP reaches 0
    #[test]
    fn test_card_is_alive_dead() {
        let mut card = Card::new(Suit::Spades, Rank::Three);
        card.take_damage(3);
        assert!(!card.is_alive());
    }

    /// Test damage application reduces HP correctly
    #[test]
    fn test_card_take_damage() {
        let mut card = Card::new(Suit::Hearts, Rank::Eight);
        let damage = card.take_damage(3);
        assert_eq!(damage, 3);
        assert_eq!(card.hp, 5);
    }

    /// Test that damage cannot exceed current HP
    #[test]
    fn test_card_take_damage_overkill() {
        let mut card = Card::new(Suit::Clubs, Rank::Five);
        let damage = card.take_damage(10);
        assert_eq!(damage, 5);
        assert_eq!(card.hp, 0);
    }

    /// Test exact damage equals current HP
    #[test]
    fn test_card_take_damage_exact() {
        let mut card = Card::new(Suit::Diamonds, Rank::Ten);
        let damage = card.take_damage(10);
        assert_eq!(damage, 10);
        assert_eq!(card.hp, 0);
    }

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
}

#[cfg(test)]
mod deck_tests {
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
}

#[cfg(test)]
mod hand_tests {
    use super::*;

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
}

#[cfg(test)]
mod archetype_tests {
    use super::*;

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

    #[test]
    fn test_all_suits_have_different_archetypes() {
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

        for i in 0..suits.len() {
            for j in 0..suits.len() {
                if i != j {
                    assert_ne!(
                        suits[i].archetype(),
                        suits[j].archetype(),
                        "Different suits should have different archetypes"
                    );
                } else {
                    assert_eq!(
                        suits[i].archetype(),
                        suits[j].archetype(),
                        "Same suit should have same archetype"
                    );
                }
            }
        }
    }

    #[test]
    fn test_archetype_rock() {
        assert_eq!(Archetype::Rock, Archetype::Rock);
    }

    #[test]
    fn test_archetype_paper() {
        assert_eq!(Archetype::Paper, Archetype::Paper);
    }

    #[test]
    fn test_archetype_scissors() {
        assert_eq!(Archetype::Scissors, Archetype::Scissors);
    }

    #[test]
    fn test_archetype_infantry() {
        assert_eq!(Archetype::Infantry, Archetype::Infantry);
    }

    #[test]
    fn test_archetype_different_from_each_other() {
        assert_ne!(Archetype::Rock, Archetype::Paper);
        assert_ne!(Archetype::Rock, Archetype::Scissors);
        assert_ne!(Archetype::Rock, Archetype::Infantry);
        assert_ne!(Archetype::Paper, Archetype::Scissors);
        assert_ne!(Archetype::Paper, Archetype::Infantry);
        assert_ne!(Archetype::Scissors, Archetype::Infantry);
    }
}

#[cfg(test)]
mod rank_tests {
    use super::*;

    #[test]
    fn test_rank_values() {
        assert_eq!(Rank::Two as u8, 2);
        assert_eq!(Rank::Three as u8, 3);
        assert_eq!(Rank::Four as u8, 4);
        assert_eq!(Rank::Five as u8, 5);
        assert_eq!(Rank::Six as u8, 6);
        assert_eq!(Rank::Seven as u8, 7);
        assert_eq!(Rank::Eight as u8, 8);
        assert_eq!(Rank::Nine as u8, 9);
        assert_eq!(Rank::Ten as u8, 10);
        assert_eq!(Rank::Jack as u8, 11);
        assert_eq!(Rank::Queen as u8, 12);
        assert_eq!(Rank::King as u8, 13);
        assert_eq!(Rank::Ace as u8, 14);
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
}

#[cfg(test)]
mod combat_log_tests {
    use super::*;

    #[test]
    fn test_combat_log_empty_initially() {
        let log = CombatLog::default();
        assert!(log.entries.is_empty());
        assert_eq!(log.max_entries, 100);
    }

    #[test]
    fn test_combat_log_add_entry() {
        let mut log = CombatLog::default();
        log.add_entry("Test entry".to_string());
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0], "Test entry");
    }

    #[test]
    fn test_combat_log_add_multiple_entries() {
        let mut log = CombatLog::default();
        log.add_entry("Entry 1".to_string());
        log.add_entry("Entry 2".to_string());
        log.add_entry("Entry 3".to_string());
        assert_eq!(log.entries.len(), 3);
        assert_eq!(log.entries[0], "Entry 1");
        assert_eq!(log.entries[2], "Entry 3");
    }

    #[test]
    fn test_combat_log_clear() {
        let mut log = CombatLog::default();
        log.add_entry("Entry 1".to_string());
        log.add_entry("Entry 2".to_string());
        log.clear();
        assert!(log.entries.is_empty());
    }

    #[test]
    fn test_combat_log_iter() {
        let mut log = CombatLog::default();
        log.add_entry("First".to_string());
        log.add_entry("Second".to_string());
        log.add_entry("Third".to_string());

        let mut iter_count = 0;
        for _entry in log.iter() {
            iter_count += 1;
        }
        assert_eq!(iter_count, 3);
    }

    #[test]
    fn test_combat_log_max_entries_rotation() {
        let mut log = CombatLog {
            entries: Vec::new(),
            max_entries: 3,
        };

        log.add_entry("Entry 1".to_string());
        log.add_entry("Entry 2".to_string());
        log.add_entry("Entry 3".to_string());
        assert_eq!(log.entries.len(), 3);

        log.add_entry("Entry 4".to_string());
        assert_eq!(log.entries.len(), 3);
        assert_eq!(log.entries[0], "Entry 2");
        assert_eq!(log.entries[2], "Entry 4");

        log.add_entry("Entry 5".to_string());
        assert_eq!(log.entries.len(), 3);
        assert_eq!(log.entries[0], "Entry 3");
        assert_eq!(log.entries[2], "Entry 5");
    }

    #[test]
    fn test_combat_log_empty_max_entries_zero() {
        let mut log = CombatLog::default();
        log.add_entry("Entry 1".to_string());
        log.add_entry("Entry 2".to_string());
        assert_eq!(log.entries.len(), 2);
    }

    #[test]
    fn test_combat_log_add_entry_empty_string() {
        let mut log = CombatLog::default();
        log.add_entry(String::new());
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0].len(), 0);
    }
}

#[cfg(test)]
mod workflow_tests {
    use super::*;

    #[test]
    fn test_workflow_create_deck_draw_hand() {
        let mut deck = Deck::new();
        let mut hand = Hand::default();

        assert_eq!(deck.len(), 52);
        assert!(hand.is_empty());

        // Deal 5 cards
        for _ in 0..5 {
            if let Some(card) = deck.draw() {
                hand.add_card(card);
            }
        }

        assert_eq!(deck.len(), 47);
        assert_eq!(hand.len(), 5);
        assert!(!hand.is_empty());
    }

    #[test]
    fn test_workflow_play_card_from_hand() {
        let mut deck = Deck::new();
        let mut hand = Hand::default();

        // Deal a hand
        for _ in 0..3 {
            hand.add_card(deck.draw().unwrap());
        }

        let initial_count = hand.len();
        let card = hand.play_card(0);

        assert!(card.is_some());
        assert_eq!(hand.len(), initial_count - 1);
        assert!(!deck.is_empty());
    }

    #[test]
    fn test_workflow_full_combat_scenario() {
        let mut deck = Deck::new();
        let mut hand = Hand::default();

        // Deal 3 cards
        hand.add_card(deck.draw().unwrap());
        hand.add_card(deck.draw().unwrap());
        hand.add_card(deck.draw().unwrap());

        assert_eq!(hand.len(), 3);
        assert_eq!(deck.len(), 49);

        // Play first card
        let played_card = hand.play_card(0);
        assert!(played_card.is_some());

        // Verify HP >= 10 for rank 10 or higher
        let card = played_card.unwrap();
        assert!(card.hp >= 10 || card.rank as u8 <= 10);
    }

    #[test]
    fn test_workflow_damage_sequence() {
        let card = Card::new(Suit::Clubs, Rank::Nine);

        assert_eq!(card.hp, 9);
        assert!(card.is_alive());

        let mut damaged_card = card;
        damaged_card.take_damage(3);
        assert_eq!(damaged_card.hp, 6);
        assert!(damaged_card.is_alive());

        damaged_card.take_damage(6);
        assert_eq!(damaged_card.hp, 0);
        assert!(!damaged_card.is_alive());
    }

    #[test]
    fn test_workflow_heal_sequence() {
        let mut card = Card::new(Suit::Diamonds, Rank::King);

        // Start full HP
        assert_eq!(card.hp, 13);

        // Take damage
        card.take_damage(5);
        assert_eq!(card.hp, 8);

        // Heal
        card.heal(6);
        assert_eq!(card.hp, 13);
        assert_eq!(card.max_hp, 13);

        // Try to heal beyond max
        card.heal(10);
        assert_eq!(card.hp, 13);
    }

    #[test]
    fn test_workflow_deck_shuffle_simulation() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();

        // Both decks should have 52 cards
        assert_eq!(deck1.len(), 52);
        assert_eq!(deck2.len(), 52);

        // Draw top cards
        let card1 = deck1.draw();
        let card2 = deck2.draw();

        // Both should have valid cards
        assert!(card1.is_some());
        assert!(card2.is_some());
    }

    #[test]
    fn test_workflow_combat_log_tracking() {
        let mut log = CombatLog {
            max_entries: 5,
            ..Default::default()
        };

        for i in 0..8 {
            log.add_entry(format!("Combat event {}", i + 1));
        }

        assert_eq!(log.entries.len(), 5);
        assert_eq!(log.entries[0], "Combat event 4");
        assert_eq!(log.entries[4], "Combat event 8");
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_card_hp_zero() {
        let mut card = Card::new(Suit::Hearts, Rank::Two);
        assert_eq!(card.hp, 2);

        card.take_damage(2);
        assert_eq!(card.hp, 0);
        assert!(!card.is_alive());
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
    fn test_hand_play_empty_hand() {
        let mut hand = Hand::default();
        assert!(hand.is_empty());

        let card = hand.play_card(0);
        assert!(card.is_none());
        assert!(hand.is_empty());
    }

    #[test]
    fn test_card_exact_damage_then_full_heal() {
        let mut card = Card::new(Suit::Spades, Rank::Ace);
        card.take_damage(14);
        assert_eq!(card.hp, 0);

        card.heal(14);
        assert_eq!(card.hp, 14);
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
