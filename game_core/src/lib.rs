//! Core game logic for Poker Card RPG
//! This module contains the ECS components, resources, and systems
//! that define the game mechanics independent of the UI layer.

pub mod ai;
pub mod card;
pub mod combat;
pub mod combat_log;
pub mod combat_stats;
pub mod deck;
pub mod game_loop;
pub mod game_session;
pub mod hand;
pub mod systems;
pub mod turn_state;
pub mod types;

// Re-export common types for convenience
pub use card::Card;
pub use combat::{apply_combat_damage, calculate_damage_multiplier, CombatResult};
pub use combat_log::CombatLog;
pub use combat_stats::{CombatStats, GameResult, GameState};
pub use deck::Deck;
pub use game_session::GameSession;
pub use hand::Hand;
pub use turn_state::SelectedCard;
pub use types::{Archetype, Rank, Suit};
