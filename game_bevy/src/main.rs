use bevy::prelude::*;
use game_bevy::plugins::{CardAssetPlugin, GameSessionPlugin, GameSessionResource};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Player Hand Demo".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CardAssetPlugin)
        .add_plugins(GameSessionPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<'_, AssetServer>,
    cameras: Query<Entity, With<Camera2d>>,
    game_resource: Res<'_, GameSessionResource>,
) {
    println!("🃏 Starting player hand demo");

    // Kill default 3D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    // Display game session info
    println!("📊 Game State: {}", game_resource.session.loop_state);
    println!(
        "📊 Player hand: {} cards, Opponent hand: {} cards",
        game_resource.session.player_hand.len(),
        game_resource.session.opponent_hand.len()
    );

    // Show 5 test cards from various suits/ranks
    for i in 0..5 {
        let suit = match i % 4 {
            0 => "card_hearts",
            1 => "card_spades",
            2 => "card_diamonds",
            3 => "card_clubs",
            _ => "card_hearts",
        };
        let rank = match i {
            0 | 1 => "02",
            2 | 3 => "05",
            4 => "07",
            _ => "02",
        };

        let path = format!(
            "kenney_playing-cards-pack/PNG/Cards (medium)/{}_{}.png",
            suit, rank
        );
        commands.spawn((
            Sprite {
                image: asset_server.load(&path),
                custom_size: Some(Vec2::new(120.0, 180.0)),
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
            Transform::from_xyz(i as f32 * 100.0 - 250.0, 0.0, 0.0),
        ));
    }

    println!("🎴 Ready to display player hand!");
}
