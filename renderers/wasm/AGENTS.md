# renderers/wasm - Web Assembly Renderer

## Purpose
A web-based frontend for Terra-Deck. This crate will allow players to play the game directly in a browser without installation, using WebAssembly (WASM).

## Role in Architecture
This is a **Renderer-only** crate. It is lightweight and communicates with `game_core` compiled to WASM.

## Implementation Strategy
1. **Compilation**: Compile `game_core` and this crate to `.wasm` using `wasm-bindgen`.
2. **Rendering**: Use HTML5 Canvas or WebGL/WebGPU to render the game state.
3. **Interaction**: Use JavaScript/TypeScript glue code to capture browser `click` and `keydown` events and pass them to the Rust logic.

## Key Responsibilities
- **Web Interface**: Render the game canvas in a browser window.
- **Event Bridging**: Map browser `click` and `keydown` events to `game_core` actions.
- **Assets**: Fetch card textures and audio over HTTP.

## Target Features
- [ ] Playable in modern browsers (Chrome, Firefox, Safari)
- [ ] Zero-install, "click-and-play" experience
- [ ] Responsive design for mobile browsers

## Complexity
**Medium** - Requires understanding of WASM compilation, browser APIs, and JavaScript interop.

## Guidelines

When implementing:
1. Keep `game_core` unchanged - export functions for WASM
2. Use `wasm-bindgen` for Rust ↔ JavaScript communication
3. All game logic should be in `game_core` - only render in the browser
4. Follow the observer pattern - react to state changes from Rust
