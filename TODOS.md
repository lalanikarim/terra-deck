# Poker Card RPG - Task List

Progress tracking for system-by-system development.

## ✅ Completed

### Project Setup
- [x] Project Scaffolding: Initialize cargo project with core/tui split and dependencies (Bevy, Ratatui)
- [x] Domain: Define Card, Suit, and Rank components and enums
- [x] Domain: Implement Deck generation and shuffling logic

### Core Domain
- [x] **Combat Engine**: Damage multiplier and probability logic (RPS + Infantry)
- [x] **Combat System Logic**: Persist card HP, remove dead cards, check win/loss
- [x] **Test Suite**: 65 tests organized by module (card, deck, hand, types, combat, combat_log)

### Code Organization
- [x] Reorganize tests from lib.rs into respective modules
- [x] Clean lib.rs (module declarations only, 21 lines)

---

## 🟡 In Progress

### Task 2: Turn Management FSM
**Goal**: Implement state machine for turn progression

**Current State:**
- `GameState` enum exists with variants: `PlayerTurn`, `OpponentTurn`, `CombatResolution`, `GameOver(GameResult)`

**What needs to be done:**

1. **State Transition Logic**
   - [ ] Implement `GameState::advance()` method
     - Input: current state, combat result stats
     - Output: next state
   - [ ] Implement `GameState::should_resolve_combat()` → returns true if in `CombatResolution` phase
   - [ ] Implement `GameState::is_active()` → returns false when in `GameOver`
   - [ ] Implement `GameState::reset()` → returns to `PlayerTurn` with fresh stats

2. **Turn Progression Rules**
   - [ ] PlayerTurn → Player selects a card → transitions to CombatResolution
   - [ ] CombatResolution → Combat system runs → transitions to OpponentTurn
   - [ ] OpponentTurn → AI selects card → transitions to CombatResolution
   - [ ] Win condition → transitions to GameOver(Won/Lost/Draw)

3. **AI Opponent Logic** (OpponentTurn behavior)
   - [ ] Basic AI: randomly select from available cards
   - [ ] Store AI opponent in separate `Hand` resource
   - [ ] Add system: `opponent_select_card_system()`

4. **Card Selection System** (PlayerTurn behavior)
   - [ ] Player indicates card to play (initially via hardcoded, later via UI input)
   - [ ] Store selected card index in `SelectedCard` resource
   - [ ] Validate selection is valid (card exists, not dead)

5. **Tests to Add**
   - [ ] State transition tests (each valid path)
   - [ ] AI card selection tests
   - [ ] Game reset tests

**Files to modify:**
- `game_core/src/combat_stats.rs` - Add methods to `GameState`
- `game_core/src/systems.rs` - Add turn progression systems
- New: `game_core/src/ai.rs` - Opponent AI logic (optional, can inline for now)

---

## ❌ Next

### Task 3: Create TUI Binary Crate
**Goal**: Set up `game_tui` binary that can instantiate Bevy app with game_core

**What needs to be done:**

1. **Project Structure**
   - [ ] Create `game_tui/` directory
   - [ ] Create `game_tui/Cargo.toml` with dependencies
   - [ ] Update workspace `Cargo.toml` to include `game_tui` member
   - [ ] Create `game_tui/src/main.rs`

2. **Cargo.toml Dependencies**
   - [ ] `bevy = "0.18"`
   - [ ] `ratatui = "0.30"`
   - [ ] `crossterm = "0.29"`
   - [ ] `game_core = { path = "../game_core" }`

3. **Bevy App Setup**
   - [ ] Create `App::new()` with `DefaultPlugins`
   - [ ] Add `game_core` resources to app: `GameState`, `CombatStats`, `CombatLog`
   - [ ] Add `Hand` resources: `PlayerHand`, `OpponentHand`
   - [ ] Add `Deck` resource
   - [ ] Initialize startup system (deal initial hands)

4. **Basic System Wiring**
   - [ ] Startup system: `init_game_system()` - creates deck, shuffles, deals hands
   - [ ] Combat system: `resolve_combat_system()` - already exists, wire it up
   - [ ] Set up system sets/pipeline

5. **Test the Binary**
   - [ ] `cargo run --package game_tui` should start without crashing
   - [ ] No UI needed yet, just headless Bevy app

**Files to create:**
- `game_tui/Cargo.toml`
- `game_tui/src/main.rs`
- Update: `Cargo.toml` (workspace)

---

### Task 4: Build TUI Rendering Components
**Goal**: Display game state in terminal using ratatui

**What needs to be done:**

1. **TUI Framework Setup**
   - [ ] Initialize ratatui terminal in main.rs
   - [ ] Set up terminal event loop with crossterm
   - [ ] Create `UiState` struct to track rendering state (selected card, scroll offset)

2. **Layout Components**
   - [ ] Create main layout: header, player hand, combat log, opponent hand, footer
   - [ ] Each component as separate function/module: `ui/header.rs`, `ui/hand.rs`, etc.

3. **Player Hand Rendering** (`ui/hand.rs`)
   - [ ] Display each card as: `[2♥ H:14] [5♦ H:14] [J♣ H:14]`
   - [ ] Highlight selected card (different color/bg)
   - [ ] Show card rank, suit, current HP
   - [ ] Add card index numbers for selection

