# Terra-Deck - Project Context for AI Agents

## Project Overview

**Poker Card RPG (Terra-Deck)** is a turn-based combat game built in Rust using a standard 52-card poker deck. Each suit represents a combat archetype with Rock-Paper-Scissors mechanics plus an Infantry archetype.

**Current State**: Fully functional terminal game with 122 passing tests, clean architecture separation.

---

## Architecture Summary

```
┌────────────────────────────────────────┐
│             game_tui                   │
│  ← UI Rendering, Input Handling        │
│  (ratatui, crossterm)                  │
├──────────┬─────────────────────────────┤
│          │    GameSession (re-export)  │
└──────────┴─────────────────────────────┘
           │
           ▼
┌────────────────────────────────────────┐
│             game_core                  │
│  ← All Game Logic, State Management    │
│  (game_loop, game_session, combat)     │
└────────────────────────────────────────┘
```

**Key Design Principle**: Complete separation of domain logic from UI. game_core is testable and renderer-agnostic.

---

## Guidelines for AI Agents

When working on tasks, follow these strict procedures to ensure project integrity:

1. **Verify after changes**: Whenever code changes are made, or when code is refactored, **run tests** to ensure we have not introduced a regression. Tests should be run after code changes are completed.

2. **Update Documentation**: Once said tasks or sub-tasks are completed, ensure the `TODOS.md` file, any relevant `README.md` files, or knowledge base files (`docs/`) are updated to reflect the new reality.

3. **Commit & Push**: After all updates (code + tests + docs) are finished, commit all the changes to the repo. If a remote ref is already set in the repo, then **push the changes** as well.

4. **DO NOT USE sed for complex edits**: 
   - ❌ Avoid `sed` for multi-line file edits (especially on macOS where sed behavior differs)
   - ❌ Avoid `sed -i ''` with quoted patterns (macOS sed requires this syntax but is error-prone)
   - ✅ Use `write` tool for file creation/modification instead
   - ✅ Use `read` tool to examine file contents first
   - ✅ For simple single-line replacements, consider using the `edit` tool with targeted text replacement
   
   **Reason**: `sed` commands often fail silently or produce unexpected results on macOS. This has caused broken builds multiple times. Always use the file tools (`read`, `write`, `edit`) instead.

---

## Codebase Structure

```
bevygame/
├── game_core/          # Domain logic (110 tests)
│   ├── src/game_loop.rs     # GameStateLoop FSM
│   ├── src/game_session.rs  # Complete game orchestration
│   ├── src/card.rs          # Card entity with HP
│   ├── src/types.rs         # Suit, Rank enums
│   ├── src/deck.rs          # Deck generation
│   ├── src/hand.rs          # Hand management
│   ├── src/combat/mod.rs    # Damage calculation
│   └── src/systems.rs       # Bevy ECS (conditional)
├── game_tui/            # Terminal UI (ratatui)
├── renderers/wasm/      # WASM Canvas renderer
│   ├── src/lib.rs         # Main WASM application
│   ├── src/canvases/      # Canvas rendering utilities
│   └── web/index.html     # Test page
└── docs/                # Documentation
    ├── ARCHITECTURE.md    # Design documents
    └── ...
```

---

## Testing Guidelines

### game_core tests
```bash
cargo test -p game_core        # Run all core tests (110 passing)
cargo test -p game_core --lib  # Run lib tests only
cargo test -p game_core card   # Run tests matching "card"
```

### game_tui tests
```bash
cargo test -p game_tui         # Run all TUI tests
```

### WASM render tests
```bash
cd renderers/wasm
wasm-pack build --target web   # Build for browser
npx http-server -p 8000 web    # Serve test page
```

---

## Common Commands

| Task | Command |
|------|---------|
| Run all tests | `cargo test` |
| Run formatted tests | `cargo test --pretty` |
| Check formatting | `cargo fmt --check` |
| Format code | `cargo fmt` |
| Build WASM | `wasm-pack build --target web` |
| Clean build | `cargo clean && cargo build` |
| Release build | `cargo build --release` |

---

## WASM Renderer (New Feature)

The WASM renderer is a browser-based card battle interface using Canvas 2D API:

**Features**:
- Player cards (face-up) at bottom
- Opponent cards (face-down) at top
- HP bars showing current/max HP above cards
- Combat effects: damage numbers, shake animations, critical hits
- Combat log on left side
- Click-to-attack interaction

**Build & Test**:
```bash
cd renderers/wasm
wasm-pack build --target web
npx http-server -p 8000 web
# Open http://localhost:8000
```

**Known**: Works with standalone card system. Game core integration pending via bridge layer.

---

## Recent Changes

- [c42d192] WASM canvas renderer added
- [ce8c8b0] Future renderer folders created
- [2fa9001] Game loop architecture documentation
- Card HP system implemented
- Combat log display
- Performance optimizations (no terrain tiles)