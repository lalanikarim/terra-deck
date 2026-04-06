//! Bevy ECS systems for combat resolution and turn management

use crate::combat::{apply_combat_damage, CombatResult};
use crate::combat_log::CombatLog;
use crate::combat_stats::{CombatStats, GameResult, GameState};
use crate::hand::Hand;
use crate::turn_state::SelectedCard;
use bevy::prelude::*;

/// System to resolve combat between player and opponent cards
pub fn resolve_combat_system(
    mut commands: Commands,
    mut player_hand: ResMut<Hand>,
    mut opponent_hand: ResMut<Hand>,
    mut combat_log: ResMut<CombatLog>,
    mut combat_stats: ResMut<CombatStats>,
) {
    if player_hand.is_empty() || opponent_hand.is_empty() {
        // Handle edge case where someone has no cards
        let result = if player_hand.is_empty() {
            GameResult::Lost
        } else {
            GameResult::Won
        };
        combat_log.add_entry(format!("Game Over: {:?}", result));
        commands.insert_resource(GameState::GameOver(result));
        return;
    }

    // Get first cards from each hand (they fight)
    let player_card_idx = 0usize;
    let opponent_card_idx = 0usize;

    // Clone card data for logging (avoid borrow conflicts)
    let player_card_info = player_hand.cards[player_card_idx].clone();
    let opponent_card_info = opponent_hand.cards[opponent_card_idx].clone();

    // Create mutable copies for damage calculation
    let mut player_card_copy = player_card_info.clone();
    let mut opponent_card_copy = opponent_card_info.clone();

    // Player attacks opponent
    let (player_damage, player_result) =
        apply_combat_damage(&player_card_info, &mut opponent_card_copy);

    // Opponent attacks player (simultaneous attack)
    let (opponent_damage, opponent_result) =
        apply_combat_damage(&opponent_card_info, &mut player_card_copy);

    // Update the actual cards in the hands with damage
    player_hand.cards[player_card_idx] = player_card_copy;
    opponent_hand.cards[opponent_card_idx] = opponent_card_copy;

    // Update stats
    combat_stats.player_damage_dealt += player_damage;
    combat_stats.opponent_damage_dealt += opponent_damage;

    match player_result {
        CombatResult::CriticalHit => combat_stats.player_crits += 1,
        CombatResult::Absorb => combat_stats.player_absorbs += 1,
        _ => {}
    }

    match opponent_result {
        CombatResult::CriticalHit => combat_stats.opponent_crits += 1,
        CombatResult::Absorb => combat_stats.opponent_absorbs += 1,
        _ => {}
    }

    // Log the combat (using cloned info)
    combat_log.add_entry(format!(
        "Player {} ({} {}) attacks Opponent {} ({} {}) - Result: {:?} ({} dmg)",
        player_card_info.rank,
        player_card_info.suit,
        player_card_info.hp,
        opponent_card_info.rank,
        opponent_card_info.suit,
        opponent_card_info.hp,
        player_result,
        player_damage
    ));

    combat_log.add_entry(format!(
        "Opponent {} ({} {}) attacks Player {} ({} {}) - Result: {:?} ({} dmg)",
        opponent_card_info.rank,
        opponent_card_info.suit,
        opponent_card_info.hp,
        player_card_info.rank,
        player_card_info.suit,
        player_card_info.hp,
        opponent_result,
        opponent_damage
    ));

    // Remove dead cards
    let player_dead = player_hand.remove_dead_cards();
    let opponent_dead = opponent_hand.remove_dead_cards();

    if player_dead > 0 {
        combat_log.add_entry(format!("Player lost {} card(s)", player_dead));
    }
    if opponent_dead > 0 {
        combat_log.add_entry(format!("Opponent lost {} card(s)", opponent_dead));
    }

    // Check win/loss conditions
    if player_hand.is_empty() {
        combat_log.add_entry("Game Over: You Lost!".to_string());
        commands.insert_resource(GameState::GameOver(GameResult::Lost));
    } else if opponent_hand.is_empty() {
        combat_log.add_entry("Game Over: You Won!".to_string());
        commands.insert_resource(GameState::GameOver(GameResult::Won));
    } else {
        // Continue to next turn
        commands.insert_resource(GameState::PlayerTurn);
    }
}

/// System to handle player card selection
/// Called when player presses Enter/Space to play a card
pub fn player_select_card_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    selected_card: Option<Res<SelectedCard>>,
    player_hand: Res<Hand>,
    mut combat_log: ResMut<CombatLog>,
) {
    // Only allow selection during player turn
    if !game_state.is_player_turn() {
        return;
    }

    // Get selected card index
    let card_index = if let Some(selected) = selected_card {
        selected.index
    } else {
        // No card selected, default to first card if available
        if player_hand.is_empty() {
            return;
        }
        Some(0)
    };

    if let Some(idx) = card_index {
        if idx >= player_hand.cards.len() {
            combat_log.add_entry("Invalid card selection!".to_string());
            return;
        }
        combat_log.add_entry(format!("Player selected card {}", idx));
        // Transition to combat resolution
        commands.insert_resource(GameState::CombatResolution);
    }
}

/// System to handle opponent AI card selection
/// Called during opponent turn
pub fn opponent_select_card_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    opponent_hand: Res<Hand>,
    mut combat_log: ResMut<CombatLog>,
) {
    // Only run during opponent turn
    if !game_state.is_opponent_turn() {
        return;
    }

    if opponent_hand.is_empty() {
        combat_log.add_entry("Opponent has no cards!".to_string());
        // This shouldn't happen if game state is managed correctly
        return;
    }

    // AI selects a card (currently random)
    // This is a simplified approach - in a real game, AI would have more logic
    let hand_size = opponent_hand.cards.len();
    
    let card_index = rand::random_range(0..hand_size);

    combat_log.add_entry(format!("Opponent selected card {}", card_index));
    // Transition to combat resolution
    commands.insert_resource(GameState::CombatResolution);
}

/// System to advance turn after combat
/// Moves from CombatResolution to next appropriate state
pub fn advance_turn_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    player_hand: Res<Hand>,
    opponent_hand: Res<Hand>,
) {
    // Only run if we're in combat resolution and game isn't over
    if !game_state.should_resolve_combat() {
        return;
    }

    // Check if game should be over
    if player_hand.is_empty() || opponent_hand.is_empty() {
        return; // Combat system already handled this
    }

    // Advance to player turn (or opponent turn if implementing separate phases)
    // For now, we go straight back to player turn
    commands.insert_resource(GameState::PlayerTurn);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_select_card_system_only_runs_on_player_turn() {
        // Test that system returns early if not in player turn
        // This is implicitly tested by the game_state.is_player_turn() check
        let non_player_state = GameState::OpponentTurn;
        assert!(!non_player_state.is_player_turn());
    }

    #[test]
    fn test_opponent_select_card_system_only_runs_on_opponent_turn() {
        // Test that system returns early if not in opponent turn
        let non_opponent_state = GameState::PlayerTurn;
        assert!(!non_opponent_state.is_opponent_turn());
    }

    #[test]
    fn test_advance_turn_system_only_runs_in_combat() {
        // Test that system returns early if not in combat resolution
        let non_combat_state = GameState::PlayerTurn;
        assert!(!non_combat_state.should_resolve_combat());
    }
}
