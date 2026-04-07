# game_tui - Terminal UI Crate

## Purpose

Terminal-based user interface for Terra-Deck using **ratatui** and **crossterm**. This crate is:
- **UI-Only**: No game logic lives here
- **Thin Layer**: Delegates all game operations to `game_core::GameSession`
- **Replaceable**: Can be swapped with Bevy GUI, web UI, or mobile UI

---

## Architecture

```
game_tui
├── src/main.rs          # Terminal setup + main loop + input
├── src/game_state.rs    # Re-exports from game_core
└── src/ui/             # Component renderers
    ├── mod.rs          # Layout composition
    ├── header.rs       # Title
    ├── hand.rs         # Player cards
    ├── opponent.rs     # Hidden opponent
    ├── log.rs          # Combat events
    ├── footer.rs       # Help text
    └── game_over.rs    # Victory screen
```

---

## Main Loop (main.rs)

```
┌─────────────────────────────────┐
│           main()                │
├─────────────────────────────────┤
│  1. setup_terminal()            │
│  2. game.start_new_game()       │
│  3. loop {                      │
│       - draw(|frame| render)     │
│       - poll events             │
│       - handle_key()            │
│     }                           │
│  4. cleanup_terminal()          │
└─────────────────────────────────┘
```

### Key Functions

**handle_key(game: &mut GameSession, key: KeyEvent) -> Option<bool>**

Maps keyboard input to game actions:

| Key | Action |
|-----|--------|
| ← → / h l | Move card selection |
| Space / Enter | Advance to next step |
| Y | Confirm attack |
| N / Esc | Cancel |
| R | Restart (game over) |
| Q | Quit |

**render_game(frame, game, is_player_turn, is_opponent_turn)**

Calls all UI component renderers with proper layout.

---

## UI Components

### ui/mod.rs

Creates layout with **Layout** from ratatui:

```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),   // Header
        Constraint::Length(7),   // Player hand
        Constraint::Min(6),       // Combat log
        Constraint::Length(7),   // Opponent hand
        Constraint::Length(2),   // Footer
        Constraint::Length(1),   // Status
    ])
    .split(frame.area());
```

### ui/hand.rs

Renders player's hand with selection:

- Shows suit symbol (♥ ♦ ♣ ♠)
- Shows rank (2-10, J, Q, K, A)
- Shows HP: current/max
- Highlights selected card with blue background
- Uses `game.selected_player_card.index` for selection

### ui/opponent.rs

**CRITICAL**: Opponent cards are **completely hidden**!

- Shows `[?] ● (alive)` for living cards
- Shows `[X] ✕ (dead)` for dead cards
- NO rank, NO suit, NO HP visible
- Only reveals at game over

### ui/log.rs

Combat event log:
- Scrolls combat history
- Color-coded entries (green for player, gray for opponent)
- Shows damage values, critical hits

### ui/footer.rs

Shows current game state:
- "Game Start"
- "Select Your Card"
- "Confirm Attack"
- etc.

---

## Import from game_core

```rust
use game_core::game_loop::GameStateLoop;
use game_core::game_session::GameSession;

// Or via re-export:
use crate::game_state::{GameStateLoop, GameSession};
```

### Re-export Pattern

`game_state.rs` re-exports to avoid deep paths:

```rust
pub use game_core::game_loop::GameStateLoop;
pub use game_core::game_session::GameSession;
```

---

## Adding New UI Feature

### Step 1: Create New Renderer

```rust
// ui/abilities.rs
pub fn render_abilities(frame: &mut Frame, area: Rect, game: &GameSession) {
    // Render card abilities
}
```

### Step 2: Update Layout

```rust
// ui/mod.rs
.constraints([... , Constraint::Length(3)]) // Add new row
```

### Step 3: Call Renderer

```rust
abilities::render_abilities(frame, chunks[5], game);
```

### Step 4: NEVER Add Game Logic

If you find yourself wanting to modify hand state, combat, or deck - **STOP**!

Move that logic to `game_core` instead:

❌ Bad (in game_tui):
```rust
game.player_hand.cards[idx].hp -= damage;  // Don't do this!
```

✅ Good (in game_tui):
```rust
game.resolve_player_attack(player_idx, opponent_idx);  // Call game_core
```

---

## Debugging UI Issues

### Terminal Too Small

```bash
# Increase terminal size or adjust constraints
Constraint::Length(7) -> Constraint::Length(10)
```

### Cards Not Showing All 5

Check `ui/mod.rs` layout constraints - need at least 6 lines (1 title + 5 cards).

### Selection Not Highlighting

Check `selected_player_card.index` vs `Some(idx)` comparison.

### Input Not Working

Verify state checks in `handle_key()`:
```rust
if game.is_opponent_turn() {
    return None;  // Ignore input during opponent turn
}
```

---

## Test Command

```bash
cd game_tui
cargo test        # Unit + integration tests
cargo test -- --nocapture
```

**Integration Tests** (`tests/integration_tests.rs`):
- Full game flow
- Card damage application
- Win/loss conditions
- Deck shuffling

---

## Migration from Old FullGameState

Before refactoring, game_tui had `FullGameState`:

```rust
// OLD (removed):
game.loop_state = GameStateLoop::SelectOpponentTarget;

// NEW:
game.loop_state = game.loop_state.advance_after_player_card_selected();
```

Key changes:
1. `FullGameState` → `GameSession`  
2. Direct field access → Same (still public fields)  
3. Selection was `Option<usize>` → Now `SelectedCard` struct  
4. Check `game.selected_player_card.index` not `game.selected_player_card`

---

## Ratatui Version

Using **ratatui 0.29** - Important imports:

```rust
// Correct imports:
use ratatui::text::Line;           // NOT widgets::Line!
use ratatui::widgets::Paragraph;
use ratatui::widgets::Block;
use ratatui::style::{Style, Color, Modifier};

// wrap() is gone - use manual text wrapping if needed
```

---

## Replace with Bevy GUI (Future)

When transitioning to graphics:

1. Keep `game_core` unchanged
2. Replace entire `game_tui` with:
   - Bevy `App` with systems
   - Sprite rendering for cards
   - Mouse/touch input handling
   - Same `GameSession` API

The architecture supports this!
