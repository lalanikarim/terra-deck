# Poker Card RPG - Task List

All tasks complete! 🎉

---

## ✅ Completed Tasks

### Core Domain
- [x] **Task 1: Combat System Logic**  
  - GameResult enum (Won/Lost/Draw)
  - Hand::remove_dead_cards() method
  - Win/loss condition detection
  - Combat log updates

- [x] **Task 2: Turn Management FSM**
  - GameState methods (is_active, is_player_turn, etc.)
  - SelectedCard resource with navigation methods
  - AI module with random card selection
  - Turn progression systems

### TUI Implementation
- [x] **Task 3: TUI Binary Crate**
  - game_tui/Cargo.toml with ratatui 0.29, crossterm 0.28
  - Standalone terminal app with main loop
  - No Bevy runtime dependency (simplified)

- [x] **Task 4: TUI Rendering Components**
  - ui/header.rs - Title display
  - ui/hand.rs - Player hand with card selection highlight
  - ui/log.rs - Combat log with colored entries
  - ui/footer.rs - Help text display
  - ui/mod.rs - AppUiState and render_game()

- [x] **Task 5: Input Handling**
  - Arrow keys / h l for navigation
  - Space / Enter to play card
  - q / Esc to quit
  - Selection highlighting

- [x] **Task 6: End-to-End Integration Tests**
  - 9 integration tests in game_tui/tests/integration_tests.rs
  - Full game flow from deck creation to combat
  - Win/loss condition testing
  - Combat mechanics verification
  - SelectedCard navigation tests

---

## Test Summary

| Testsuite | Tests | Passing |
|--|-----|----|
| Game Core (card, deck, hand, types, combat, etc.) | 92 | ✅ 92 |
| TUI Unit Tests | 5 | ✅ 5 |
| Integration Tests | 9 | ✅ 9 |
| **TOTAL** | **106** | **✅ 106** |

---

## Run the Game

```bash
cargo run --package game_tui
```

### Controls:
- **← →** or **h l** - Navigate cards
- **Space / Enter** - Play selected card
- **q / Esc** - Quit

---

## Git History

```
94a0846 - Complete Task 6: End-to-End integration tests
cb7dfa6 - Merge Tasks 4-5: Complete TUI with full rendering and input
69eb0f6 - Complete Task 4: Build TUI rendering components
407c6c2 - Complete Task 3: Create TUI binary crate
fb6e4fe - Complete Task 2: Turn Management FSM
aca2a5d - Complete Task 1: Implement combat system logic
192b3dd - Update project organization
f7a6f86 - Create TODOS.md with detailed task breakdown
```

---

## Project Structure

```
bevygame/
├── game_core/          ← Core game logic (92 tests)
│   ├── src/
│   │   ├── ai.rs          ← AI opponent logic
│   │   ├── card.rs        ← Card struct and methods
│   │   ├── combat/        ← Combat mechanics
│   │   ├── combat_log.rs  ← Event logging
│   │   ├── combat_stats.rs← GameState, GameResult
│   │   ├── deck.rs        ← Deck management
│   │   ├── hand.rs        ← Hand management
│   │   ├── systems.rs     ← Combat systems
│   │   ├── turn_state.rs  ← SelectedCard
│   │   └── types.rs       ← Suit, Rank, Archetype
│   └── Cargo.toml
├── game_tui/           ← Terminal UI (9 tests)
│   ├── src/
│   │   ├── main.rs       ← Terminal setup and loop
│   │   └── ui/
│   │       ├── footer.rs
│   │       ├── hand.rs
│   │       ├── header.rs
│   │       ├── log.rs
│   │       └── mod.rs
│   ├── tests/
│   │   └── integration_tests.rs
│   └── Cargo.toml
├── docs/               ← Knowledge base
│   ├── BEVY_KNOWLEDGE.md
│   ├── CODING_ERRORS.md
│   ├── RAND_KNOWLEDGE.md
│   └── RATATUI_KNOWLEDGE.md
├── TODOS.md            ← This file
└── README.md           ← Project overview
```

