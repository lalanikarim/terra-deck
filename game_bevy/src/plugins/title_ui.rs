//! Game Title UI - displays the main game title at top center

use bevy::prelude::*;

// Color helper
fn white() -> Color {
    Color::srgb(1.0, 1.0, 1.0)
}

/// Spawn the game title element
pub fn spawn_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Text::new("Terra-Deck".to_string()),
        TextFont {
            font,
            font_size: 52.0,
            ..Default::default()
        },
        TextColor(white()),
        Node {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
    ));
}

/// Plugin for game title UI
pub struct TitleUiPlugin;

impl Plugin for TitleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_title);
    }
}
