use bevy::prelude::*;
use game_bevy::plugins::{
    render_opponent_hand, CardAssetPlugin, GameSessionPlugin, GameSessionResource,
    OpponentRendererPlugin,
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
        .add_plugins(CardAssetPlugin)
        .add_plugins(GameSessionPlugin)
        .add_plugins(OpponentRendererPlugin)
        .add_systems(Startup, initialize)
        .add_systems(Update, (render_hand, render_opponent_hand))
        .run();
}

fn initialize(mut commands: Commands, cameras: Query<Entity, With<Camera2d>>) {
    for camera in cameras.iter() {
        commands.entity(camera).despawn();
    }
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));
}

fn render_hand(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_resource: Res<GameSessionResource>,
) {
    let hand = &game_resource.session.player_hand;
    if hand.len() == 0 {
        return;
    }

    let total_width = (hand.len() - 1) as f32 * 90.0;
    let start_x = -total_width / 2.0;

    for (i, card) in hand.cards.iter().enumerate() {
        let t = i as f32 / (hand.len() - 1) as f32;

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

        let texture = asset_server.load(format!(
            "kenney_playing-cards-pack/PNG/Cards (large)/{}_{}.png",
            suit, rank
        ));

        // Player cards at bottom of screen. Card is 150 tall (centered at -0.5)
        let x = start_x + t * total_width;
        commands.spawn((
            Sprite {
                image: texture,
                custom_size: Some(Vec2::new(100.0, 150.0)),
                ..default()
            },
            Transform::from_xyz(x, -0.75, 1.0),
        ));
    }
}
