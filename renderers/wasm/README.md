# Terra-Deck WASM Canvas Renderer

A Rust-based WebAssembly canvas renderer for the Terra-Deck browser game.

## Overview

This module provides:
- 🎨 Canvas 2D rendering with `web-sys`
- 🦀 Pure Rust game logic with `wasm-bindgen` FFI
- 🎯 Particle system for visual effects
- 🖱️ Mouse event handling
- 📱 High-DPI screen support
- ⚡ Optimized build pipeline with `wasm-pack`

## Project Structure

```
wasm/
├── Cargo.toml              # Rust dependencies & build config
├── src/
│   └── lib.rs             # Main WASM module with canvas renderer
├── web/
│   └── index.html        # HTML shell for testing
└── README.md              # This file
```

## Prerequisites

### Install Rust Toolchain
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version
cargo --version
```

### Install wasm-pack
```bash
# Using cargo-binstall (recommended, faster)
cargo binstall wasm-pack

# Or using cargo install (slower, needs compilation)
cargo install wasm-pack

# Verify installation
wasm-pack --version
```

### Install Node.js (optional, for serving)
```bash
# Install Node.js 18+ from https://nodejs.org/
node --version
npm --version
```

## Quick Start

### 1. Build WASM for Development
```bash
cd /Users/karim/Projects/ocproject/bevygame/renderers/wasm
wasm-pack build --target bundler
```

### 2. Build for Production (Optimized)
```bash
wasm-pack build --target bundler --release
```

### 3. Serve and Test
```bash
# Option 1: Use Python HTTP server
cd web
python3 -m http.server 3000
# Open http://localhost:3000 in your browser

# Option 2: Use Node.js HTTP server
npx http-server ./web -p 3000

# Option 3: Use wasm-pack serve (includes WASM building)
wasm-pack serve --open
```

## Build Targets

### bundler (Recommended)
Produces `.wasm` files bundled with JavaScript glue code, suitable for use with bundlers like webpack or vite.

```bash
wasm-pack build --target bundler
```

### web
Produces `.wasm` files that can be loaded directly into the browser via `<script>` tags.

```bash
wasm-pack build --target web
```

### no-modules
Produces `.wasm` files for environments that don't support ES modules.

```bash
wasm-pack build --target no-modules
```

### nodejs
Produces `.wasm` files for Node.js environments.

```bash
wasm-pack build --target nodejs
```

## Configuration

### Dependencies (Cargo.toml)

```toml
[dependencies]
wasm-bindgen = "0.2.117"              # FFI between Rust and JS
web-sys = {                            # Web API bindings
    version = "0.3.94",
    features = [
        "CanvasRenderingContext2d",
        "HtmlCanvasElement",
        "Window",
        "Document",
        "MouseEvent",
        "KeyboardEvent",
    ]
}
js-sys = "0.3.94"                      # JavaScript interop
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6.5"          # Serialization

[dev-dependencies]
console_error_panic_hook = "0.1.7"    # Panic handling in browser
wasm-bindgen-test = "0.3"             # WASM testing
```

### Features

The `web-sys` crate uses feature flags to include only the APIs you need, reducing bundle size:

- `CanvasRenderingContext2d` - 2D canvas rendering
- `HtmlCanvasElement` - Canvas DOM element
- `Window` - Window object access
- `Document` - Document access
- `MouseEvent` - Mouse event handling
- `RgbColor` - Color object for canvas

## Usage Examples

### Initialize Canvas

```javascript
import init, { CanvasApplication } from '../pkg/terradeck_wasm.js';

