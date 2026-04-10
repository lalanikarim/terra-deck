# Poker Card RPG - Task List

All tasks complete! 🎉

---

## ✅ Completed Tasks

### Core Domain
- [x] **Task 1: Combat System Logic**  
  - GameResult enum (Won/Lost/Draw)
  - Hand::remove_dead_cards() method
  - Win/loss condition detection
  - Combat log updates

- [x] **Task 2: Turn Management FSM**
  - GameState methods (is_active, is_player_turn, etc.)
  - SelectedCard resource with navigation methods
  - AI module with random card selection
  - Turn progression systems

### TUI Implementation
- [x] **Task 3: TUI Binary Crate**
  - game_tui/Cargo.toml with ratatui 0.29, crossterm 0.28
  - Standalone terminal app with main loop
  - No Bevy runtime dependency (simplified)

- [x] **Task 4: TUI Rendering Components**
  - ui/header.rs - Title display
  - ui/hand.rs - Player hand with card selection highlight
  - ui/log.rs - Combat log with colored entries
  - ui/footer.rs - Help text display
  - ui/mod.rs - AppUiState and render_game()

- [x] **Task 5: Input Handling**
  - Arrow keys / h l for navigation
  - Space / Enter to play card
  - q / Esc to quit
  - Selection highlighting

- [x] **Task 6: End-to-End Integration Tests**
  - 9 integration tests in game_tui/tests/integration_tests.rs
  - Full game flow from deck creation to combat
  - Win/loss condition testing
  - Combat mechanics verification
  - SelectedCard navigation tests

- [x] **Task 7: Full Game Loop Integration**
  - Three-step combat flow: Select Card → Select Target → Confirm
  - GameStateLoop FSM with 10 states
  - FullGameState struct with deck/shuffle/hands
  - Hidden opponent cards display ([?] ● markers only)
  - Combat resolution with damage calculation
  - Dead card removal
  - Win/loss detection
  - Game over screen with opponent reveal
  - Restart functionality (R key)

### Architecture Refactoring
- [x] **Task 8: Separate Game Logic from UI**
  - Moved GameStateLoop to `game_core/src/game_loop.rs`
  - Created `GameSession` in `game_core/src/game_session.rs`
  - Consolidated combat orchestration in game_core
  - Simplified game_tui to only handle rendering and input
  - Added 19 new tests in game_core
  - 242 lines of game logic moved from UI to core

---

## Test Summary

| Testsuite | Tests | Passing |
|--|-----|----|
| Game Core (card, deck, hand, types, combat, game_loop, game_session) | 110 | ✅ 110 |
| TUI Unit Tests | 3 | ✅ 3 |
| Integration Tests | 9 | ✅ 9 |
| **TOTAL** | **122** | **✅ 122** |

---

## Run the Game

```bash
cargo run --package game_tui
```

### Controls:
- **← →** or **h l** - Navigate cards
- **Space / Enter** - Select card and advance to target selection
- **Y / Enter** - Confirm attack (after selecting card and target)
- **N / Esc** - Cancel and go back
- **R** - Restart after game over
- **q** - Quit

---

## Git History

```
2b4c30a - Step 3: Refactor game_tui to use game_core::GameSession
20661a8 - Step 2: Add GameSession to game_core
0cbf40b - Step 1: Add GameStateLoop enum to game_core
5629057 - Fix UI layout - allocate more space for 5 cards
255bc65 - Update TODOS.md and README.md to reflect Task 7 completion
f79fa10 - Fix Task 7: Complete UI rendering and fix compilation issues
13dd468 - WIP Task 7: Full Game Loop Integration (partially complete)
94a0846 - Complete Task 6: End-to-End integration tests
```

---

## Project Structure

