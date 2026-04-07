# Terra-Deck - Project Context for AI Agents

## Project Overview

**Poker Card RPG (Terra-Deck)** is a turn-based combat game built in Rust using a standard 52-card poker deck. Each suit represents a combat archetype with Rock-Paper-Scissors mechanics plus an Infantry archetype.

**Current State**: Fully functional terminal game with 122 passing tests, clean architecture separation.

---

## Architecture Summary

```
┌─────────────────────────────────────────┐
│              game_tui                  │
│  ← UI Rendering, Input Handling        │
│  (ratatui, crossterm)                  │
└──────────────┬──────────────────────────┘
               │ GameSession (re-export)
               ▼
┌─────────────────────────────────────────┐
│              game_core                  │
│  ← All Game Logic, State Management    │
│  (game_loop, game_session, combat)     │
└─────────────────────────────────────────┘
```

**Key Design Principle**: Complete separation of domain logic from UI. game_core is testable and renderer-agnostic.

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
├── docs/               # Knowledge base (Bevy, Ratatui, Rand)
├── TODOS.md            # Task tracking
└── README.md           # Project overview
