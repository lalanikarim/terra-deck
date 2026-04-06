/// Resource representing combat statistics for a turn
use bevy::prelude::Resource;
use std::clone::Clone;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Copy;

/// Resource representing the game state (turn management)
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerTurn,
    OpponentTurn,
    CombatResolution,
    GameOver,
}

/// Resource representing combat statistics for a turn
#[derive(Resource, Default)]
pub struct CombatStats {
    pub player_damage_dealt: u8,
    pub opponent_damage_dealt: u8,
    pub player_crits: u8,
    pub opponent_crits: u8,
    pub player_absorbs: u8,
    pub opponent_absorbs: u8,
}
