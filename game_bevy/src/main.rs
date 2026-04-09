use bevy::prelude::*;
use game_bevy::plugins::{GameSessionPlugin, GameSessionResource, CardAssetPlugin};

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
    asset_server: Res<'_, AssetServer>,
    mut commands: Commands,
    cameras: Query<Entity, With<Camera2d>>,
    game_resource: Res<'_, GameSessionResource>,
) {
    println!("🃏 Starting player hand demo with bridge");
    
    // Kill default 3D camera and spawn 2D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    // Display game session info
    println!("📊 Game State: {} - Player: {} cards, Opponent: {} cards", 
             game_resource.session.loop_state,
             game_resource.session.player_hand.len(),
             game_resource.session.opponent_hand.len());

    // Load card back for demo
    let card_back = asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_back.png");

    // Display 10 demo cards of various types
    // Using small/medium size to fit better on screen
    let card_sizes = [
        100.0, 110.0, 120.0, 130.0, 140.0, 
        130.0, 120.0, 110.0, 100.0, 115.0
    ];
    let card_paths = [
        "card_hearts_02", "card_hearts_05", "card_hearts_08", "card_hearts_J",
        "card_hearts_K", "card_spades_03", "card_spades_07", "card_spades_Q",
        "card_diamonds_04", "card_diamonds_K"
    ];
    let positions = [
        (-500.0, 0.0), (-350.0, 30.0), (-200.0, 20.0), (-50.0, 50.0), (100.0, 30.0),
        (250.0, 50.0), (400.0, 20.0), (550.0, 30.0), (700.0, 0.0), (850.0, -20.0)
    ];

    for i in 0..10 {
        let card_asset = format!("kenney_playing-cards-pack/PNG/Cards (medium)/{}.png", card_paths[i]);
        let card_texture = asset_server.load(&card_asset);
        let height = (card_sizes[i] as f32) * 1.5;
        commands.spawn((
            Sprite {
                image: card_texture,
                custom_size: Some(Vec2::new(card_sizes[i], height)),
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
            Transform::from_xyz(positions[i].0, positions[i].1, 0.0),
        ));
    }

    // Also spawn a card back
    commands.spawn((
        Sprite {
            image: card_back,
            custom_size: Some(Vec2::new(130.0, 195.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(900.0, 0.0, 0.0),
    ));

    println!("🂫 Displayed 11 demo cards with variable sizing!");
}
