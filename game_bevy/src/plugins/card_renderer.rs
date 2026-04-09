//! Card Renderer - displays player cards with fan layout

use crate::plugins::assets::{CardAssets, Rank, Suit};
use crate::GameSessionResource;
use bevy::prelude::*;

/// Component marker for player cards
#[derive(Component)]
pub struct PlayerCard;

/// Render player hand using cached CardAssets
pub fn render_player_hand(
    mut commands: Commands,
    card_assets: Res<CardAssets>,
    game_resource: Res<GameSessionResource>,
) {
    let player_hand = &game_resource.session.player_hand;
    let card_count = player_hand.len() as f32;

    if card_count == 0.0 {
        return;
    }

    // Fan spread parameters
    let total_width = (card_count - 1.0) * 90.0;
    let start_x = -total_width / 2.0;

    for (index, card) in player_hand.cards.iter().enumerate() {
        // Map to cached asset coordinates
        let asset_suit = match card.suit {
            game_core::Suit::Hearts => Suit::Hearts,
            game_core::Suit::Diamonds => Suit::Diamonds,
            game_core::Suit::Clubs => Suit::Clubs,
            game_core::Suit::Spades => Suit::Spades,
        };

        let asset_rank = match card.rank {
            game_core::Rank::Two => Rank::Two,
            game_core::Rank::Three => Rank::Three,
            game_core::Rank::Four => Rank::Four,
            game_core::Rank::Five => Rank::Five,
            game_core::Rank::Six => Rank::Six,
            game_core::Rank::Seven => Rank::Seven,
            game_core::Rank::Eight => Rank::Eight,
            game_core::Rank::Nine => Rank::Nine,
            game_core::Rank::Ten => Rank::Ten,
            game_core::Rank::Jack => Rank::Jack,
            game_core::Rank::Queen => Rank::Queen,
            game_core::Rank::King => Rank::King,
            game_core::Rank::Ace => Rank::Ace,
        };

        // Get texture from cached assets
        if let Some(texture_handle) = card_assets.get_card(asset_suit, asset_rank) {
            // Player cards at bottom of screen, 150 units down from center
            let t = if card_count > 1.0 {
                index as f32 / (card_count - 1.0)
            } else {
                0.0
            };
            let x = start_x + total_width * t;

            commands.spawn((
                Sprite {
                    image: (*texture_handle).clone(),
                    custom_size: Some(Vec2::new(100.0, 150.0)),
                    ..default()
                },
                Transform::from_xyz(x, -150.0, 1.0),
                PlayerCard,
            ));
        }
    }
}

/// Plugin for player card rendering
pub struct PlayerRendererPlugin;

impl Plugin for PlayerRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_player_hand);
    }
}
