//! Game Title UI - placeholder (now handled by root_ui)

use bevy::prelude::*;

/// Empty plugin - title rendering now in root_ui
pub struct TitleUiPlugin;

impl Plugin for TitleUiPlugin {
    fn build(&self, _app: &mut App) {
        // Title rendered by root_ui
    }
}
