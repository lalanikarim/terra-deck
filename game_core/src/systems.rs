//! Bevy ECS systems for combat resolution
use crate::combat::{apply_combat_damage, CombatResult};
use crate::combat_log::CombatLog;
use crate::combat_stats::{CombatStats, GameResult, GameState};
use crate::hand::Hand;
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
