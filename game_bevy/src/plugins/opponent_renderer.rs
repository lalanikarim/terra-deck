//! Opponent Card Renderer - displays opponent cards with fog of war

use crate::GameSessionResource;
use bevy::prelude::*;

/// Marker component for opponent cards
#[derive(Component)]
pub struct OpponentCard {}

/// Render opponent hand - cards are hidden (card back) during player's turn
pub fn render_opponent_hand(
    mut commands: Commands,
    card_assets: Res<crate::plugins::assets::CardAssets>,
    game_resource: Res<GameSessionResource>,
    existing_opponents: Query<Entity, With<OpponentCard>>,
) {
    let opponent_hand = &game_resource.session.opponent_hand;
    let card_count = opponent_hand.len();

    // Clear existing opponent cards
    for entity in existing_opponents.iter() {
        commands.entity(entity).despawn();
    }

    if card_count == 0 {
        return;
    }

    let total_width = (card_count - 1) as f32 * 90.0;
    let start_x = -total_width / 2.0;

    // Opponent cards are always hidden - show card back texture
    let card_back = &card_assets.card_back;

    for (i, _card) in opponent_hand.cards.iter().enumerate() {
        let t = if card_count > 1 {
            i as f32 / (card_count - 1) as f32
        } else {
            0.0
        };
        let x = start_x + total_width * t;

        // Opponent cards are at Y = 25 (top of screen, since origin is center)
        commands.spawn((
            bevy::prelude::Sprite {
                image: (*card_back).clone(),
                custom_size: Some(bevy::prelude::Vec2::new(80.0, 120.0)),
                ..default()
            },
            bevy::prelude::Transform::from_xyz(x, 25.0 * 1.5, 1.0),
            OpponentCard {},
        ));
    }
}

/// Resource to track which opponent card is currently being targeted
#[derive(Resource, Default)]
pub struct GameTarget {
    index: Option<usize>,
}

impl GameTarget {
    pub fn new(index: usize) -> Self {
        Self { index: Some(index) }
    }

    pub fn clear(&mut self) {
        self.index = None;
    }

    pub fn index(&self) -> Option<usize> {
        self.index
    }
}

/// Plugin for opponent rendering
pub struct OpponentRendererPlugin;

impl Plugin for OpponentRendererPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTarget>()
            .add_systems(Update, render_opponent_hand);
    }
}
