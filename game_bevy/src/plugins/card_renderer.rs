//! Card Renderer - displays cards with fan layout

use crate::GameSessionResource;
use bevy::prelude::*;

/// Card renderer component
#[derive(Component)]
pub struct CardRenderer;

/// Display player hand with fan layout
pub fn render_player_hand(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_resource: Res<GameSessionResource>,
) {
    let player_hand = &game_resource.session.player_hand;
    let card_count = player_hand.len() as f32;

    // Fan spread parameters
    let total_width = if card_count > 0.0 {
        (card_count - 1.0) * 90.0
    } else {
        0.0
    };
    let start_x = -total_width / 2.0;

    for (index, card) in player_hand.cards.iter().enumerate() {
        // Map suit to path
        let suit_path = match card.suit {
            game_core::Suit::Hearts => "card_hearts",
            game_core::Suit::Diamonds => "card_diamonds",
            game_core::Suit::Clubs => "card_clubs",
            game_core::Suit::Spades => "card_spades",
        };

        // Map rank to path
        let rank_path = match card.rank {
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

        let card_texture = asset_server.load(format!(
            "kenney_playing-cards-pack/PNG/Cards (large)/{}_{}.png",
            suit_path, rank_path
        ));

        // Calculate position for fan layout
        let t = if card_count > 1.0 {
            index as f32 / (card_count - 1.0)
        } else {
            0.0
        };
        let x = start_x + total_width * t;

        // Spawn the card
        commands.spawn((
            Sprite {
                image: card_texture,
                custom_size: Some(Vec2::new(100.0, 150.0)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, 0.0),
            CardRenderer,
        ));
    }
}