```
bevygame/
├── game_core/          ← Core game logic (110 tests)
│   ├── src/
│   │   ├── ai.rs          ← AI opponent logic
│   │   ├── card.rs        ← Card struct and methods
│   │   ├── combat/        ← Combat mechanics
│   │   ├── combat_log.rs  ← Event logging
│   │   ├── combat_stats.rs← GameState, GameResult
│   │   ├── deck.rs        ← Deck management
│   │   ├── game_loop.rs   ← GameStateLoop FSM (NEW)
│   │   ├── game_session.rs← Full game state management (NEW)
│   │   ├── hand.rs        ← Hand management
│   │   ├── systems.rs     ← Combat systems
│   │   ├── turn_state.rs  ← SelectedCard
│   │   └── types.rs       ← Suit, Rank, Archetype
│   └── Cargo.toml
├── game_tui/           ← Terminal UI (12 tests)
│   ├── src/
│   │   ├── main.rs       ← Terminal setup and input handling
│   │   ├── game_state.rs ← Re-exports from game_core
│   │   └── ui/
│   │       ├── footer.rs
│   │       ├── game_over.rs
│   │       ├── hand.rs
│   │       ├── header.rs
│   │       ├── log.rs
│   │       ├── mod.rs
│   │       └── opponent.rs
│   ├── tests/
│   │   └── integration_tests.rs
│   └── Cargo.toml
├── docs/               ← Knowledge base
│   ├── BEVY_KNOWLEDGE.md
│   ├── CODING_ERRORS.md
│   ├── RAND_KNOWLEDGE.md
│   └── RATATUI_KNOWLEDGE.md
├── TODOS.md            ← This file
└── README.md           ← Project overview
```

---

## Architecture

### Separation of Concerns

**game_core** (Domain Logic):
- All game mechanics and state management
- GameStateLoop finite state machine
- GameSession orchestrates entire game
- Deck, Hand, Card, Combat logic
- **Testable without UI dependencies**

**game_tui** (UI Layer):
- Terminal rendering with ratatui
- Input handling (keyboard)
- Maps user input to game actions
- **Pure UI component - can be replaced with Bevy GUI**

### Design Benefits

✅ **Testable**: All game logic in game_core with 110 unit tests  
✅ **Renderer Agnostic**: Same game_core works with TUI or GUI  
✅ **No Coupling**: UI doesn't know about game mechanics  
✅ **Clean API**: game_tui calls simple methods on GameSession  
✅ **Future Proof**: Easy to add new renderers (Bevy, web, mobile)

---

## Phase 2: WASM Renderer

See [WASM_IMPLEMENTATION.md](./renderers/wasm/WASM_IMPLEMENTATION.md) for full details.

### Phase 2.1: Foundation
- [ ] **Task 9: Create WASM crate structure**
  - Create `renderers/wasm/crate/` with lib.cdylib crate
  - Add wasm-bindgen, web-sys, serde-wasm-bindgen dependencies
  - Setup Cargo.toml with wasm-specific config
  - Write build.sh and dev.sh scripts

- [ ] **Task 10: Setup HTML scaffolding**
  - Create `renderers/wasm/www/` with index.html, main.css, main.js
  - Add Canvas 2D element for game rendering
  - Wire up wasm-bindgen imports
  - Get "Hello World" rendering in browser

### Phase 2.2: Core Integration
- [ ] **Task 11: WASM bridge layer**
  - Expose GameSession API via wasm-bindgen
  - Implement #[wasm_bindgen] annotated functions
  - Handle JsValue ↔ Rust type conversions
  - Add error handling for WASM ↔ JS interop
  
- [ ] **Task 12: Canvas renderer**
  - Implement card rendering on HTML5 Canvas
  - Draw suit symbols (♠♥♣♦) and ranks
  - Render player hand with selection highlight
  - Show opponent cards as hidden ([?] markers)
  - Display combat log as text overlay

### Phase 2.3: Game Loop & Interop
- [ ] **Task 13: RequestAnimationFrame loop**
  - Implement continuous render loop
  - Expose on_frame() callback to JS
  - Sync state efficiently without full redraw
  - Batch canvas updates for performance

- [ ] **Task 14: State serialization**
  - Export GameState to JSON for rendering
  - Use serde-wasm-bindgen for complex types
  - Minimize cross-boundary calls
  - Cache serialized state when possible

### Phase 2.4: Mobile Support
- [ ] **Task 15: Responsive design**
  - Canvas auto-resizes with viewport
  - CSS media queries for tablet/mobile
  - Touch event handlers (tap, swipe)
  - 44px minimum hitboxes for cards