---

## 🚧 Pending Tasks

These tasks are required to make the game fully playable.

### Task 7: Full Game Loop Integration
**Goal**: Connect UI with game_core systems for actual gameplay

**What needs to be done:**

1. **Deck and Hand Management**
   - [ ] Initialize full deck at game start (52 cards)
   - [ ] Shuffle deck randomly
   - [ ] Deal 5 cards to player and opponent hands
   - [ ] Update TUI to use real hands from resources
   - [ ] Refill hands from deck when needed?

2. **Combat Trigger and Resolution**
   - [ ] When Space pressed → trigger combat between selected cards
   - [ ] Call `apply_combat_damage()` from game_core
   - [ ] Update card HP in hands based on combat result
   - [ ] Display combat log entries with colored text
   - [ ] Show critical hits, absorbs, multipliers
   - [ ] **Keep opponent card hidden** - only show suit/suit archetype

3. **Dead Card Removal**
   - [ ] Call `Hand::remove_dead_cards()` after combat
   - [ ] Update TUI to show new hand state (fewer cards)
   - [ ] Log which cards died
   - [ ] Handle case where player has no cards left

4. **Turn Progression**
   - [ ] Track current turn (PlayerTurn → OpponentTurn → Combat → PlayerTurn)
   - [ ] AI opponent selects card after player plays
   - [ ] Disable input during opponent turn (waiting visual)
   - [ ] Reset selection after each turn

5. **Win/Loss Detection**
   - [ ] Check `GameState` after each combat round
   - [ ] Display "YOU WON!" or "YOU LOST!" message
   - [ ] **Reveal all opponent cards at game end only**
   - [ ] Show final scoreboard (cards remaining, damage dealt)
   - [ ] Option to restart or quit

6. **Restart Game**
   - [ ] Create `reset_game()` function
   - [ ] Re-shuffle deck, re-deal hands
   - [ ] Clear combat log
   - [ ] Reset to initial state

---

## Opponent Card Display Rules ⚡

**During gameplay:**
- Show as `[?] ●` (alive) or `[X] ✕` (dead)
- **NO HP values visible** - keeps opponent completely hidden
- **NO card values revealed** until game end
- Only alive/dead status shown

**Example opponent hand display:**
```
OPPONENT'S HAND:
 [?] ●  (alive)
 [?] ●  (alive)
 [X] ✕  (dead - removed this turn)
 [?] ●  (alive)
 [X] ✕  (dead)
```

**At game end (victory screen):**
```
OPPONENT'S HAND (REVEALED):
 [?] ♦ 5  HP:0/5   (was dead)
 [?] ♣ J HP:3/11   (alive, took 8 damage)
 [X] ♠ Q HP:0/12  (was dead)
```

**Combat log shows archetype but not card value:**
```
✓ Your ♥ 10 (Rock) vs Opponent ? (Scissors)
  → Rock beats Scissors! You deal 5 damage
  → Opponent hits back, but you absorb 4 damage (0 taken)
```

**What files to modify/create:**
- `game_tui/src/main.rs` (major rewrite needed)
- `game_tui/src/game_state.rs` (new - track game loop state)
- `game_tui/src/ui/mod.rs` (update render to show combat state)
- `game_tui/src/ui/hand.rs` (show HP updates, opponent hidden)
- `game_tui/src/ui/opponent.rs` (new - hidden opponent display)

## Next Steps (Future)

These were out of scope for the initial sprint, but could be implemented later:

- [ ] **Multi-round battles** - Combat continues until both hands empty
- [ ] **Smart AI opponent** - Use combat logic to pick best card each turn
- [ ] **Card drawing between turns** - Pull from deck when hand gets low
- [ ] **Card synergies and abilities** - Special effects for card combos
- [ ] **Persistent save/load** - Resume game later
- [ ] **Transition to Bevy graphics** - Full GUI with sprites and animations (Phase 2)
- [ ] **Sound effects** - Combat sounds, card play sounds
- [ ] **Achievements system** - Track wins, combos, perfect games
