//! Root UI Container - centers all game UI elements horizontally

use bevy::prelude::*;

/// Spawn root container that holds title and combat log as children
pub fn spawn_root_ui_container(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load fonts
    let title_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let log_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Spawn title as child with its own centering
    let title_centering = (
        Node {
            width: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![(
            Text::new("🃏 Terra-Deck 🃏".to_string()),
            TextFont {
                font: title_font,
                font_size: 42.0,
                ..Default::default()
            },
            TextColor(Color::srgb(0.0, 0.0, 0.0)),
        )],
    );

    // Spawn combat log element with its own centering  
    let log_element = (
        Node {
            width: Val::Px(350.0),
            min_width: Val::Px(350.0),
            min_height: Val::Px(400.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,  // Center content within container
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.85, 0.85, 0.85)),
        children![(
            Text::new("COMBAT LOG".to_string()),
            TextFont {
                font: log_font,
                font_size: 22.0,
                ..Default::default()
            },
            TextColor(Color::srgb(0.0, 0.0, 0.0)),
        )],
    );

    // Root container - full width, center children
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,  // Center all children horizontally
            width: Val::Percent(100.0),  // Full width
            ..Default::default()
        },
        children![(title_centering), (log_element)],
    ));
}

/// Plugin for root UI container
pub struct RootUiContainerPlugin;

impl Plugin for RootUiContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_root_ui_container);
    }
}
