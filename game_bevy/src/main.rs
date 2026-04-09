use bevy::prelude::*;
use game_bevy::plugins::{GameSessionPlugin, GameSessionResource};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Bridge Demo".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameSessionPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    cameras: Query<Entity, With<Camera2d>>,
    game_resource: Res<GameSessionResource>,
) {
    println!("🃏 Starting bridge demo with GameSessionResource");

    // Kill default 3D camera and spawn 2D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    // Display game session info from bridge
    println!(
        "📊 Game Session: {} - {} cards each",
        game_resource.session.loop_state,
        game_resource.session.player_hand.len()
    );

    // Load a few test cards to display
    let heart_A =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_hearts_A.png");
    let spade_K =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_spades_K.png");
    let diamond_Q =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_diamonds_Q.png");
    let clover_7 =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_clubs_07.png");
    let card_back = asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_back.png");

    // Display 5 cards in an arc layout
    let positions = [
        (-500.0, 0.0),   // Heart A - far left
        (-200.0, 100.0), // Spade K - mid left, elevated
        (0.0, 200.0),    // Diamond Q - center, highest
        (250.0, 100.0),  // Clover 7 - mid right, elevated
        (550.0, 0.0),    // Card back - far right
    ];

    let cards = [
        (heart_A, positions[0]),
        (spade_K, positions[1]),
        (diamond_Q, positions[2]),
        (clover_7, positions[3]),
        (card_back, positions[4]),
    ];

    for (card_handle, (x, y)) in cards {
        commands.spawn((
            Sprite {
                image: card_handle,
                custom_size: Some(Vec2::new(180.0, 270.0)),
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    println!("🂫 Displayed 5 test cards in arc layout!");
}
