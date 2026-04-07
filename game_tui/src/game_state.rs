//! Game state management for the full combat loop

use game_core::*;
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
    Quit,
    GameOver,
}

impl GameStateLoop {
    pub fn is_player_turn(&self) -> bool {
        matches!(
            self,
            GameStateLoop::SelectPlayerCard
                | GameStateLoop::SelectOpponentTarget
                | GameStateLoop::ConfirmAttack
        )
    }

    pub fn is_opponent_turn(&self) -> bool {
        matches!(
            self,
            GameStateLoop::WaitingForOpponent
                | GameStateLoop::OpponentSelectingTarget
                | GameStateLoop::OpponentAttackResolving
        )
    }

    pub fn is_resolving(&self) -> bool {
        matches!(
            self,
            GameStateLoop::ResolvingCombat | GameStateLoop::OpponentAttackResolving
        )
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

/// Full game state including hands, selection, and combat state
#[derive(Default, Clone)]
pub struct FullGameState {
    pub loop_state: GameStateLoop,
    pub player_hand: Hand,
    pub opponent_hand: Hand,
    pub deck: Deck,
    pub selected_player_card: Option<usize>,
    pub selected_opponent_card: Option<usize>,
    pub combat_log: CombatLog,
    pub game_over_result: Option<GameResult>,
    pub current_combat_round: usize,
}

impl FullGameState {
    pub fn new() -> Self {
        FullGameState {
            loop_state: GameStateLoop::Start,
            ..Default::default()
        }
    }

    /// Check if it's player turn
    pub fn is_player_turn(&self) -> bool {
        self.loop_state.is_player_turn()
    }

    /// Check if it's opponent turn
    pub fn is_opponent_turn(&self) -> bool {
        self.loop_state.is_opponent_turn()
    }

    /// Check if combat is resolving
    pub fn is_resolving(&self) -> bool {
        self.loop_state.is_resolving()
    }

    /// Initialize a new game with full deck, shuffle, and deal
    pub fn start_new_game(&mut self) {
        self.deck = Deck::new();
        self.deck.shuffle(&mut rand::rng());

        // Deal 5 cards each
        self.player_hand = Hand::default();
        self.opponent_hand = Hand::default();

        for _ in 0..5 {
            if let Some(card) = self.deck.draw() {
                self.player_hand.add_card(card);
            }
            if let Some(card) = self.deck.draw() {
                self.opponent_hand.add_card(card);
            }
        }

        self.combat_log.add_entry(format!(
            "Game started! You have {} cards, Opponent has {} cards",
            self.player_hand.len(),
            self.opponent_hand.len()
        ));

        self.loop_state = GameStateLoop::SelectPlayerCard;
        self.selected_player_card = Some(0);
        self.selected_opponent_card = None;
        self.game_over_result = None;
        self.current_combat_round = 0;
    }

    /// Check if game should end (either hand empty)
    pub fn check_game_over(&mut self) {
        if self.player_hand.is_empty() {
            self.loop_state = GameStateLoop::GameOver;
            self.game_over_result = Some(GameResult::Lost);
            self.combat_log.add_entry("=== YOU LOST ===".to_string());
        } else if self.opponent_hand.is_empty() {
            self.loop_state = GameStateLoop::GameOver;
            self.game_over_result = Some(GameResult::Won);
            self.combat_log.add_entry("=== YOU WON! ===".to_string());
        }
    }

    /// Get the number of alive opponent cards (for display)
    pub fn get_alive_opponent_count(&self) -> usize {
        self.opponent_hand.len()
    }

    /// Resolve combat between player and opponent cards
    pub fn resolve_player_attack(
        &mut self,
        player_idx: usize,
        opponent_idx: usize,
    ) -> CombatResultInfo {
        // Get cards
        let player_card = self.player_hand.cards[player_idx].clone();
        let opponent_card = self.opponent_hand.cards[opponent_idx].clone();

        // Clone for damage calculation
        let mut player_card_copy = player_card.clone();
        let mut opponent_card_copy = opponent_card.clone();

        // Apply damage
        let (player_dmg, player_result) = apply_combat_damage(&player_card, &mut opponent_card_copy);
        let (opponent_dmg, opponent_result) = apply_combat_damage(&opponent_card, &mut player_card_copy);
        let player_crit = matches!(player_result, CombatResult::CriticalHit);
        let opponent_crit = matches!(opponent_result, CombatResult::CriticalHit);

        // Update cards in hands
        self.player_hand.cards[player_idx] = player_card_copy.clone();
        self.opponent_hand.cards[opponent_idx] = opponent_card_copy.clone();

        // Log the combat
        let archetype_str = self.get_combat_log_entry(
            &player_card,
            &opponent_card,
            player_dmg,
            opponent_dmg,
            player_crit,
            opponent_crit,
        );

        self.combat_log.add_entry(archetype_str);

        // Remove dead cards
        let player_dead = self.player_hand.remove_dead_cards();
        let opponent_dead = self.opponent_hand.remove_dead_cards();

        // Log dead cards
        if player_dead > 0 {
            self.combat_log.add_entry(format!("Your card #{} died!", player_idx + 1));
        }
        if opponent_dead > 0 {
            self.combat_log.add_entry(format!("Opponent's card #{} was destroyed!", opponent_idx + 1));
        }

        // Update selection after dead card removal
        if let Some(idx) = self.selected_player_card {
            if idx >= self.player_hand.len() {
                self.selected_player_card = Some(self.player_hand.len().saturating_sub(1));
            }
        }

        let result = CombatResultInfo {
            player_dmg,
            opponent_dmg,
            player_crit,
            opponent_crit,
            player_dead,
            opponent_dead,
        };

        self.check_game_over();

        result
    }

    /// Get a combat log entry string
    fn get_combat_log_entry(
        &self,
        player_card: &Card,
        opponent_card: &Card,
        player_dmg: u8,
        _opponent_dmg: u8,
        player_crit: bool,
        _opponent_crit: bool,
    ) -> String {
        let player_arch = player_card.suit.archetype();
        let opponent_arch = opponent_card.suit.archetype();

        let crit_prefix = if player_crit { "CRITICAL! " } else { "" };

        format!(
            "{}{} (vs {}) dealt {} damage",
            crit_prefix,
            player_arch,
            opponent_arch,
            player_dmg
        )
    }

    /// Play opponent's turn using AI
    pub fn resolve_opponent_attack(&mut self, opponent_idx: usize, player_idx: usize) {
        self.resolve_player_attack(opponent_idx, player_idx);
        self.check_game_over();
    }
}

/// Result information after combat resolution
#[derive(Debug, Clone)]
pub struct CombatResultInfo {
    pub player_dmg: u8,
    pub opponent_dmg: u8,
    pub player_crit: bool,
    pub opponent_crit: bool,
    pub player_dead: usize,
    pub opponent_dead: usize,
}
