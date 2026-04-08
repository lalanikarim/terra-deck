//! Combat logic and damage calculation

use crate::types::{Archetype, Suit};
use crate::Card;
use rand::Rng;
use rand::SeedableRng;

/// Enum representing the result of a combat interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatResult {
    Normal,
    CriticalHit,
    Absorb,
}

/// is the probability of a special effect (absorb or crit)
pub fn calculate_damage_multiplier(attacker_suit: Suit, defender_suit: Suit) -> (f32, f32) {
    let attacker_archetype = attacker_suit.archetype();
    let defender_archetype = defender_suit.archetype();

    // Infantry is vulnerable - when exactly one participant is Infantry,
    // standard 1x damage with 25% special chance
    if (attacker_archetype == Archetype::Infantry) != (defender_archetype == Archetype::Infantry) {
        return (1.0, 0.25);
    }

    // RPS logic: Rock > Scissors > Paper > Rock
    match (attacker_archetype, defender_archetype) {
        // Same archetype - standard damage
        (Archetype::Rock, Archetype::Rock) => (1.0, 0.0),
        (Archetype::Paper, Archetype::Paper) => (1.0, 0.0),
        (Archetype::Scissors, Archetype::Scissors) => (1.0, 0.0),
        (Archetype::Infantry, Archetype::Infantry) => (1.0, 0.0),

        // Attacker dominates defender (attacker wins)
        (Archetype::Rock, Archetype::Scissors) => (0.5, 0.3), // 50% damage, 30% chance to absorb
        (Archetype::Scissors, Archetype::Paper) => (0.5, 0.3), // 50% damage, 30% chance to absorb
        (Archetype::Paper, Archetype::Rock) => (0.5, 0.3),    // 50% damage, 30% chance to absorb

        // Defender dominates attacker (defender wins)
        (Archetype::Scissors, Archetype::Rock) => (2.0, 0.25), // 2x damage, 25% chance to crit (5x total)
        (Archetype::Paper, Archetype::Scissors) => (2.0, 0.25), // 2x damage, 25% chance to crit (5x total)
        (Archetype::Rock, Archetype::Paper) => (2.0, 0.25), // 2x damage, 25% chance to crit (5x total)

        _ => (1.0, 0.0),
    }
}

