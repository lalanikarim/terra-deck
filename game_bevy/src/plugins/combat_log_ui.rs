//! Combat Log UI - placeholder (now handled by root_ui)

use bevy::prelude::*;

/// Empty plugin - combat log rendering now in root_ui
pub struct CombatLogUiPlugin;

impl Plugin for CombatLogUiPlugin {
    fn build(&self, _app: &mut App) {
        // Combat log rendered by root_ui
    }
}
