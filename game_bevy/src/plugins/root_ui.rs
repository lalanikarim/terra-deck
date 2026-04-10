//! Root UI Container - holds game UI elements with proper flexbox layout

use bevy::prelude::*;

// Color helpers
fn black() -> Color {
    Color::srgb(0.0, 0.0, 0.0)
}

fn white() -> Color {
    Color::srgb(1.0, 1.0, 1.0)
}

fn lightgray() -> Color {
    Color::srgb(0.85, 0.85, 0.85)
}

/// Spawn UI elements with proper flexbox layout
pub fn spawn_root_ui_container(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load fonts
    let title_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let log_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Title element - centered horizontally
    let title_element = (
        Text::new("Terra-Deck".to_string()),
        TextFont {
            font: title_font,
            font_size: 52.0,
            ..Default::default()
        },
        TextColor(white()),
        Node {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
    );

    // Combat log container - left-aligned with margin
    let log_element = (
        Node {
            width: Val::Px(350.0),
            min_width: Val::Px(350.0),
            min_height: Val::Px(400.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..Default::default()
        },
        BackgroundColor(lightgray()),
        children![(
            Text::new("COMBAT LOG".to_string()),
            TextFont {
                font: log_font,
                font_size: 22.0,
                ..Default::default()
            },
            TextColor(black()),
        )],
    );

    // Root container - full width with centered children
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
        children![
            title_element,
            log_element,
        ],
    ));
}

/// Plugin for root UI container
pub struct RootUiContainerPlugin;

impl Plugin for RootUiContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_root_ui_container);
    }
}
