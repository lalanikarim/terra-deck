# Terra-Deck - Project Context for AI Agents

## Project Overview

**Poker Card RPG (Terra-Deck)** is a turn-based combat game built in Rust using a standard 52-card poker deck. Each suit represents a combat archetype with Rock-Paper-Scissors mechanics plus an Infantry archetype.

**Current State**: Fully functional terminal game with 122 passing tests, clean architecture separation.

---

## Architecture Summary

```
┌───────────────────────────────────────────┐
│             game_tui                     │
│  ← UI Rendering, Input Handling          │
│  (ratatui, crossterm)                    │
├───────────────────────┬──────────────────┤
│      GameSession (re-export)             │
└──────────────┬───────────────────────────┘
               │
               ▼
┌───────────────────────────────────────────┐
│             game_core                    │
│  ← All Game Logic, State Management      │
│  (game_loop, game_session, combat)       │
└───────────────────────────────────────────┘
```

**Key Design Principle**: Complete separation of domain logic from UI. game_core is testable and renderer-agnostic.

---

## Guidelines for AI Agents

When working on tasks, follow these strict procedures to ensure project integrity:

1. **Verify after changes**: Whenever code changes are made, or when code is refactored, **run tests** to ensure we have not introduced a regression. Tests should be run after code changes are completed.
2. **Update Documentation**: Once said tasks or sub-tasks are completed, ensure the `TODOS.md` file, any relevant `README.md` files, or knowledge base files (`docs/`) are updated to reflect the new reality.
3. **Commit & Push**: After all updates (code + tests + docs) are finished, commit all the changes to the repo. If a remote ref is already set in the repo, then **push the changes** as well.

---

## Codebase Structure

```
bevygame/
├── game_core/          # Domain logic (110 tests)
│   ├── src/game_loop.rs     # GameStateLoop FSM
│   ├── src/game_session.rs  # Complete game orchestration
│   ├── src/combat/          # Damage calculation
│   ├── src/deck.rs          # Deck/Card management
│   └── ...
├── game_tui/           # Terminal UI (12 tests)
│   ├── src/main.rs        # Terminal loop + input
│   ├── src/ui/            # Component renderers
│   └── tests/             # Integration tests
├── renderers/          # Future renderers
│   ├── bevy/            # 2D graphical frontend (planned)
│   └── wasm/            # Web frontend (planned)
├── docs/               # Knowledge base (Bevy, Ratatui, Rand, Game Loop)
├── TODOS.md            # Task tracking
└── README.md           # Project overview
```

---

## Quick Reference

### Build & Test
```bash
cargo build -p game_core -p game_tui
cargo test           # All 122 tests
```

### Run Game
```bash
cargo run -p game_tui
```

### Key Files to Understand First

1. **game_core/src/game_session.rs** - Complete game state orchestration
2. **game_core/src/game_loop.rs** - Turn progression FSM
3. **game_core/src/combat/mod.rs** - Damage calculation mechanics
4. **game_tui/src/main.rs** - Main loop and input handling

---

## Common Tasks

### Add New Game Feature
1. Implement in `game_core` (logic, tests)
2. Update `GameSession` API if needed
3. Update `game_tui` rendering if UI changes

### Add New Renderer
1. Keep `game_core` unchanged
2. Replace `game_tui` with new renderer
3. Use same `GameSession` API

### Debug Game Logic
```bash
# Run with debug output
RUST_LOG=debug cargo test -p game_core
```

---

## Test Philosophy

- **game_core**: 110 unit tests - pure logic, no UI dependencies
- **game_tui**: 12 tests - UI integration, rendering flow
- All tests colocated with code they test (NO separate tests/ folder in game_core)
- Integration tests in `game_tui/tests/integration_tests.rs`

---

## Next Steps / Future Work

See `TODOS.md` - Next Steps section:
- Smart AI opponent
- Multi-round battles
- Card drawing between turns
- Transition to Bevy graphics (Phase 2)
- Sound effects, achievements
