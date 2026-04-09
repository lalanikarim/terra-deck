//! Game Session Bridge - connects game_core to Bevy ECS

use bevy::prelude::*;

/// Bridge resource: wraps the game_core GameSession
/// This is the "Source of Truth" that game_bevy observes
#[derive(Resource)]
pub struct GameSessionResource {
    /// The actual game_core GameSession
    pub session: game_core::GameSession,
    /// Flag to track if we need to spawn initial cards
    pub initial_spawn_done: bool,
}

impl GameSessionResource {
    pub fn new() -> Self {
        Self {
            session: game_core::GameSession::new(),
            initial_spawn_done: false,
        }
    }

    /// Start a new game and deal cards
    pub fn start_new_game(&mut self) {
        self.session.start_new_game();
        self.initial_spawn_done = false;
    }

    /// Check if player can select a card
    pub fn can_player_select(&self) -> bool {
        matches!(
            self.session.loop_state,
            game_core::game_loop::GameStateLoop::SelectPlayerCard
        )
    }

    /// Check if game is over
    pub fn game_over(&self) -> bool {
        self.session.is_game_over()
    }

    /// Get current game result (won/lost)
    pub fn game_result(&self) -> Option<&game_core::GameResult> {
        self.session.game_over_result.as_ref()
    }
}

impl Default for GameSessionResource {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin that initializes the GameSessionResource
pub struct GameSessionPlugin;

impl Plugin for GameSessionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSessionResource>()
            .add_systems(Startup, init_game_session);
    }
}

/// System to initialize the game session on startup
fn init_game_session(mut resource: ResMut<GameSessionResource>) {
    println!("🎮 GameSessionResource initialized");
    resource.session.start_new_game();
    println!(
        "🎮 New game started! {} player cards, {} opponent cards",
        resource.session.player_hand.len(),
        resource.session.opponent_hand.len()
    );
}