4. **Opponent Hand Rendering** (`ui/hand.rs`)
   - [ ] Show number of cards (hidden)
   - [ ] Display as: `[?][?][?] - 3 unknown cards`
   - [ ] During combat, optionally reveal opponent's fighting card

5. **Combat Log Rendering** (`ui/log.rs`)
   - [ ] Create scrollable area (max 10-15 visible lines)
   - [ ] Render log messages with colors for different events
   - [ ] Handle scroll offset updates when new messages added

6. **Turn Indicator** (`ui/header.rs`)
   - [ ] Display current `GameState`: "YOUR TURN", "OPPONENT TURN", "COMBAT!", "GAME OVER"
   - [ ] Show different colors per state

7. **Help/Footer** (`ui/footer.rs`)
   - [ ] Display input hints: "Arrow keys: move | Enter: play card | q: quit"

8. **Rendering Loop**
   - [ ] Connect TUI render function to Bevy `Update` event
   - [ ] Ensure UI updates each frame based on current game state

**Files to create:**
- `game_tui/src/ui/mod.rs`
- `game_tui/src/ui/header.rs`
- `game_tui/src/ui/hand.rs`
- `game_tui/src/ui/log.rs`
- `game_tui/src/ui/footer.rs`

---

### Task 5: Add TUI Input Handling
**Goal**: Accept user input to control game flow

**What needs to be done:**

1. **Input Event Setup**
   - [ ] Configure crossterm for raw input mode
   - [ ] Handle keyboard events: Arrow/Keys, Enter, q, etc.
   - [ ] Map raw events to custom `InputAction` enum

2. **Input Actions**
   - [ ] `MoveLeft`, `MoveRight` - select different card in hand
   - [ ] `MoveUp`, `MoveDown` - scroll combat log
   - [ ] `Select` (Enter/Space) - play selected card, trigger combat
   - [ ] `Skip` - pass turn without playing
   - [ ] `Quit` (q/Escape) - exit game

3. **Input Processing System**
   - [ ] Create `process_input_system()` that reads `InputAction`
   - [ ] Update `SelectedCard` resource based on movement
   - [ ] Transition game state based on actions
   - [ ] Validate input is legal for current state (e.g., can't play card in OpponentTurn)

4. **Edge Cases**
   - [ ] Hand has only 1 card selected (can't move left/right)
   - [ ] Empty hand (game over or draw phase)
   - [ ] Invalid input during GameOver state (ignored or force quit)

5. **Feedback**
   - [ ] Visual feedback when action invalid (flash/hide)
   - [ ] Log message for actions taken

**Files to create:**
- `game_tui/src/input.rs`
- `game_tui/src/event_loop.rs` (or integrate into main.rs)

---

### Task 6: End-to-End Playable Loop Testing
**Goal**: Verify complete game works from start to finish

**What needs to be done:**

1. **Test Scenarios (Manual + Code)**
   - [ ] Start game → deck created → hands dealt (5-7 cards each)
   - [ ] Player can navigate hand with keys
   - [ ] Player selects card → combat triggers
   - [ ] Combat resolves with correct damage shown in log
   - [ ] Cards show updated HP
   - [ ] Cards at 0 HP disappear from hand
   - [ ] Turn passes to opponent (or back to player, depending on design)
   - [ ] Repeating until one hand is empty
   - [ ] Game displays "You Won/Lost" on completion
   - [ ] Ability to restart or quit

2. **Code Tests** (integration tests in `game_tui/tests/`)
   - [ ] Full game loop test (headless simulation)
   - [ ] Verify combat damage is calculated correctly over multiple rounds
   - [ ] Verify dead cards are removed
   - [ ] Verify win condition triggers at right time

3. **Bug Hunting**
   - [ ] Handle edge case: both hands empty at same time (draw)
   - [ ] Handle edge case: card takes 0 damage (absorb) multiple times
   - [ ] Handle edge case: player skips turn repeatedly
   - [ ] Verify no panics or asserts during normal play

**Files to create:**
- `game_tui/tests/integration_tests.rs`
- New resource: `game_core/src/game_state.rs` (for restart/reset logic if needed)

---

## 📋 Technical Debt & Notes

### Current Limitations
- [ ] Shuffling uses `thread_rng()` but not cryptographically secure (fine for game)
- [ ] Combat always uses first cards in hand (should add configurable strategy)
- [ ] No card drawing between turns (should implement draw phase to replace played cards)
- [ ] AI is random (should add smart AI later)
- [ ] No combo/synergy mechanics between cards

### Future Enhancements (Post-MVP)
- [ ] Card synergies (combo bonuses when certain suits play together)
- [ ] Special abilities/skills on certain cards
- [ ] Multi-round battles with mana/energy system
- [ ] Score tracking and ranking
- [ ] Sound effects (bevy_audio)
- [ ] Transition to Bevy graphics (Phase 2 - replace ratatui with sprites)

---

## Progress Summary

| Phase | Status | Tests | Notes |
|-------|-----|------|-------|
| Core Domain | ✅ Complete | 65 tests | All combat logic working |
| Combat System | ✅ Complete | Included | Task 1 done |
| Turn FSM | 🟡 Partial | 0 tests | Task 2 in progress |
| TUI Layer | ❌ Not started | 0 tests | Tasks 3-5 |
| Integration | ❌ Not started | 0 tests | Task 6 |

**Total Tests: 65 passing** (0 failed)

**Current Focus**: Task 2 - Turn Management FSM
