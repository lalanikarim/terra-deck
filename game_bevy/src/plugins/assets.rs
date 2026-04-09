//! Card Asset Loader - Pre-caches all Kenney card sprites

use bevy::prelude::*;
use std::collections::HashMap;

/// Suit enum - card suits
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

/// Rank enum - card ranks  
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Resource holding all pre-loaded card textures
#[derive(Resource, Default)]
pub struct CardAssets {
    /// Map of card handles: (Suit, Rank) -> Handle<Image>
    pub cards: HashMap<(Suit, Rank), Handle<Image>>,
    /// Handle to card back texture
    pub card_back: Handle<Image>,
}

impl CardAssets {
    /// Get card handle by suit and rank
    pub fn get_card(&self, suit: Suit, rank: Rank) -> Option<&Handle<Image>> {
        self.cards.get(&(suit, rank))
    }

    /// Get card back handle
    pub fn get_card_back(&self) -> &Handle<Image> {
        &self.card_back
    }
}

/// Asset loading system - runs at Startup to cache all card textures
pub struct CardAssetPlugin;

impl Plugin for CardAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardAssets>()
            .add_systems(Startup, load_card_assets);
    }
}

fn load_card_assets(asset_server: Res<AssetServer>, mut card_assets: ResMut<CardAssets>) {
    println!("🎴 Loading Kenney playing card assets...");

    // Load card back
    let card_back_path = "kenney_playing-cards-pack/PNG/Cards (large)/card_back.png";
    card_assets.card_back = asset_server.load(card_back_path);
    println!("  ✓ Loaded: {}", card_back_path);

    // Load all card ranks (2-10, J, Q, K, A)
    let ranks: Vec<Rank> = vec![
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    // Load all suits
    let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

    // Map suit symbols to Kenney filenames
    let suit_paths = |suit: &Suit| -> &'static str {
        match suit {
            Suit::Hearts => "card_hearts",
            Suit::Diamonds => "card_diamonds",
            Suit::Clubs => "card_clubs",
            Suit::Spades => "card_spades",
        }
    };

    // Map rank to Kenney filename suffix
    let rank_path = |rank: &Rank| -> &'static str {
        match rank {
            Rank::Two => "02",
            Rank::Three => "03",
            Rank::Four => "04",
            Rank::Five => "05",
            Rank::Six => "06",
            Rank::Seven => "07",
            Rank::Eight => "08",
            Rank::Nine => "09",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    };

    // Load all 52 cards
    for suit in &suits {
        for rank in &ranks {
            let suit_prefix = suit_paths(suit);
            let rank_suffix = rank_path(rank);
            let card_path = format!(
                "kenney_playing-cards-pack/PNG/Cards (large)/{}_{}.png",
                suit_prefix, rank_suffix
            );

            card_assets
                .cards
                .insert((*suit, *rank), asset_server.load(card_path));
            println!("  ✓ Loaded: {}_{}.png", suit_prefix, rank_suffix);
        }
    }

    println!(
        "🎴 Asset loading complete! {} cards cached.",
        card_assets.cards.len()
    );
}
