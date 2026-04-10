//! Log Container UI - displays combat log with gray background on left side

use bevy::prelude::*;

// Color helpers
fn black() -> Color {
    Color::srgb(0.0, 0.0, 0.0)
}

fn lightgray() -> Color {
    Color::srgb(0.85, 0.85, 0.85)
}

/// Spawn the combat log container
pub fn spawn_log_container(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Node {
            width: Val::Px(350.0),
            min_width: Val::Px(350.0),
            min_height: Val::Px(400.0),
            margin: UiRect {
                left: Val::Px(250.0),
                ..Default::default()
            },
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(15.0)),
            ..Default::default()
        },
        BackgroundColor(lightgray()),
        (
            Text::new("COMBAT LOG".to_string()),
            TextFont {
                font,
                font_size: 22.0,
                ..Default::default()
            },
            TextColor(black()),
        ),
    ));
}

/// Plugin for combat log container UI
pub struct LogContainerUiPlugin;

impl Plugin for LogContainerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_log_container);
    }
}
