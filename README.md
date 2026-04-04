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
    *   The fourth suit is **Infantry**, which is vulnerable to all other archetypes.
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

## Development Strategy
The project follows a strict **System-by-System Verification** approach:
1.  **Core/TUI Split:** The domain logic (ECS, Combat, Turn FSM) will be decoupled from the rendering layer.
2.  **Phase 1 (TUI):** Implement the game using `ratatui` to ensure all mechanics work in a headless/text environment.
3.  **Phase 2 (Graphics):** Transition the rendering layer to Bevy's 2D/3D capabilities, reusing the core domain logic.

## Project Todo List

- [x] **Project Scaffolding:** Initialize cargo project with core/tui split and dependencies (Bevy, Ratatui)
- [x] **Domain: Define Card, Suit, and Rank components and enums**
- [x] **Domain: Implement Deck generation and shuffling logic**
- [ ] **Domain: Implement Combat Engine (Damage multiplier and probability logic)**
- [ ] **Domain: Implement Turn Management FSM (States: Player, Opponent, Combat, GameOver)**
- [ ] **TUI: Implement Basic Terminal Rendering (Displaying Hand and Hidden Opponent)**
- [ ] **TUI: Implement Input Handling (Selecting cards and triggering turns)**
- [ ] **TUI: Implement Combat Log/Event Stream rendering**
- [ ] **Integration: End-to-end playable loop testing in TUI**

## Current Progress

### Completed Systems

#### 1. Project Scaffolding
- Initialized workspace with `game_core` (library) and `game_tui` (binary)
- Configured dependencies: Bevy 0.18.1, ratatui 0.30.0

#### 2. Core Domain - Card System
**Implemented in:** `game_core/src/lib.rs`

**Components:**
- `Suit`: Enum with Hearts (Rock), Diamonds (Paper), Clubs (Scissors), Spades (Infantry)
- `Archetype`: Combat archetype relationships
- `Rank`: Enum from 2 to Ace (2-14)
- `Card`: Entity with suit, rank, current HP, and max HP

**Resources:**
- `Deck`: Vector of Card with methods for creation, drawing, and shuffling
- `Hand`: Player hand management with add/play methods
- `GameState`: Turn management FSM (PlayerTurn, OpponentTurn, CombatResolution, GameOver)
- `CombatLog`: Event logging system

**Verified Functionality:**
- Deck generation creates 52 unique cards
- Card creation properly initializes HP from rank values
- Card take_damage and heal methods work correctly

### Technical Notes & Hurdles

#### Known Issues:
1. **Random number generation**: Initially struggled with `rand` crate imports. Resolved by using `thread_rng()` from `prelude`.
2. **Typo resolution**: Initial implementation had `Infary` instead of `Infantry` in archetype mapping.

#### Architectural Decisions:
- **Randomness deferred**: Card shuffling will be implemented later with proper random number generation
- **ECS-first approach**: All game state stored as Bevy Resources, no non-Bevy state
- **Separation of concerns**: Core domain (`game_core`) has no UI dependencies

#### Next Steps:
1. Implement combat damage calculation systems
2. Add turn management logic
3. Build TUI layer with ratatui

## Knowledge Base

### Bevy Knowledge (BEVY_KNOWLEDGE.md)
### Ratatui Knowledge (RATATUI_KNOWLEDGE.md)
