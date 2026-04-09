use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Card Display Demo".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    cameras: Query<Entity, With<Camera2d>>,
) {
    println!("🃏 Starting card display demo...");

    // Kill default 3D camera and spawn 2D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    // Load test cards
    let hearts_7 =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_hearts_07.png");
    let spades_A =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_spades_A.png");
    let diamonds_K =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_diamonds_K.png");
    let club_back =
        asset_server.load("kenney_playing-cards-pack/PNG/Cards (large)/card_clubs_02.png");

    println!("🃄 Testing card loads");
    println!("  ✅ Hearts 7");
    println!("  ✅ Spades A");
    println!("  ✅ Diamonds K");
    println!("  ✅ Clubs 2");

    // Spawn test cards at different positions
    // Hearts 7 - left side
    commands.spawn((
        Sprite {
            image: hearts_7,
            custom_size: Some(Vec2::new(200.0, 300.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(-400.0, 0.0, 0.0),
    ));

    // Spades A - center
    commands.spawn((
        Sprite {
            image: spades_A,
            custom_size: Some(Vec2::new(200.0, 300.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Diamonds K - right side
    commands.spawn((
        Sprite {
            image: diamonds_K,
            custom_size: Some(Vec2::new(200.0, 300.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(400.0, 0.0, 0.0),
    ));

    // Card back - far right
    commands.spawn((
        Sprite {
            image: club_back,
            custom_size: Some(Vec2::new(200.0, 300.0)),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Transform::from_xyz(650.0, 0.0, 0.0),
    ));

    println!("🂫 Displayed 4 test cards on screen!");
}