/// Applies damage to a defender card based on attacker card
/// Returns the actual damage dealt and whether it was a critical hit or absorb
pub fn apply_combat_damage(attacker: &Card, defender: &mut Card) -> (u8, CombatResult) {
    let base_damage = attacker.rank as u8;
    let (multiplier, special_chance) = calculate_damage_multiplier(attacker.suit, defender.suit);

    // Calculate modified damage
    let modified_damage = (base_damage as f32 * multiplier) as u8;

    // Determine if special effect occurs
    let mut rng = rand::rngs::StdRng::from_seed([0; 32]);
    let special_occurs = rng.gen_range(0.0..1.0) < special_chance;

    let (final_damage, result) = if special_occurs {
        match (attacker.suit.archetype(), defender.suit.archetype()) {
            // Dominant vs Lesser - chance to absorb
            (Archetype::Rock, Archetype::Scissors)
            | (Archetype::Scissors, Archetype::Paper)
            | (Archetype::Paper, Archetype::Rock) => {
                // 50% chance to absorb (0x) vs normal reduced damage
                if rng.gen_range(0.0..1.0) < 0.5 {
                    (0, CombatResult::Absorb)
                } else {
                    (modified_damage, CombatResult::Normal)
                }
            }
            // Lesser vs Dominant - chance to crit
            (Archetype::Scissors, Archetype::Rock)
            | (Archetype::Paper, Archetype::Scissors)
            | (Archetype::Rock, Archetype::Paper) => {
                // Crit does 5x total damage (2x base multiplier * 2.5 crit bonus = 5x)
                (modified_damage * 5, CombatResult::CriticalHit)
            }
            // Infantry special - 0.25x absorb or 2x crit
            (_, Archetype::Infantry) | (Archetype::Infantry, _) => {
                // 50/50 split between absorb and crit for infantry
                if rng.gen_range(0.0..1.0) < 0.5 {
                    (0, CombatResult::Absorb)
                } else {
                    (modified_damage * 2, CombatResult::CriticalHit)
                }
            }
            // Same archetype - no special
            _ => (modified_damage, CombatResult::Normal),
        }
    } else {
        (modified_damage, CombatResult::Normal)
    };

    // Apply damage (capped at defender's current HP)
    let actual_damage = defender.take_damage(final_damage);

    (actual_damage, result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Rank;

    #[test]
    fn test_damage_multiplier_same_archetype() {
        // Rock vs Rock
        assert_eq!(
            calculate_damage_multiplier(Suit::Hearts, Suit::Hearts),
            (1.0, 0.0)
        );
        // Paper vs Paper
        assert_eq!(
            calculate_damage_multiplier(Suit::Diamonds, Suit::Diamonds),
            (1.0, 0.0)
        );
        // Scissors vs Scissors
        assert_eq!(
            calculate_damage_multiplier(Suit::Clubs, Suit::Clubs),
            (1.0, 0.0)
        );
        // Infantry vs Infantry
        assert_eq!(
            calculate_damage_multiplier(Suit::Spades, Suit::Spades),
            (1.0, 0.0)
        );
    }

    #[test]
    fn test_damage_multiplier_rps_dominant_vs_lesser() {
        // Rock dominates Scissors
        assert_eq!(
            calculate_damage_multiplier(Suit::Hearts, Suit::Clubs),
            (0.5, 0.3)
        );
        // Scissors dominates Paper
        assert_eq!(
            calculate_damage_multiplier(Suit::Clubs, Suit::Diamonds),
            (0.5, 0.3)
        );
        // Paper dominates Rock
        assert_eq!(
            calculate_damage_multiplier(Suit::Diamonds, Suit::Hearts),
            (0.5, 0.3)
        );
    }

    #[test]
    fn test_damage_multiplier_rps_lesser_vs_dominant() {
        // Scissors vs Rock (defender dominates)
        assert_eq!(
            calculate_damage_multiplier(Suit::Clubs, Suit::Hearts),
            (2.0, 0.25)
        );
        // Paper vs Scissors (defender dominates)
        assert_eq!(
            calculate_damage_multiplier(Suit::Diamonds, Suit::Clubs),
            (2.0, 0.25)
        );
        // Rock vs Paper (defender dominates)
        assert_eq!(
            calculate_damage_multiplier(Suit::Hearts, Suit::Diamonds),
            (2.0, 0.25)
        );
    }

    #[test]
    fn test_damage_multiplier_infantry_vulnerable() {
        // Infantry vs any = 1x damage with 25% special chance
        assert_eq!(
            calculate_damage_multiplier(Suit::Spades, Suit::Hearts),
            (1.0, 0.25)
        ); // Infantry attacks Rock
        assert_eq!(
            calculate_damage_multiplier(Suit::Hearts, Suit::Spades),
            (1.0, 0.25)
        ); // Rock attacks Infantry
        assert_eq!(
            calculate_damage_multiplier(Suit::Spades, Suit::Diamonds),
            (1.0, 0.25)
        ); // Infantry attacks Paper
        assert_eq!(
            calculate_damage_multiplier(Suit::Diamonds, Suit::Spades),
            (1.0, 0.25)
        ); // Paper attacks Infantry
    }

    #[test]
    fn test_apply_combat_damage_deterministic() {
        let attacker = Card::new(Suit::Hearts, Rank::Ten); // 10 HP Rock
        let mut defender = Card::new(Suit::Clubs, Rank::Five); // 5 HP Scissors (Rock > Scissors)

        // Rock vs Scissors: 0.5x damage, 30% absorb chance
        let initial_hp = defender.hp;
        let (damage, result) = apply_combat_damage(&attacker, &mut defender);

        // Damage should be applied (5 * 0.5 = 2.5 -> 2 base damage, possibly modified by special)
        assert!(damage <= initial_hp);
        assert_eq!(defender.hp, initial_hp - damage);
        assert!(matches!(
            result,
            CombatResult::Normal | CombatResult::Absorb
        ));
    }

    #[test]
    fn test_apply_combat_damage_infantry_special() {
        let attacker = Card::new(Suit::Hearts, Rank::Eight); // 8 HP Rock
        let mut defender = Card::new(Suit::Spades, Rank::Four); // 4 HP Infantry

        // Test that damage is applied
        let initial_hp = defender.hp;
        let (damage, result) = apply_combat_damage(&attacker, &mut defender);

        assert!(damage <= initial_hp);
        assert_eq!(defender.hp, initial_hp - damage);
        // Result should be Normal, Absorb, or CriticalHit based on RNG
        assert!(matches!(
            result,
            CombatResult::Normal | CombatResult::Absorb | CombatResult::CriticalHit
        ));
    }

    #[test]
    fn test_combat_result_enum() {
        assert!(matches!(CombatResult::Normal, CombatResult::Normal));
        assert!(matches!(
            CombatResult::CriticalHit,
            CombatResult::CriticalHit
        ));
        assert!(matches!(CombatResult::Absorb, CombatResult::Absorb));

        assert_ne!(CombatResult::Normal, CombatResult::CriticalHit);
        assert_ne!(CombatResult::Normal, CombatResult::Absorb);
        assert_ne!(CombatResult::CriticalHit, CombatResult::Absorb);
    }
}
