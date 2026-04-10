//! Combat Log UI - displays game events in Bevy's UI system

use bevy::prelude::*;

/// Helper to make color black for high contrast text
fn black() -> Color {
    Color::srgb(0.0, 0.0, 0.0)
}

/// Helper to make color light gray for container background  
fn lightgray() -> Color {
    Color::srgb(0.85, 0.85, 0.85)
}

/// System to render combat log entries as UI text  
pub fn render_combat_log_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font,
        font_size: 28.0,
        ..Default::default()
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(400.0),
            min_height: Val::Px(100.0),
            padding: UiRect::all(Val::Px(15.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(35.0),
            ..Default::default()
        },
        BackgroundColor(lightgray()),
        children![(
            Text::new("=== COMBAT LOG ===".to_string()),
            text_font,
            TextColor(black())
        )],
    ));
}

/// Plugin for combat log UI rendering
pub struct CombatLogUiPlugin;

impl Plugin for CombatLogUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_combat_log_ui);
    }
}
