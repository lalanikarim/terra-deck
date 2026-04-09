//! Game loop state machine
//! Manages the state transitions during gameplay

use std::fmt::Display;

/// Current state of the game loop
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GameStateLoop {
    #[default]
    Start,
    SelectPlayerCard,
    SelectOpponentTarget,
    ConfirmAttack,
    ResolvingCombat,
    WaitingForOpponent,
    OpponentSelectingTarget,
    OpponentAttackResolving,
    GameOver,
    Quit,
}

impl GameStateLoop {
    /// Check if it's the player's turn to make decisions
    pub fn is_player_turn(&self) -> bool {
        matches!(
            self,
            GameStateLoop::SelectPlayerCard
                | GameStateLoop::SelectOpponentTarget
                | GameStateLoop::ConfirmAttack
        )
    }

    /// Check if it's the opponent's turn (AI acting)
    pub fn is_opponent_turn(&self) -> bool {
        matches!(
            self,
            GameStateLoop::WaitingForOpponent
                | GameStateLoop::OpponentSelectingTarget
                | GameStateLoop::OpponentAttackResolving
        )
    }

    /// Check if combat is currently resolving
    pub fn is_resolving(&self) -> bool {
        matches!(
            self,
            GameStateLoop::ResolvingCombat | GameStateLoop::OpponentAttackResolving
        )
    }

    /// Check if the game has ended
    pub fn is_game_over(&self) -> bool {
        matches!(self, GameStateLoop::GameOver)
    }

    /// Advance from SelectPlayerCard to SelectOpponentTarget
    pub fn advance_after_player_card_selected(&self) -> Self {
        if *self == GameStateLoop::SelectPlayerCard {
            GameStateLoop::SelectOpponentTarget
        } else {
            self.clone()
        }
    }

    /// Advance from SelectOpponentTarget to ConfirmAttack
    pub fn advance_after_target_selected(&self) -> Self {
        if *self == GameStateLoop::SelectOpponentTarget {
            GameStateLoop::ConfirmAttack
        } else {
            self.clone()
        }
    }

    /// Go back from target selection to player card selection
    pub fn cancel_target_selection(&self) -> Self {
        if *self == GameStateLoop::SelectOpponentTarget || *self == GameStateLoop::ConfirmAttack {
            GameStateLoop::SelectPlayerCard
        } else {
            self.clone()
        }
    }

    /// Reset to start of player turn after combat
    pub fn reset_to_player_turn(&self) -> Self {
        GameStateLoop::SelectPlayerCard
    }
}

impl Display for GameStateLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameStateLoop::Start => "Game Start",
                GameStateLoop::SelectPlayerCard => "Select Your Card",
                GameStateLoop::SelectOpponentTarget => "Select Target",
                GameStateLoop::ConfirmAttack => "Confirm Attack",
                GameStateLoop::ResolvingCombat => "Resolving Combat...",
                GameStateLoop::WaitingForOpponent => "Opponent's Turn...",
                GameStateLoop::OpponentSelectingTarget => "Opponent Selecting...",
                GameStateLoop::OpponentAttackResolving => "Opponent Attacking...",
                GameStateLoop::GameOver => "Game Over",
                GameStateLoop::Quit => "Quitting",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = GameStateLoop::default();
        assert_eq!(state, GameStateLoop::Start);
    }

    #[test]
    fn test_is_player_turn() {
        assert!(GameStateLoop::SelectPlayerCard.is_player_turn());
        assert!(GameStateLoop::SelectOpponentTarget.is_player_turn());
        assert!(GameStateLoop::ConfirmAttack.is_player_turn());
        assert!(!GameStateLoop::WaitingForOpponent.is_player_turn());
        assert!(!GameStateLoop::GameOver.is_player_turn());
    }

    #[test]
    fn test_is_opponent_turn() {
        assert!(!GameStateLoop::SelectPlayerCard.is_opponent_turn());
        assert!(GameStateLoop::WaitingForOpponent.is_opponent_turn());
        assert!(GameStateLoop::OpponentSelectingTarget.is_opponent_turn());
        assert!(!GameStateLoop::GameOver.is_opponent_turn());
    }

    #[test]
    fn test_is_resolving() {
        assert!(GameStateLoop::ResolvingCombat.is_resolving());
        assert!(GameStateLoop::OpponentAttackResolving.is_resolving());
        assert!(!GameStateLoop::SelectPlayerCard.is_resolving());
    }

    #[test]
    fn test_is_game_over() {
        assert!(GameStateLoop::GameOver.is_game_over());
        assert!(!GameStateLoop::SelectPlayerCard.is_game_over());
    }

    #[test]
    fn test_advance_after_player_card_selected() {
        let state = GameStateLoop::SelectPlayerCard;
        let next = state.advance_after_player_card_selected();
        assert_eq!(next, GameStateLoop::SelectOpponentTarget);
    }

    #[test]
    fn test_advance_after_target_selected() {
        let state = GameStateLoop::SelectOpponentTarget;
        let next = state.advance_after_target_selected();
        assert_eq!(next, GameStateLoop::ConfirmAttack);
    }

    #[test]
    fn test_cancel_target_selection() {
        let state = GameStateLoop::ConfirmAttack;
        let next = state.cancel_target_selection();
        assert_eq!(next, GameStateLoop::SelectPlayerCard);
    }

    #[test]
    fn test_reset_to_player_turn() {
        let states = vec![
            GameStateLoop::ResolvingCombat,
            GameStateLoop::GameOver,
            GameStateLoop::WaitingForOpponent,
        ];
        for state in states {
            assert_eq!(
                state.reset_to_player_turn(),
                GameStateLoop::SelectPlayerCard
            );
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", GameStateLoop::SelectPlayerCard),
            "Select Your Card"
        );
        assert_eq!(format!("{}", GameStateLoop::GameOver), "Game Over");
    }
}
