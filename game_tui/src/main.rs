//! Poker Card RPG - TUI Application
//!
//! This is the entry point for the terminal user interface using Bevy.
//! Currently sets up the Bevy app with game_core resources.

use bevy::prelude::*;
use game_core::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .init_resource::<CombatStats>()
        .init_resource::<CombatLog>()
        .init_resource::<SelectedCard>()
        .init_resource::<Deck>()
        // Initialize player and opponent hands
        .insert_resource(Hand::default())
        .insert_resource(Hand::default())
        .add_systems(Startup, init_game)
        .run();
}

/// Initialize the game state
fn init_game(_commands: Commands) {
    // Initialize with default values
    // Deck will be created, shuffled, and hands dealt
    // For now, just log startup message
    info!("Poker Card RPG - Terra-Deck");
    info!("All systems initialized and ready!");
    info!("Next: Implement Tasks 4-6 (TUI rendering, input, testing)");
}
