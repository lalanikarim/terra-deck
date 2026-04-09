//! Plugins for game_bevy

pub mod assets;
pub mod bridge;
pub mod card_renderer;
pub mod opponent_renderer;

pub use assets::CardAssetPlugin;
pub use bridge::{GameSessionPlugin, GameSessionResource};
pub use card_renderer::render_player_hand;
pub use opponent_renderer::{
    render_opponent_hand, GameTarget, OpponentCard, OpponentRendererPlugin,
};
