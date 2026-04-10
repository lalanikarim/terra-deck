//! Game Title UI - displays the main game title at top of screen

use bevy::prelude::*;

/// Helper to make color black for high contrast text
fn black() -> Color {
    Color::srgb(0.0, 0.0, 0.0)
}

/// System to spawn the game title
pub fn spawn_title(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font,
        font_size: 42.0,
        ..Default::default()
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(10.0),
            width: Val::Px(400.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![(
            Text::new("🃏 Terra-Deck 🃏".to_string()),
            text_font,
            TextColor(black()),
        )],
    ));
}

/// Plugin for game title UI
pub struct TitleUiPlugin;

impl Plugin for TitleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_title);
    }
}
