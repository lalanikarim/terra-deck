//! Root UI Container - holds game UI elements

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

/// Spawn UI elements (title and combat log)
pub fn spawn_root_ui_container(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load fonts
    let title_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let log_font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Title element - white text, centered
    let title_element = (
        Text::new("Terra-Deck".to_string()),
        TextFont {
            font: title_font,
            font_size: 52.0,
            ..Default::default()
        },
        TextColor(white()),
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
    );

    // Combat log element - at left with left margin, text centered within
    let log_element = (
        Node {
            width: Val::Px(350.0),
            min_width: Val::Px(350.0),
            min_height: Val::Px(400.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,  // Content centered inside container
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

    // Spawn title as top-level centered element
    commands.spawn((
        Node {
            width: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![(title_element)],
    ));

    // Spawn combat log as top-level element with left margin
    // Use fixed position or margin to anchor to left
    commands.spawn((
        Node {
            width: Val::Px(350.0),
            min_width: Val::Px(350.0),
            min_height: Val::Px(400.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            ..Default::default()
        },
        children![(log_element)],
    ));
}

/// Plugin for root UI container
pub struct RootUiContainerPlugin;

impl Plugin for RootUiContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_root_ui_container);
    }
}
