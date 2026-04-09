//! Plugins for game_bevy

pub mod assets;
pub mod bridge;
pub mod card_renderer;

pub use assets::CardAssetPlugin;
pub use bridge::{GameSessionPlugin, GameSessionResource};
pub use card_renderer::render_player_hand;
