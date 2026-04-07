# WASM Renderer Implementation Plan

## Overview

This document outlines the implementation of a WebAssembly renderer for Terra-Deck, enabling browser-based gameplay while reusing the existing `game_core` logic.

## Architecture

```
┌─────────────────────────────────────────┐
│           Browser (HTML/JS)            │
│  ← Canvas rendering, Input handling    │
│  (wasm-bindgen, web-sys)               │
├─────────────────────────────────────────┤
│      game_wasm (wasm crate)            │
│  ← WASM bridge layer                   │
│  (exports JS-friendly API)             │
├─────────────────────────────────────────┤
│      game_core (re-export/dependency)  │
│  ← Shared game logic                   │
└─────────────────────────────────────────┘
```

## Key Design Decisions

1. **Keep game_core pure**: No WASM-specific code in core
2. **Bridge layer (game_wasm)**: Exports typed Rust API to JavaScript
3. **Rendering**: HTML5 Canvas (2D context) or DOM elements
4. **Input**: JavaScript event listeners → Rust callbacks
5. **State**: Rust owns state, JS renders each frame

## Implementation Phases

### Phase 1: Foundation (Week 1)
- Create `game_wasm` crate structure
- Setup wasm-bindgen bindings
- Basic HTML/CSS scaffolding
- Cargo-web / wasm-pack integration

### Phase 2: Core Integration (Week 1-2)
- Port GameSession to WASM-friendly API
- Implement turn progression via callbacks
- Canvas 2D rendering for cards
- Deck/Creature visual representation

### Phase 3: UI & Polishing (Week 2-3)
- Card animations (CSS transitions)
- Sound effects via Web Audio API
- Responsive layout
- Touch support for mobile

### Phase 4: Advanced Features (Week 3+)
- LocalStorage for game state
- Keyboard shortcuts
- CSS theming
- Performance optimization

## Technical Details

### Dependencies
```toml
[dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Document", "Window", "CanvasRenderingContext2d"] }
console_error_panic_hook = "0.1"
game_core = { path = "../game_core" }

[lib]
crate-type = ["cdylib"]
```

### Exported API (Rust → JS)
```rust
pub fn init_game() -> usize;
pub fn get_game_state(game_id: usize) -> String;
pub fn play_card(game_id: usize, card_index: usize);
pub fn advance_turn(game_id: usize);
pub fn get_deck_state(game_id: usize) -> String;
pub fn on_frame(game_id: usize) -> JsValue; // For rendering callbacks
```

### HTML Structure
```html
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="main.css">
</head>
<body>
    <div id="game-container">
        <canvas id="game-canvas" width="800" height="600"></canvas>
    </div>
    <script type="module">
        import init, { TerraDeckGame } from './wasm.js';
        init().then(() => {
            const game = new TerraDeckGame();
            // ... setup rendering loop
        });
    </script>
</body>
</html>
```

## Testing Strategy

1. **Unit tests**: Reuse game_core tests (110 existing)
2. **WASM tests**: Headless browser tests via jsdom
3. **Integration**: Manual testing in Chrome/Firefox
4. **E2E**: Playwright tests for critical paths

## Challenges & Solutions

| Challenge | Solution |
|-----------|----------|
| Async/await in JS | Use wasm-bindgen-futures, Promises |
| Canvas performance | RequestAnimationFrame, offscreen canvas |
| Card animations | CSS transforms + requestAnimationFrame |
| State serialization | serde_wasm_bindgen for complex types |
| No std::fs in WASM | Bundle assets or use CDN |

## Files to Create

### Crates
```
renderers/wasm/
├── crate/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs          # Main WASM entry
│   │   ├── game.rs         # Game wrapper with bindings
│   │   ├── renderer.rs     # Canvas rendering
│   │   └── input.rs        # Input handling
├── www/
│   ├── index.html
│   ├── main.css
│   └── main.js
└── README.md
```

### Build Scripts
```
renderers/wasm/
├── build.sh          # wasm-pack build
└── dev.sh            # wasm-pack serve (hot reload)
```

## Success Metrics

- [ ] Game loads in browser in < 2s
- [ ] 60 FPS rendering on mid-range devices
- [ ] All 110 game_core tests still pass
- [ ] Touch controls work on mobile
- [ ] Code coverage > 80% for WASM bridge

## Next Steps

See TODOS.md for actionable items.
