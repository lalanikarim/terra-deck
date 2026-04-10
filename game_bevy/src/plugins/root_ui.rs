//! Root UI Container - holds all game UI elements

use bevy::prelude::*;

/// Spawn root container that wraps all UI elements
pub fn spawn_root_ui_container(
    mut commands: Commands,
) {
    // Root container - full width column, center children
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
    ));
}

/// Plugin for root UI container
pub struct RootUiContainerPlugin;

impl Plugin for RootUiContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_root_ui_container);
    }
}
