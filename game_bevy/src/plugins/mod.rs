//! Plugins for game_bevy

pub mod assets;
pub mod bridge;
pub mod card_renderer;
pub mod combat_log_ui;
pub mod log_container_ui;
pub mod opponent_renderer;
pub mod root_ui;
pub mod title_ui;

pub use assets::CardAssetPlugin;
pub use bridge::{GameSessionPlugin, GameSessionResource};
pub use card_renderer::{render_player_hand, PlayerRendererPlugin};
pub use combat_log_ui::CombatLogUiPlugin;
pub use log_container_ui::LogContainerUiPlugin;
pub use opponent_renderer::{render_opponent_hand, OpponentRendererPlugin};
pub use root_ui::RootUiContainerPlugin;
pub use title_ui::TitleUiPlugin;

pub use bevy::prelude::*;
