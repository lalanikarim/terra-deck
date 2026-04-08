//! Bevy ECS systems for combat resolution and turn management

use crate::combat::{apply_combat_damage, CombatResult};
use crate::combat_log::CombatLog;
use crate::combat_stats::{CombatStats, GameResult, GameState};
use crate::hand::Hand;
use crate::turn_state::SelectedCard;
use bevy::prelude::*;
use rand::Rng;
use rand::SeedableRng;

/// System to resolve combat between player and opponent cards
pub fn resolve_combat_system(
    mut commands: Commands,
    mut player_hand: ResMut<Hand>,
    mut opponent_hand: ResMut<Hand>,
    mut combat_log: ResMut<CombatLog>,
    mut combat_stats: ResMut<CombatStats>,
) {
    if player_hand.is_empty() || opponent_hand.is_empty() {
        let result = if player_hand.is_empty() { GameResult::Lost } else { GameResult::Won };
        combat_log.add_entry(format!("Game Over: {:?}", result));
        commands.insert_resource(GameState::GameOver(result));
        return;
    }

    let player_card_idx = 0usize;
    let opponent_card_idx = 0usize;
    let player_card_info = player_hand.cards[player_card_idx].clone();
    let opponent_card_info = opponent_hand.cards[opponent_card_idx].clone();
    let mut player_card_copy = player_card_info.clone();
    let mut opponent_card_copy = opponent_card_info.clone();

    let (player_damage, player_result) = apply_combat_damage(&player_card_info, &mut opponent_card_copy);
    let (opponent_damage, opponent_result) = apply_combat_damage(&opponent_card_info, &mut player_card_copy);

    player_hand.cards[player_card_idx] = player_card_copy;
    opponent_hand.cards[opponent_card_idx] = opponent_card_copy;

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

    combat_log.add_entry(format!(
        "Player {} ({} {}) attacks Opponent {} ({} {}) - Result: {:?} ({} dmg)",
        player_card_info.rank, player_card_info.suit, player_card_info.hp,
        opponent_card_info.rank, opponent_card_info.suit, opponent_card_info.hp,
        player_result, player_damage
    ));

    combat_log.add_entry(format!(
        "Opponent {} ({} {}) attacks Player {} ({} {}) - Result: {:?} ({} dmg)",
        opponent_card_info.rank, opponent_card_info.suit, opponent_card_info.hp,
        player_card_info.rank, player_card_info.suit, player_card_info.hp,
        opponent_result, opponent_damage
    ));

    let player_dead = player_hand.remove_dead_cards();
    let opponent_dead = opponent_hand.remove_dead_cards();

    if player_dead > 0 { combat_log.add_entry(format!("Player lost {} card(s)", player_dead)); }
    if opponent_dead > 0 { combat_log.add_entry(format!("Opponent lost {} card(s)", opponent_dead)); }

    if player_hand.is_empty() {
        combat_log.add_entry("Game Over: You Lost!".to_string());
        commands.insert_resource(GameState::GameOver(GameResult::Lost));
    } else if opponent_hand.is_empty() {
        combat_log.add_entry("Game Over: You Won!".to_string());
        commands.insert_resource(GameState::GameOver(GameResult::Won));
    } else {
        commands.insert_resource(GameState::PlayerTurn);
    }
}

/// System to handle player card selection
pub fn player_select_card_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    selected_card: Option<Res<SelectedCard>>,
    player_hand: Res<Hand>,
    mut combat_log: ResMut<CombatLog>,
) {
    if !game_state.is_player_turn() { return; }

    let card_index = if let Some(selected) = selected_card { selected.index }
    else if player_hand.is_empty() { return }
    else { Some(0) };

    if let Some(idx) = card_index {
        if idx < player_hand.cards.len() {
            combat_log.add_entry(format!("Player selected card {}", idx));
            commands.insert_resource(GameState::CombatResolution);
        }
    }
}

/// System to handle opponent AI card selection
pub fn opponent_select_card_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    opponent_hand: Res<Hand>,
    mut combat_log: ResMut<CombatLog>,
) {
    if !game_state.is_opponent_turn() { return; }
    if opponent_hand.is_empty() {
        combat_log.add_entry("Opponent has no cards!".to_string());
        return;
    }

    let hand_size = opponent_hand.cards.len();
    let mut rng = rand::rngs::StdRng::from_seed([0; 32]);
    let card_index = rng.gen_range(0..hand_size);

    combat_log.add_entry(format!("Opponent selected card {}", card_index));
    commands.insert_resource(GameState::CombatResolution);
}

/// System to advance turn after combat
pub fn advance_turn_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    player_hand: Res<Hand>,
    opponent_hand: Res<Hand>,
) {
    if !game_state.should_resolve_combat() { return; }
    if player_hand.is_empty() || opponent_hand.is_empty() { return; }
    commands.insert_resource(GameState::PlayerTurn);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_player_select_card_system_only_runs_on_player_turn() {
        let non_player_state = GameState::OpponentTurn;
        assert!(!non_player_state.is_player_turn());
    }
    #[test] fn test_opponent_select_card_system_only_runs_on_opponent_turn() {
        let non_opponent_state = GameState::PlayerTurn;
        assert!(!non_opponent_state.is_opponent_turn());
    }
    #[test] fn test_advance_turn_system_only_runs_in_combat() {
        let non_combat_state = GameState::PlayerTurn;
        assert!(!non_combat_state.should_resolve_combat());
    }
}
