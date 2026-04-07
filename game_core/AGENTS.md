# game_core - Domain Logic Crate

## Purpose

Contains **all game mechanics and state management** for the Poker Card RPG. This crate is:
- **UI-Agnostic**: No dependencies on ratatui, crossterm, or any UI framework
- **Testable**: 110 unit tests with no external dependencies
- **Reusable**: Can be used by TUI, GUI (Bevy), web, or any other renderer

---

## Public API

### Core Types

```rust
// Game state orchestration
pub struct GameSession { ... }

// Turn progression
pub enum GameStateLoop { ... }

// Game entities
pub struct GameSession { ... }
pub struct Deck { ... }
pub struct Hand { ... }
pub struct Card { ... }

// Combat
pub fn apply_combat_damage(
    attacker: &Card,
    defender: &mut Card
) -> (u8, CombatResult)

// Results
pub enum GameResult { Won, Lost, Draw }
pub enum CombatResult { Normal, CriticalHit, Absorb }
```

---

## Module Overview

### game_loop.rs (19 lines, 10 tests)

**GameStateLoop** - Finite State Machine for game progression

States:
- `Start` → Initial state
- `SelectPlayerCard` → Player chooses their card (←→)
- `SelectOpponentTarget` → Player chooses target (←→)
- `ConfirmAttack` → Player confirms (Y/N)
- `ResolvingCombat` → Damage calculation
- `WaitingForOpponent` → AI turn
- `GameOver` → Win/Loss detected
- `Quit` → User quit

Key Methods:
- `advance_after_player_card_selected()` - Transition to target selection
- `cancel_target_selection()` - Go back to card selection
- `reset_to_player_turn()` - After combat resolved

### game_session.rs (371 lines, 9 tests)

**GameSession** - Complete game orchestration

Fields:
- `loop_state: GameStateLoop` - Current state
- `player_hand: Hand` - Player's 5 cards
- `opponent_hand: Hand` - Opponent's 5 cards (hidden)
- `deck: Deck` - Remaining cards
- `combat_log: CombatLog` - Event history
- `game_over_result: Option<GameResult>` - Win/Loss state

Key Methods:
- `start_new_game()` - Initialize deck, shuffle, deal 5 cards each
- `resolve_player_attack(player_idx, opponent_idx)` - Execute combat
- `check_game_over()` - Detect win/loss conditions
- `is_player_turn()`, `is_opponent_turn()`, `is_resolving()` - State queries

### combat/mod.rs

**Combat Engine** - Damage calculation with archetypes

Archetypes (by suit):
- Hearts = Rock
- Diamonds = Paper
- Clubs = Scissors
- Spades = Infantry

Damage Multipliers:
- Dominant → Lesser: 0.5x (chance to absorb = 0x)
- Lesser → Dominant: 2.0x (chance of crit = 5.0x)
- Infantry → Any: 1.0x (can crit or absorb)

### deck.rs, hand.rs, card.rs

**Card Management**

- `Card`: suit, rank, hp, max_hp
- `Hand`: Vec<Card>, `add_card()`, `remove_dead_cards()`
- `Deck`: 52 cards, `draw()`, `shuffle()`

---

## Adding New Features

### Step 1: Define in game_core

```rust
// Example: Add card ability system
pub enum CardAbility {
    Heal,
    DoubleDamage,
    ...
}

pub struct Card {
    // ... existing fields
    pub ability: Option<CardAbility>,
}
```

### Step 2: Update GameSession

```rust
impl GameSession {
    pub fn apply_card_abilities(&mut self) {
        // Logic here
    }
}
```

### Step 3: Add Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_ability_heal() {
        // ...
    }
}
```

### Step 4: Update UI (if needed)

Update `game_tui` rendering to show new features.

---

## Test Command

```bash
cd game_core
cargo test        # All 110 tests
cargo test <name> # Filter tests
cargo test -- --nocapture # See output
```

---

## Dependencies

- `bevy`: For ECS Component trait (Card has #[derive(Component)])
- `rand`: For deck shuffling
- No UI dependencies!

---

## Design Principles

1. **All game logic lives here** - No logic in game_tui
2. **Pure functions** - Most functions are `fn(input) -> output`
3. **No side effects** - Easy to test and reason about
4. **Type safety** - Enums, Result types prevent invalid states

---

## Common Patterns

### State Transitions

```rust
// In game_session.rs
self.loop_state = self.loop_state.advance_after_player_card_selected();
```

### Card Operations

```rust
// Take damage
let card = &mut self.player_hand.cards[idx];
card.take_damage(damage);

// Check if dead
if !card.is_alive() {
    // Will be removed by remove_dead_cards()
}
```

### Combat Flow

```rust
let (dmg1, result1) = apply_combat_damage(&player_card, &mut opponent_copy);
let (dmg2, result2) = apply_combat_damage(&opponent_card, &mut player_copy);

// Update cards
self.player_hand.cards[player_idx] = player_copy;
self.opponent_hand.cards[opponent_idx] = opponent_copy;

// Remove dead
self.player_hand.remove_dead_cards();
self.opponent_hand.remove_dead_cards();
```

---

## Migration Notes

If migrating from old FullGameState in game_tui:

1. Use `GameSession` instead of `FullGameState`
2. Access fields directly: `game.player_hand`, `game.loop_state`
3. Call methods: `game.resolve_player_attack(idx, idx)`
4. Check state: `game.is_player_turn()`

All old logic has been moved here!