- [ ] **Task 16: Browser compatibility**
  - Test on Chrome, Firefox, Safari
  - Handle iOS Safari quirks
  - Add fallback fonts
  - Optimize for slower 4G connections

### Phase 2.5: Tests & Deployment
- [ ] **Task 17: E2E testing**
  - Playwright tests for game flow
  - Manual testing across browsers
  - Performance benchmarks (60 FPS target)
  - Memory leak testing

- [ ] **Task 18: Production build**
  - wasm-opt optimization
  - Gzip/Brotli compression
  - Deploy to GitHub Pages
  - Add Lighthouse performance score

---

## Phase 3: Future Enhancements (Post-WASM)

- [ ] **Multi-round battles** - Combat continues until both hands empty
- [ ] **Smart AI opponent** - Use combat logic to pick best card each turn
- [ ] **Card drawing between turns** - Pull from deck when hand gets low
- [ ] **Card synergies and abilities** - Special effects for card combos
- [ ] **Persistent save/load** - Resume game later
- [ ] **Transition to Bevy graphics** - Full GUI with sprites and animations
- [ ] **Sound effects** - Combat sounds, card play sounds
- [ ] **Achievements system** - Track wins, combos, perfect games

---

## Phase 4: Bevy Renderer Enhancements

### 4.1: Code Quality & Architecture
- [ ] **Task 19: Centralize Game Configuration**
  - Create `game_bevy/src/config.rs` with constants:
    - `PLAYER_CARD_Y = -150.0`
    - `OPPONENT_CARD_Y = 150.0`
    - `CARD_WIDTH = 100.0`
    - `CARD_HEIGHT = 150.0`
    - `CARD_SPACING = 90.0`
    - `ASSET_PATH_BASE` for kenney textures
  - Update both renderer modules to use constants
  - Add config to `main.rs`

- [ ] **Task 20: Unified Card Rendering Abstraction**
  - Create shared base service for card rendering
  - Extract common fan layout logic (t calculation, X positioning)
  - Create CardBundle component with:
    - Sprite + Transform
    - CardData (suit, rank, health, selected state)
    - CardType (Player/Opponent)
    - CardState (Revealed/Hidden)
  - Both player_renderer and opponent_renderer use this abstraction

- [ ] **Task 21: Card Component Consistency**
  - Create `CardView` component with unified properties:
    - `CardType { player: bool }` - identifies card ownership
    - `CardState { revealed: bool }` - for fog of war logic
    - `CardHealth { current: u32, max: u32 }` - for battle display
  - Replace `PlayerCard` and `OpponentCard` markers with unified component
  - Use component queries with `With<CardView>` instead of separate queries

- [ ] **Task 22: Plugin Structure Refactor**
  - Split rendering into `RenderingPlugin` that combines:
    - `PlayerRendererPlugin`
    - `OpponentRendererPlugin`
    - Optional combat effects plugins (camera shake, particles)
  - Move camera setup to `ViewSetupPlugin`
  - Keep `GameSessionPlugin` for game state
  - Keep `CardAssetsPlugin` for textures
  - Better separation of concerns, easier to test each plugin

- [ ] **Task 23: Configuration Abstraction**
  - Create `AssetPaths` module with constants:
    ```rust
    pub const KENNEY_CARDS_PATH: &str = "kenney_playing-cards-pack/PNG/Cards (large)/";
    pub const CARD_BACK_FILENAME: &str = "card_back.png";
    ```
  - Centralize in one place for easy modification
  - Add error handling for missing assets
  - Make asset loading more flexible (configurable path)

- [ ] **Task 24: Camera Setup Refactor**
  - Replace manual camera spawning with `Camera2d` in `Camera2dBundle`
  - Follow Bevy 0.18 best practices
  - Consider making camera position configurable
  - Add debug visibility for testing camera positioning

- [ ] **Task 25: Texture Handle Optimization**
  - Investigate if `.clone()` on `Handle<Image>` is necessary
  - Profile to confirm if Bevy handle auto-clone on use
  - Remove unnecessary clones if possible
  - Add benchmark between cloned vs non-cloned versions

