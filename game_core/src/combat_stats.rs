//! Combat statistics and game state
use bevy::prelude::Resource;

/// Result of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameResult {
    Won,
    Lost,
    Draw,
}

/// Current state of the game
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerTurn,
    OpponentTurn,
    CombatResolution,
    GameOver(GameResult),
}

/// Combat statistics for a turn
#[derive(Resource, Default)]
pub struct CombatStats {
    pub player_damage_dealt: u8,
    pub opponent_damage_dealt: u8,
    pub player_crits: u8,
    pub opponent_crits: u8,
    pub player_absorbs: u8,
    pub opponent_absorbs: u8,
}
