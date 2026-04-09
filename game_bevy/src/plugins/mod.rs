//! Plugins for game_bevy
//!
//! This module re-exports all plugins that make up the Bevy game renderer.

pub mod assets;
pub mod bridge;

pub use assets::CardAssetPlugin;
pub use bridge::{GameSessionPlugin, GameSessionResource};