- [ ] **Task 26: Asset Path Management**
  - Create `AssetPaths` trait or constant module
  - Stringify asset paths in config, avoid hardcoded paths
  - Add validation of asset existence at startup
  - Log missing assets with clear error messages

---

## Phase 5: Visual Feedback & UX Polish

### 5.1: Combat Visual Effects
- [ ] **Task 27: Card Selection Feedback**
  - Add pulsing/glow effect to selected cards
  - Golden border highlight when card selected
  - Animated selection marker
  - Use Bevy animations or sprite scaling

- [ ] **Task 28: Combat Hit Effects**
  - Floating damage numbers when cards take damage
  - Use `DamageText` component with lifecycle animation
  - Spawn upward-moving numbers that fade out
  - Position at target card's coordinates

- [ ] **Task 29: Camera Shake on Combat**
  - Trigger camera shake when combat event occurs
  - Implement shake system with decaying amplitude
  - Configurable shake intensity and duration
  - Use Bevy animation or custom transform update

- [ ] **Task 30: Critical Hit Visuals**
  - Red burst particle effect for critical hits
  - Spawn temporary particle entities
  - Fade and despawn after animation
  - Add to `CardRenderer::CardRendererPlugin`

### 5.2: UI Enhancements
- [ ] **Task 31: Card Health Bars**
  - Display current/max HP above each card
  - Green/Yellow/Red coloring based on HP percentage
  - Update dynamically as cards take damage
  - Component: `HealthBar { current: u32, max: u32, position: Vec3 }`

- [ ] **Task 32: Fog of War Interaction**
  - Reveal opponent cards when targeted
  - Add animation when cards flip (scale from 0 to 1 or rotation)
  - Show full HP bar on revealed cards
  - Add click-to-target interaction system

### 5.3: Game State Display
- [ ] **Task 33: Combat Log Display**
  - Show combat events on screen (side panel)
  - Recent events visible, older fade out
  - Colored by event type (damage, heal, critical)
  - Scrollable if overflow exceeds visible area

- [ ] **Task 34: Turn Indicator**
  - Clear visual indicator of current turn phase
  - "Player Turn" / "Opponent Turn" overlay
  - Color-coded backgrounds or badges
  - Update automatically with GameStateLoop state

- [ ] **Task 35: Game State HUD**
  - Display current game state in HUD
  - Show deck size, remaining cards
  - Player/Opponent card counts
  - Win condition counter

### 5.4: Performance & Polish
- [ ] **Task 36: Texture Atlas Optimization**
  - Texture atlas for all cards (batch rendering)
  - Culling for off-screen cards
  - Optimize asset loading (async, lazy loading)
  - Profile and fix rendering bottlenecks

- [ ] **Task 37: Debug Tools**
  - Toggle overlay for FPS counter
  - Inspector for component inspection
  - Debug draw for hitboxes and positioning
  - Console commands for testing (spawn card, heal, etc.)

- [ ] **Task 38: Sound Effects (Audio Integration)**
  - Add sound for card selection
  - Combat sound effects
  - Win/loss fanfare
  - Integrate with Bevy audio plugin

- [ ] **Task 39: Background & Aesthetics**
  - Add game background (battlefield scene)
  - Card shadow effects
  - Dynamic lighting/ambiance
  - Color theme consistency

---

## Quick Reference Checklists

### Bevy 0.18 API Checklist
- [ ] Camera2d spawned directly (not Camera2dBundle)
- [ ] Sprite::from_image() with texture handle
- [ ] Resources for global state
- [ ] Components for per-entity data
- [ ] Query<Entity, With<T>> for entity selection
- [ ] Transform from_xyz for positioning
- [ ] Z > 0 for visibility (Z=1.0)

### Bevy Plugin Best Practices
- [ ] Plugin structure follows Bevy 0.18 conventions
- [ ] Plugin registration via .add_plugins()
- [ ] Systems organized by schedule (Startup, Update)
- [ ] Resource initialization in plugin build()
- [ ] Proper use of Res/ResMut for resource access

