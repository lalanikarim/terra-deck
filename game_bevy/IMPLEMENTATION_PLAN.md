# 🚀 Implementation Plan: Terra-Deck Bevy Engine (`game_bevy`)

## 📋 Overview
This document outlines the architectural design and execution roadmap for the `game_bevy` crate. The goal is to implement a high-fidelity 2D graphical version of Terra-Deck using the **Bevy 0.18** engine, while strictly adhering to the project's core principle: **Separation of Domain Logic from Rendering**.

---

 💡 **Core Principle**: `game_core` remains the "Source of Truth." `game_bevy` is merely a "Visual Observer" that reacts to changes in the `GameSession` state.

## 🏗️ 1. Architectural Design

### 🧬 The "Bridge" Pattern (ECS Integration)
We will use a **Reactive Wrapper** approach to bridge the imperative `game_core` with the declarative Bevy ECS.

**Pseudocode for the Bridge:**
```rust
// Bevy Resource wrapping the core logic
struct GameSessionResource {
    session: GameSession, // The actual game_core object
}

// System: The State Driver
fn game_logic_driver_system(
    mut res: ResMut<GameSessionResource>,
    input_events: Res<BevyInputEvents>, 
) {
    for event in input_events.iter() {
        match event {
            Action::SelectCard(idx) => res.session.select_player_card(idx),
            Action::ConfirmAttack => res.session.resolve_attack(),
            // ... mapping Bevy inputs to game_core methods
        }
    }
}
```

### 🧱 Entity Relationship Model
| Entity | Bevy Components | Description |
| :--- | :--- | :--- |
| **Player Hand Card** | `CardRef(idx)`, `SpriteBundle`, `SelectionHighlight`, `HealthBar` | Visible cards in player's hand. |
| **Opponent Hand Card**| `CardRef(idx)`, `SpriteBundle`, `HiddenTag`, `HealthBar` | Cards with `card_back.png` texture. |
| **Combat FX** | `DamageText`, `Lifetime`, `Velocity` | Floating numbers during hits. |
| **UI Overlay** | `CombatLogUI`, `GameEndOverlay` | Bevy UI Text/Buttons. |

---

## 🎨 2. Visual Strategy

### 🃏 Card Rendering
- **Texture Mapping**: A utility function will map `(Suit, Rank)` $\rightarrow$ `path/to/assets/card_{suit}_{rank}.png`.
- **Fog of War**: 
  - *State: Selecting Target* $\rightarrow$ Render `card_back.png` for opponent cards.
   
### 💥 The "Juice" (Feedback Loops)
We will use **Bevy Events** to trigger visual-only animations:
1. **`CombatHitEvent(damage, position)`**: Spawns a `DamageText` entity and triggers a `CameraShake` component.
2. **`CardDeathEvent(card_entity)`**: Trigggers a "Dissolve" or "Shrink" animation before despawning the entity.
3. **`CriticalHitEvent`**: Spawns a burst of `particle_red.png` around the target.

---

## 🛤️ 3. Implementation Roadmap (Task Breakdown)

### 🛠️ Phase 1: The Foundation (The "Observable" Engine)
*Goal: Get a Bevy window running that mirrors the TUI state.*
- [ ] **Task 1.1: Workspace Setup**: Create `game_bevy` crate and add to `Cargo.toml` workspace.
- [ ] **Task 1.2: Asset Pipeline**: Implement `CardAssetLoader` to pre-cache all Kenney card sprites.
- [ ] **Task 1.3: The Bridge Resource**: Implement `GameSessionResource` and the basic `InputMappingSystem`.
- [ ] **Task 1.4: Primitive Rendering**: Render player cards as simple colored squares (to verify indexing/input works).

### 🃏 Phase 2: Visual Identity (The "Card" Era)
*Goal: Replace primitives with the actual Kenney card assets.*
- [ ] **Task 2.1: Sprite Mapping**: Implement the `Rank/Suit` $\rightarrow$ `TextureHandle` lookup.
- [ ] **Task 2.2: Hand Layout**: Implement "Fan" math to position cards in an arc for the player hand.
- [ ] **Task 2.3: The Fog of War**: Implement the texture swap logic (Back to Front) for opponent cards.
- [ ] **Task 2.4: UI Overlay**: Implement the `CombatLog` using Bevy `TextBundle`.

### 💥 Phase 3: Interaction & Juice (The "Impact" Era)
*Goal: Add animations and feedback.*
- [ ] **Task 3.1: Floating Damage**: Implement `DamageText` entities that move upward and fade out.
- [ ] **Task 3.2: Camera Shake**: Implement a `CameraShake` system triggered by `CombatResult`.
- [ ] **Task 3.3: Selection Feedback**: Add a pulsing golden border to the `SelectedCard`.
- [ ] **Task 3.4: Particle Systems**: Implement simple sprite-based particles for `CriticalHits`.

### 🏆 Phase 4: Polish & Finalization
*Goal: Production-ready game.*
- [ ] **Task 4.1: Game Over States**: Beautiful overlays for Win/Loss/Restart.
- [ ] **Task 4.2: Audio Integration**: Add sound effects for card flips and hits.
- [ ] **Task 4.3: Performance Optimization**: Ensure all textures are in a single `TextureAtlas` if possible.

---

## 📝 Summary of Dependencies
- **`game_core`**: Logic (Input source)
- **`bevy`**: Engine (Rendering/Input/ECS)
- **`kenney_assets`**: Textures/Audio (Visuals)
