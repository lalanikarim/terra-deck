use bevy::prelude::*;
use game_bevy::plugins::{CardAssetPlugin, GameSessionPlugin, GameSessionResource};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D - Player Hand Render".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CardAssetPlugin)
        .add_plugins(GameSessionPlugin)
        .add_systems(Startup, (init_game_session, setup))
        .run();
}

fn init_game_session(mut resource: ResMut<'_, GameSessionResource>) {
    resource.session.start_new_game();
    println!(
        "🎮 Game started! {} player cards, {} opponent cards",
        resource.session.player_hand.len(),
        resource.session.opponent_hand.len()
    );
}

fn setup(
    mut commands: Commands,
    asset_server: Res<'_, AssetServer>,
    cameras: Query<Entity, With<Camera2d>>,
    game_resource: Res<'_, GameSessionResource>,
) {
    println!("🃏 Rendering player hand from game_core...");

    // Kill default 3D camera
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    println!("📊 Game State: {}", game_resource.session.loop_state);
    println!(
        "📊 Player hand: {} cards, Opponent hand: {} cards",
        game_resource.session.player_hand.len(),
        game_resource.session.opponent_hand.len()
    );

    // Display actual player hand cards
    let card_size = 100.0;
    for (i, card) in game_resource.session.player_hand.cards.iter().enumerate() {
        let suit = match card.suit {
            game_core::Suit::Hearts => "card_hearts",
            game_core::Suit::Diamonds => "card_diamonds",
            game_core::Suit::Clubs => "card_clubs",
            game_core::Suit::Spades => "card_spades",
        };

        let rank = match card.rank {
            game_core::Rank::Two => "02",
            game_core::Rank::Three => "03",
            game_core::Rank::Four => "04",
            game_core::Rank::Five => "05",
            game_core::Rank::Six => "06",
            game_core::Rank::Seven => "07",
            game_core::Rank::Eight => "08",
            game_core::Rank::Nine => "09",
            game_core::Rank::Ten => "10",
            game_core::Rank::Jack => "J",
            game_core::Rank::Queen => "Q",
            game_core::Rank::King => "K",
            game_core::Rank::Ace => "A",
        };

        let path = format!(
            "kenney_playing-cards-pack/PNG/Cards (medium)/{}_{}.png",
            suit, rank
        );

        println!(
            "  🃏 Card {}: {}{}",
            i + 1,
            match card.suit {
                game_core::Suit::Hearts => "♥️",
                game_core::Suit::Diamonds => "♦️",
                game_core::Suit::Clubs => "♣️",
                game_core::Suit::Spades => "♠️",
            },
            match card.rank {
                game_core::Rank::Two => "2",
                game_core::Rank::Three => "3",
                game_core::Rank::Four => "4",
                game_core::Rank::Five => "5",
                game_core::Rank::Six => "6",
                game_core::Rank::Seven => "7",
                game_core::Rank::Eight => "8",
                game_core::Rank::Nine => "9",
                game_core::Rank::Ten => "10",
                game_core::Rank::Jack => "J",
                game_core::Rank::Queen => "Q",
                game_core::Rank::King => "K",
                game_core::Rank::Ace => "A",
            }
        );

        commands.spawn((
            Sprite {
                image: asset_server.load(&path),
                custom_size: Some(Vec2::new(card_size, card_size * 1.5)),
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
            Transform::from_xyz((i as f32 - 2.0) * 70.0, 0.0, 0.0),
        ));
    }

    println!("🎴 Hand rendering complete!");
}
