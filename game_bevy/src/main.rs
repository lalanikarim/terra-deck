use bevy::prelude::*;
use game_bevy::plugins::opponent_renderer::render_opponent_hand;
use game_bevy::plugins::{
    CardAssetPlugin, CombatLogUiPlugin, GameSessionPlugin, OpponentRendererPlugin,
    PlayerRendererPlugin, RootUiContainerPlugin, TitleUiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Terra-Deck".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CardAssetPlugin)
        .add_plugins(GameSessionPlugin)
        .add_plugins(PlayerRendererPlugin)
        .add_plugins(OpponentRendererPlugin)
        .add_plugins(CombatLogUiPlugin)
        .add_plugins(TitleUiPlugin)
        .add_plugins(RootUiContainerPlugin)
        .add_systems(Startup, initialize)
        .add_systems(Update, render_opponent_hand)
        .run();
}

fn initialize(mut commands: Commands, cameras: Query<Entity, With<Camera2d>>) {
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    // No ClearColor set - default background
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));
}
