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

## Resources
- **Task List**: See [TODOS.md](TODOS.md) for current progress and next steps

## Knowledge Base
- [BEVY_KNOWLEDGE.md](BEVY_KNOWLEDGE.md) - Bevy ECS and system patterns
- [RATATUI_KNOWLEDGE.md](RATATUI_KNOWLEDGE.md) - Terminal UI patterns
- [RAND_KNOWLEDGE.md](RAND_KNOWLEDGE.md) - Random number generation in Rust
