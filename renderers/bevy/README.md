# game_bevy - 2D Graphical Renderer

## Purpose
A high-fidelity 2D graphical frontend for Terra-Deck using the **Bevy** game engine. This crate will "observe" the `game_core` session and render it using sprites, animations, and particle effects.

## Role in Architecture
This is a **Renderer-only** crate. It should contain zero game logic. All combat rules, card-handling, and win/loss conditions are handled by `game_core`.

## Key Responsibilities
- **Rendering**: Render `Card` entities using textures and sprites.
- **Animations**: Handle combat animations (damage numbers, card movement, particle effects for Critical Hits).
- **Input Translation**: Map mouse/keyboard inputs to `game_primitives` (e.g., clicking a card calls `game_session.selected_player_card = Some(idx)`).
- **Asset Management**: Manage textures, fonts, and sound effects.

## Implementation Strategy
1. **Dependency**: Import `game_core` as a library.
2. **System Setup**: Create a Bevy `Plugin` that manages the `GameSession` resource.
3. **Observe State**: Use Bevy systems to monitor changes in `GameSession` (e.g., when `combat_log` changes, spawn a text entity).

## Target Features
- [ ] Sprite-based cards with flip animations
- [ ] Screen shake on critical hits
- [ ] Particle systems for "Absorb" and "Critical" effects
- [ ] Smooth UI overlays using `bevy_ui`
