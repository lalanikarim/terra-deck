//! Game session management
//! Tracks the complete game state including hands, deck, selections, and combat state

use crate::combat::{apply_combat_damage, CombatResult};
use crate::game_loop::GameStateLoop;
use crate::{Card, Deck, Hand, CombatLog, GameResult, SelectedCard};
use rand::SeedableRng;

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

/// Complete game session state
#[derive(Clone)]
pub struct GameSession {
    pub loop_state: GameStateLoop,
    pub player_hand: Hand,
    pub opponent_hand: Hand,
    pub deck: Deck,
    pub selected_player_card: SelectedCard,
    pub selected_opponent_card: SelectedCard,
    pub combat_log: CombatLog,
    pub game_over_result: Option<GameResult>,
    pub current_combat_round: usize,
}

impl Default for GameSession {
    fn default() -> Self {
        Self::new()
    }
}

impl GameSession {
    /// Create a new empty game session
    pub fn new() -> Self {
        GameSession {
            loop_state: GameStateLoop::default(),
            player_hand: Hand::default(),
            opponent_hand: Hand::default(),
            deck: Deck::default(),
            selected_player_card: SelectedCard::none(),
            selected_opponent_card: SelectedCard::none(),
            combat_log: CombatLog::default(),
            game_over_result: None,
            current_combat_round: 0,
        }
    }

    /// Initialize a new game with full deck, shuffle, and deal
    pub fn start_new_game(&mut self) {
        self.deck = Deck::new();
        let mut rng = rand::rngs::StdRng::from_seed([0; 32]);
        self.deck.shuffle(&mut rng);

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

        self.combat_log = CombatLog::default();
        self.combat_log.add_entry(format!(
            "Game started! You have {} cards, Opponent has {} cards",
            self.player_hand.len(),
            self.opponent_hand.len()
        ));

        self.loop_state = GameStateLoop::SelectPlayerCard;
        self.selected_player_card = SelectedCard::new(0);
        self.selected_opponent_card = SelectedCard::none();
        self.game_over_result = None;
        self.current_combat_round = 0;
    }

    // ... rest unchanged
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

    pub fn get_alive_opponent_count(&self) -> usize {
        self.opponent_hand.len()
    }

    pub fn resolve_player_attack(
        &mut self,
        player_idx: usize,
        opponent_idx: usize,
    ) -> CombatResultInfo {
        let player_card = self.player_hand.cards[player_idx].clone();
        let opponent_card = self.opponent_hand.cards[opponent_idx].clone();
        let mut player_card_copy = player_card.clone();
        let mut opponent_card_copy = opponent_card.clone();
        let (player_dmg, player_result) = apply_combat_damage(&player_card, &mut opponent_card_copy);
        let (opponent_dmg, opponent_result) = apply_combat_damage(&opponent_card, &mut player_card_copy);
        let player_crit = matches!(player_result, CombatResult::CriticalHit);
        let opponent_crit = matches!(opponent_result, CombatResult::CriticalHit);
        self.player_hand.cards[player_idx] = player_card_copy.clone();
        self.opponent_hand.cards[opponent_idx] = opponent_card_copy.clone();
        let archetype_str = self.get_combat_log_entry(&player_card, &opponent_card, player_dmg, opponent_dmg, player_crit, opponent_crit);
        self.combat_log.add_entry(archetype_str);
        let player_dead = self.player_hand.remove_dead_cards();
        let opponent_dead = self.opponent_hand.remove_dead_cards();
        if player_dead > 0 { self.combat_log.add_entry(format!("Your card #{} died!", player_idx + 1)); }
        if opponent_dead > 0 { self.combat_log.add_entry(format!("Opponent's card #{} was destroyed!", opponent_idx + 1)); }
        self.selected_player_card.on_cards_removed(player_dead);
        self.selected_opponent_card.on_cards_removed(opponent_dead);
        let result = CombatResultInfo { player_dmg, opponent_dmg, player_crit, opponent_crit, player_dead, opponent_dead };
        self.check_game_over();
        result
    }

    fn get_combat_log_entry(&self, player_card: &Card, opponent_card: &Card, player_dmg: u8, _opponent_dmg: u8, player_crit: bool, _opponent_crit: bool) -> String {
        let crit_prefix = if player_crit { "CRITICAL! " } else { "" };
        let player_arch = player_card.suit.archetype();
        let opponent_arch = opponent_card.suit.archetype();
        format!("{}{} (vs {}) dealt {} damage", crit_prefix, player_arch, opponent_arch, player_dmg)
    }

    pub fn resolve_opponent_attack(&mut self, opponent_idx: usize, player_idx: usize) {
        let _result = self.resolve_player_attack(opponent_idx, player_idx);
        self.check_game_over();
    }

    pub fn is_player_turn(&self) -> bool { self.loop_state.is_player_turn() }
    pub fn is_opponent_turn(&self) -> bool { self.loop_state.is_opponent_turn() }
    pub fn is_resolving(&self) -> bool { self.loop_state.is_resolving() }
    pub fn is_game_over(&self) -> bool { self.loop_state.is_game_over() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new_game_session() {
        let session = GameSession::new();
        assert_eq!(session.loop_state, GameStateLoop::Start);
    }
    #[test] fn test_start_new_game() {
        let mut session = GameSession::new();
        session.start_new_game();
        assert_eq!(session.deck.len(), 42);
    }
    #[test] fn test_combat_resolution() {
        let mut session = GameSession::new();
        session.start_new_game();
        let result = session.resolve_player_attack(0, 0);
        assert!(result.player_dmg > 0 || result.player_crit);
    }
}
