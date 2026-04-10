//! Plugins for game_bevy

pub mod assets;
pub mod bridge;
pub mod card_renderer;
pub mod combat_log_ui;
pub mod opponent_renderer;

pub use assets::CardAssetPlugin;
pub use bridge::{GameSessionPlugin, GameSessionResource};
pub use card_renderer::{render_player_hand, PlayerRendererPlugin};
pub use combat_log_ui::{render_combat_log_ui, CombatLogUiPlugin};
pub use opponent_renderer::{render_opponent_hand, OpponentRendererPlugin};
