//! Terra-Deck Bevy 2D Renderer
//!
//! This crate provides a graphical rendering layer for Terra-Deck's `game_core`.
//! The game logic remains in `game_core`, while `game_bevy` handles 2D rendering.

pub mod plugins;

// Re-export plugins for external use
pub use plugins::CardAssetPlugin;
