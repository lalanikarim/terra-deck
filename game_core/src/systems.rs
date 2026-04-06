//! Bevy ECS systems for combat resolution
use crate::combat::{apply_combat_damage, CombatResult};
use crate::combat_log::CombatLog;
use crate::combat_stats::CombatStats;
use crate::hand::Hand;
use bevy::prelude::*;

/// System to resolve combat between player and opponent cards
pub fn resolve_combat_system(
    _commands: Commands,
    player_hand: Res<Hand>,
    opponent_hand: Res<Hand>,
    mut combat_log: ResMut<CombatLog>,
    mut combat_stats: ResMut<CombatStats>,
) {
    // Simple implementation: first card in each hand fights
    // In a full game, this would be more sophisticated

    if let (Some(player_card), Some(opponent_card)) = (player_hand.get(0), opponent_hand.get(0)) {
        // Make mutable copies for combat
        let mut player_card_copy = player_card.clone();
        let mut opponent_card_copy = opponent_card.clone();

        // Player attacks opponent
        let (player_damage, player_result) =
            apply_combat_damage(player_card, &mut opponent_card_copy);

        // Opponent attacks player
        let (opponent_damage, opponent_result) =
            apply_combat_damage(opponent_card, &mut player_card_copy);

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

        // Log the combat
        combat_log.add_entry(format!(
            "Player {} (HP: {}) attacks Opponent {} (HP: {}) - Result: {:?} ({} dmg)",
            player_card.rank,
            player_card.hp,
            opponent_card.rank,
            opponent_card.hp,
            player_result,
            player_damage
        ));

        combat_log.add_entry(format!(
            "Opponent {} (HP: {}) attacks Player {} (HP: {}) - Result: {:?} ({} dmg)",
            opponent_card.rank,
            opponent_card.hp,
            player_card.rank,
            player_card.hp,
            opponent_result,
            opponent_damage
        ));

        // Update hands with damaged cards (simplified - in real game would manage game state properly)
        // For now, we'll just note that cards were damaged
    }
}