await init();
const canvasApp = new CanvasApplication('terradeck-canvas');
```

### Update and Render

```javascript
function gameLoop(currentTime) {
    const deltaTime = (currentTime - lastTime) / 1000.0;
    
    canvasApp.update(deltaTime);
    canvasApp.render();
    
    requestAnimationFrame(gameLoop);
}
```

### Event Handling

Events like mouse click are automatically handled in the Rust code and create particles at the mouse position.

## Canvas API Reference

### Available Methods

| Method | Parameters | Description |
|--------|-----------|-------------|
| `new(canvasId)` | string | Create new canvas application |
| `update(deltaTime)` | f64 | Update game state |
| `render()` | - | Render current frame |
| `on_mouse_down(x, y)` | f64, f64 | Handle mouse down |
| `on_mouse_up()` | - | Handle mouse up |
| `on_mouse_move(x, y)` | f64, f64 | Handle mouse move |

### Canvas Rendering Functions

These are internal methods for drawing:

| Function | Description |
|----------|-------------|
| `draw_rect()` | Draw rectangle |
| `draw_circle()` | Draw circle |
| `draw_line()` | Draw line |
| `draw_particle()` | Draw particle with alpha |
| `draw_ui()` | Draw UI text |

## Performance Tips

### 1. Use Release Builds for Production
```bash
wasm-pack build --target bundler --release
```

### 2. Enable LTO (Link-Time Optimization)
See `Cargo.toml` - LTO is enabled in release profile.

### 3. Use Offscreen Canvas
For complex rendering, consider using offscreen canvas to reduce main thread load.

### 4. Batch Rendering Operations
Minimize context changes by batching similar drawing operations.

### 5. Object Pooling
Re-use objects instead of creating new ones each frame.

## Debugging

### Console Logging
Use the `log()`, `warn()`, and `error()` functions imported from `console`:

```rust
log(&format!("Position: {:?}", particle.pos));
```

### Panic Hook
In debug mode, panics are shown in browser console:

```rust
#[cfg(debug_assertions)]
console_error_panic_hook::set_once();
```

### WASM Bundle Size
Check bundle size:
```bash
ls -lh pkg/
```

## Integration with Terra-Deck

### Next Steps

1. **Terrain Rendering**: Implement sprite-based terrain tiles
2. **Structure Rendering**: Add building/structure sprites
3. **Fog of War**: Implement canvas masking for fog effect
4. **Serialization**: Add save/load using `serde-wasm-bindgen`
5. **Game Loop**: Implement proper async frame loop
6. **Input Handling**: Add keyboard controls
7. **State Management**: Separate game state from rendering

### File Structure for Integration

```
terradeck-wasm/
├── src/
│   ├── lib.rs                    # Main module
│   ├── game.rs                   # Game logic
│   ├── rendering/
│   │   ├── mod.rs               # Rendering module
│   │   ├── canvas.rs            # Canvas rendering
│   │   ├── terrain.rs           # Terrain sprites
│   │   └── fog_of_war.rs        # Fog rendering
│   ├── input/
│   │   ├── mod.rs               # Input handling
│   │   └── events.rs            # Event processing
│   └── serialization.rs          # Save/load
```

## Troubleshooting

### Issue: "Canvas not found"
**Solution**: Ensure canvas element has the correct `id` attribute matching passed string.

### Issue: Blurry canvas on retina displays
**Solution**: High-DPI support is already implemented in the code.

### Issue: Module not loading
**Solution**: Check that you ran `wasm-pack build` and the `pkg/` directory exists.

### Issue: Performance problems
**Solution**: Build with `--release` flag and check for excessive allocations.

## Resources

### Documentation
- [WASM_KNOWLEDGE_BASE.md](./WASM_KNOWLEDGE_BASE.md) - Comprehensive WASM reference
- [wasm-bindgen Guide](https://wasm-bindgen.github.io/wasm-bindgen/)
- [wasm-pack Book](https://rustwasm.github.io/wasm-pack/)

### Crates Documentation
- [wasm-bindgen](https://docs.rs/wasm-bindgen)
- [web-sys](https://docs.rs/web-sys)
- [serde-wasm-bindgen](https://docs.rs/serde-wasm-bindgen)

### Community
- [Rust WASM Discord](https://discord.gg/rustwasm)

## License

MIT License - Same as Terra-Deck project.

---

**Terra-Deck WASM Renderer**  
Part of the OC Project
