//! Combat statistics and game state
use bevy::prelude::Resource;

/// Result of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameResult {
    Won,
    Lost,
    Draw,
}

/// Current state of the game
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerTurn,
    OpponentTurn,
    CombatResolution,
    GameOver(GameResult),
}

impl GameState {
    /// Returns true if the game is active (not over)
    pub fn is_active(&self) -> bool {
        !matches!(self, GameState::GameOver(_))
    }

    /// Returns true if it's player's turn to act
    pub fn is_player_turn(&self) -> bool {
        matches!(self, GameState::PlayerTurn)
    }

    /// Returns true if it's opponent's turn to act
    pub fn is_opponent_turn(&self) -> bool {
        matches!(self, GameState::OpponentTurn)
    }

    /// Returns true if combat should be resolved
    pub fn should_resolve_combat(&self) -> bool {
        matches!(self, GameState::CombatResolution)
    }

    /// Resets the game state to player's turn
    pub fn reset() -> Self {
        GameState::PlayerTurn
    }

    /// Advances to the next logical state based on game flow
    /// Current simplified rules:
    /// - PlayerTurn -> CombatResolution (player selected a card)
    /// - OpponentTurn -> CombatResolution (opponent selected a card)
    /// - CombatResolution -> PlayerTurn (round completed)
    pub fn advance_from_combat() -> Self {
        GameState::PlayerTurn
    }

    /// Creates a game over state from the result
    pub fn into_game_over(result: GameResult) -> Self {
        GameState::GameOver(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === GameResult tests ===

    #[test]
    fn test_game_result_variants_exist() {
        let won = GameResult::Won;
        let lost = GameResult::Lost;
        let draw = GameResult::Draw;
        
        assert_ne!(won, lost);
        assert_ne!(won, draw);
        assert_ne!(lost, draw);
    }

    // === GameState tests ===

    #[test]
    fn test_game_state_default_is_player_turn() {
        let state = GameState::default();
        assert_eq!(state, GameState::PlayerTurn);
    }

    #[test]
    fn test_game_state_is_active() {
        assert!(GameState::PlayerTurn.is_active());
        assert!(GameState::OpponentTurn.is_active());
        assert!(GameState::CombatResolution.is_active());
        assert!(!GameState::GameOver(GameResult::Won).is_active());
        assert!(!GameState::GameOver(GameResult::Lost).is_active());
        assert!(!GameState::GameOver(GameResult::Draw).is_active());
    }

    #[test]
    fn test_game_state_is_player_turn() {
        assert!(GameState::PlayerTurn.is_player_turn());
        assert!(!GameState::OpponentTurn.is_player_turn());
        assert!(!GameState::CombatResolution.is_player_turn());
        assert!(!GameState::GameOver(GameResult::Won).is_player_turn());
    }

    #[test]
    fn test_game_state_is_opponent_turn() {
        assert!(GameState::OpponentTurn.is_opponent_turn());
        assert!(!GameState::PlayerTurn.is_opponent_turn());
        assert!(!GameState::CombatResolution.is_opponent_turn());
        assert!(!GameState::GameOver(GameResult::Won).is_opponent_turn());
    }

    #[test]
    fn test_game_state_should_resolve_combat() {
        assert!(GameState::CombatResolution.should_resolve_combat());
        assert!(!GameState::PlayerTurn.should_resolve_combat());
        assert!(!GameState::OpponentTurn.should_resolve_combat());
        assert!(!GameState::GameOver(GameResult::Won).should_resolve_combat());
    }

    #[test]
    fn test_game_state_reset() {
        let state = GameState::reset();
        assert_eq!(state, GameState::PlayerTurn);
    }

    #[test]
    fn test_game_state_advance_from_combat() {
        let state = GameState::advance_from_combat();
        assert_eq!(state, GameState::PlayerTurn);
    }

    #[test]
    fn test_game_state_into_game_over() {
        let won_state = GameState::into_game_over(GameResult::Won);
        let lost_state = GameState::into_game_over(GameResult::Lost);
        let draw_state = GameState::into_game_over(GameResult::Draw);

        assert_eq!(won_state, GameState::GameOver(GameResult::Won));
        assert_eq!(lost_state, GameState::GameOver(GameResult::Lost));
        assert_eq!(draw_state, GameState::GameOver(GameResult::Draw));
    }
}

/// Combat statistics for a turn
#[derive(Resource, Default)]
pub struct CombatStats {
    pub player_damage_dealt: u8,
    pub opponent_damage_dealt: u8,
    pub player_crits: u8,
    pub opponent_crits: u8,
    pub player_absorbs: u8,
    pub opponent_absorbs: u8,
}
