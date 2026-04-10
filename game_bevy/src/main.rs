use bevy::prelude::*;
use game_bevy::plugins::combat_log_ui::render_combat_log_ui;
use game_bevy::plugins::opponent_renderer::render_opponent_hand;
use game_bevy::plugins::{
    CardAssetPlugin, CombatLogUiPlugin, GameSessionPlugin, OpponentRendererPlugin,
    PlayerRendererPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Terra-Deck".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_plugins(CardAssetPlugin)
        .add_plugins(GameSessionPlugin)
        .add_plugins(PlayerRendererPlugin)
        .add_plugins(OpponentRendererPlugin)
        .add_plugins(CombatLogUiPlugin)
        .add_systems(Startup, initialize)
        .add_systems(Update, (render_opponent_hand, render_combat_log_ui))
        .run();
}

fn initialize(mut commands: Commands, cameras: Query<Entity, With<Camera2d>>) {
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));
}
