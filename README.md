# Project: Poker Card RPG (Project CodeName: Terra-Deck)

## Overview
An incremental, system-by-system development of a turn-based RPG built using the Bevy game engine and Rust. The game leverages a deck of poker cards to drive combat, where each suit represents a distinct combat archetype.

## Game Description
A card-collection RPG combat game. Players engage in turn-based battles using a deck of standard poker cards. The game starts as a Text User Interface (TUI) using `ratatui` to verify core ECS (Entity Component System) mechanics before transitioning to a full graphical engine.

## Game Mechanics

### 1. The Card System
*   **Suits & Archetypes:**
    *   Each suit is assigned an archetype.
    *   Three suits follow a **Rock-Paper-Scissors** dynamic.
    *   The Fourth suit is **Infantry**, which is vulnerable to all other archetypes.
*   **Rank & Health:**
    *   The face value of the card (2-10, J, Q, K, A) determines the `Hit Points` (HP) of the card during combat.

### 2. Combat Dynamics
Damage is calculated based on the relationship between the attacker's archetype and the defender's archetype:

| Scenario | Damage Multiplier | Special Effect |
| :--- | :--- | :--- |
| **Dominant vs. Lesser** | 0.5x (Reduction) | Chance to **Absorb** (0x damage) |
| **Lesser vs. Dominant** | 2.0x (Increased) | Chance of **Critical Hit** (5.0x damage) |
| **Infantry vs. Any** | 1.0x (Standard) | Chance to **Absorb** (0.25x) OR **Critical Hit** (2.0x) |

### 3. Information Asymmetry (Fog of War)
*   **Player Vision:** The player can only see the cards in their own hand.
*   **Opponent Vision:** The opponent's cards are hidden from the player.
*   **Symmetry:** The opponent is similarly unaware of the player's specific hand composition.

## Architecture

### Core/TUI Split

The project follows a strict **separation of concerns**:

```
┌─────────────────────────────────────────────────────────┐
│                     game_tui                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │   Input     │  │   Rendering │  │   Layout    │    │
│  │  Handling   │  │  (ratatui)  │  │  (Layout)   │    │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘    │
│         │                │                │           │
│         └────────────────┼────────────────┘           │
│                          ▼                            │
│              ┌─────────────────────┐                  │
│              │    GameSession      │                  │
│              │  (Re-exported from  │                  │
│              │    game_core)       │                  │
│              └─────────────────────┘                  │
└─────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                    game_core                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │  GameLoop   │  │  GameLogic  │  │  Combat     │    │
│  │   (FSM)     │  │  (Session)  │  │  (Engine)   │    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │   Deck      │  │    Hand     │  │     Card    │    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────────────────────────────────────────────────┘
```

**game_core**: All game mechanics, state management, combat logic (110 tests)  
**game_tui**: Terminal UI rendering, input handling (12 tests)

### Key Components

**game_core/src/**
- `game_loop.rs` - GameStateLoop FSM for turn progression
- `game_session.rs` - Complete game state orchestration
- `combat/mod.rs` - Damage calculation with archetypes
- `deck.rs`, `hand.rs`, `card.rs` - Card management
- `turn_state.rs` - SelectedCard tracking

**game_tui/src/**
- `main.rs` - Terminal setup and main loop
- `ui/` - Rendering modules (header, hand, opponent, log, footer)

### Benefits

✅ **Testable**: All game logic in game_core with 110 unit tests  
✅ **Renderer Agnostic**: Same game_core works with TUI or GUI  
✅ **No Coupling**: UI doesn't know about game mechanics  
✅ **Clean API**: game_tui calls simple methods on GameSession  
✅ **Future Proof**: Easy to add new renderers (Bevy, web, mobile)

---

## Development Strategy

The project follows a strict **System-by-System Verification** approach:
1.  **Core/TUI Split:** The domain logic (ECS, Combat, Turn FSM) decoupled from rendering layer.
2.  **Phase 1 (TUI):** Implemented using `ratatui` to ensure all mechanics work in text environment.
3.  **Phase 2 (Graphics):** Transition the rendering layer to Bevy's 2D/3D capabilities, reusing game_core.

---

## Documentation

- **Task List**: See [TODOS.md](TODOS.md) for detailed task breakdown and progress
- Knowledge base in `docs/` folder:
  - [docs/BEVY_KNOWLEDGE.md](docs/BEVY_KNOWLEDGE.md) - Bevy ECS and system patterns
  - [docs/RATATUI_KNOWLEDGE.md](docs/RATATUI_KNOWLEDGE.md) - Terminal UI patterns
  - [docs/RAND_KNOWLEDGE.md](docs/RAND_KNOWLEDGE.md) - Random number generation in Rust
  - [docs/CODING_ERRORS.md](docs/CODING_ERRORS.md) - Common errors and solutions

---

## ✅ All Complete!

All 8 development tasks completed including architecture refactoring:

- ✅ **Tasks 1-7**: Core game mechanics and gameplay implementation
- ✅ **Task 8**: Architecture refactoring - Separated game logic from UI

The game is fully functional as a terminal-based RPG with:
- Full deck management (52 cards, shuffled)
- Three-step combat flow
- Hidden opponent mechanics (fog of war)
- Game over with opponent reveal
- **122 passing tests** (110 core + 12 TUI)

---

## Quick Start

### Build and Run
```bash
# Build
cargo build --release -p game_tui

# Run
cargo run --package game_tui
```

### Controls
- **← →** / **h l** - Navigate cards
- **Space / Enter** - Confirm selection
- **Y** - Confirm attack
- **N / Esc** - Cancel
- **R** - Restart after game over
- **Q** - Quit

### Tests
```bash
# Core tests
cargo test -p game_core

# TUI tests  
cargo test -p game_tui

# All tests
cargo test
```
