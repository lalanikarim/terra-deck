# Terra-Deck WASM Integration Guide

This guide walks you through integrating the WASM canvas renderer into the Terra-Deck browser game.

---

## 🎯 Step 1: Environment Setup

### Verify Prerequisites

```bash
# Check Rust installation
rustc --version
# Should show Rust 1.71+

# Check WASM target
rustup target list --installed | grep wasm
# Should show: wasm32-unknown-unknown

# Check wasm-pack
wasm-pack --version
# Should show: 0.14.0+
```

### Install Missing Dependencies

```bash
# Install WASM target if missing
rustup target add wasm32-unknown-unknown

# Install wasm-pack if missing
cargo binstall wasm-pack

# Install Node.js if missing
# Download from https://nodejs.org/
```

---

## 📦 Step 2: Building the WASM Module

### Quick Build Command

From the `bevygame/renderers/wasm` directory:

```bash
cd /Users/karim/Projects/ocproject/bevygame/renderers/wasm

# Development build
wasm-pack build --target bundler

# Production build (optimized)
wasm-pack build --target bundler --release
```

### Understanding Build Output

After running `wasm-pack build`, you'll get:

```
pkg/
├── terradeck_wasm_bg.wasm      # WASM binary
├── terradeck_wasm.js          # JavaScript glue code
├── terradeck_wasm.d.ts        # TypeScript definitions
├── terradeck_wasm_bg.wasm.d.ts
├── package.json
└── .gitignore
```

### Build Flags Reference

| Flag | Description |
|------|--|
| `--target bundler` | Bundle for web bundlers (webpack, vite) |
| `--target web` | Load via script tag |
| `--target nodejs` | For Node.js environment |
| `--target no-modules` | No ES modules support |
| `--release` | Optimized build |
| `--dev` | Debug build with source maps |

---

## 🧪 Step 3: Testing the Renderer

### Method 1: Using wasm-pack serve

```bash
wasm-pack serve ./wasm --open
```

This will:
- Build the WASM module
- Start a local web server
- Open the browser automatically
- Auto-rebuild on file changes

### Method 2: Manual HTTP Server

```bash
# Build first
wasm-pack build --target bundler --release

# Serve with Python
cd web && python3 -m http.server 3000

# Or with Node.js
npx http-server ./web -p 3000
```

Then open: `http://localhost:3000`

### Method 3: Using Vite (Recommended for Production)

Create a `vite.config.js`:

```javascript
import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    port: 3000,
    open: true,
  },
  optimizeDeps: {
    exclude: ['terradeck-wasm'],
  },
});
```

Run: `npx vite ./web`

---

## 🔌 Step 4: JavaScript Integration

### Basic Integration

```javascript
// Import WASM module
import init, { CanvasApplication } from '../pkg/terradeck_wasm.js';

async function initGame() {
  // Initialize WASM runtime
  await init();
  
  // Create canvas application
  const canvasApp = new CanvasApplication('game-canvas');
  
  // Start game loop
  let lastTime = performance.now();
  
  function gameLoop(currentTime) {
    const deltaTime = (currentTime - lastTime) / 1000.0;
    lastTime = currentTime;
    
    canvasApp.update(deltaTime);
    canvasApp.render();
    
    requestAnimationFrame(gameLoop);
  }
  
  requestAnimationFrame(gameLoop);
}

initGame();
```

### With TypeScript Support

The `pkg/terradeck_wasm.d.ts` provides TypeScript definitions:

```typescript
import init, { CanvasApplication } from '../pkg/terradeck_wasm';

const canvasApp: CanvasApplication = new CanvasApplication('game');
canvasApp.update(0.16);
canvasApp.render();
```

---

## 🎮 Step 5: Game Loop Implementation

### Optimized Frame Loop

```javascript
// Game loop with delta time calculation
let lastFrameTime = 0;
const TARGET_FPS = 60;
const FRAME_INTERVAL = 1000 / TARGET_FPS;
let accumulator = 0;

function gameLoop(currentTime) {
  const deltaTime = currentTime - lastFrameTime;
  lastFrameTime = currentTime;
  
  accumulator += deltaTime;
  
  // Update game state at fixed timestep
  while (accumulator >= FRAME_INTERVAL) {
    canvasApp.update(FRAME_INTERVAL / 1000.0);
    accumulator -= FRAME_INTERVAL;
  }
  
  // Render every frame
  canvasApp.render();
  
  requestAnimationFrame(gameLoop);
}

requestAnimationFrame(gameLoop);
```

### Async Game State Loading

```javascript
async function loadGameState(stateData) {
  // Send state data to Rust
  await canvasApp.loadState(stateData);
  
  // Start game loop after loading
  requestAnimationFrame(gameLoop);
}
```

---

## 💾 Step 6: Serialization & Save/Load

### Save Game State

```javascript
function saveGame() {
  // Get state from Rust
  const state = canvasApp.getState();
  
  // Serialize to JSON
  const json = JSON.stringify(state);
  
  // Store in localStorage
  localStorage.setItem('terradeck_save', json);
  
  console.log('Game saved');
}
```

### Load Game State

```javascript
async function loadGame() {
  try {
    const json = localStorage.getItem('terradeck_save');
    if (!json) {
      console.log('No save found');
      return;
    }
    
    const state = JSON.parse(json);
    await canvasApp.loadState(state);
    
    console.log('Game loaded');
  } catch (error) {
    console.error('Failed to load game:', error);
  }
}
```

---

## 🎨 Step 7: Canvas Rendering Integration

### Using the Render Module

```rust
use crate::canvases::*;

pub fn render_frame(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    // Clear canvas
    clear_rect(ctx, 0.0, 0.0, width as f64, height as f64);
    
    // Draw terrain
    draw_terrain(ctx, 0.0, 0.0)?;
    
    // Draw structures
    draw_structures(ctx)?;
    
    // Apply fog of war
    apply_fog_mask(ctx, width as f64, height as f64)?;
    
    // Draw UI
    draw_ui(ctx, width as f64, height as f64)?;
    
    Ok(())
}
```

### Custom Rendering Pipeline

```rust
pub struct RenderPipeline {
    background: Renderer,
    terrain: Renderer,
    entities: Renderer,
    fog: FogRenderer,
    ui: UiRenderer,
}

impl RenderPipeline {
    pub fn render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.background.render(ctx)?;
        self.terrain.render(ctx)?;
        self.entities.render(ctx)?;
        self.fog.render(ctx)?;
        self.ui.render(ctx)?;
        Ok(())
    }
}
```

---

## ⚡ Step 8: Event Handling Integration

### Enhanced Event Handlers

```javascript
// Setup event listeners
function setupEventListeners(canvasApp) {
  const canvas = document.getElementById('game-canvas');
  
  // Mouse events
  canvas.addEventListener('mousedown', (e) => {
    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) * (canvas.width / rect.width);
    const y = (e.clientY - rect.top) * (canvas.height / rect.height);
    canvasApp.on_mouse_down(x, y);
  });
  
  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) * (canvas.width / rect.width);
    const y = (e.clientY - rect.top) * (canvas.height / rect.height);
    canvasApp.on_mouse_move(x, y);
  });
  
  canvas.addEventListener('mouseup', () => {
    canvasApp.on_mouse_up();
  });
  
  // Keyboard events
  document.addEventListener('keydown', (e) => {
    canvasApp.handle_keydown(e.key);
  });
  
  document.addEventListener('keyup', (e) => {
    canvasApp.handle_keyup(e.key);
  });
  
  // Window resize
  window.addEventListener('resize', () => {
    canvasApp.resize();
  });
}

setupEventListeners(canvasApp);
```

---

## 🎯 Step 9: Production Deployment

### Build for Production

```bash
# Create optimized build
wasm-pack build --target bundler --release

# Bundle size analysis
ls -lh pkg/*.wasm
```

### Optimize WASM Size

```bash
# Add to .cargo/config.toml
[target.wasm32-unknown-unknown]
rustflags = ["-C", "opt-level=s", "-C", "lto=fat"]

# Rebuild
wasm-pack build --target bundler --release
```

### Additional WASM Optimization

Use `wasm-opt` for further optimization:

```bash
# Install binaryen
npm install -g binaryen

# Optimize the WASM
wasm-opt -O4 pkg/terradeck_wasm_bg.wasm -o pkg/terradeck_wasm_bg.wasm
```

---

## 🔍 Step 10: Debugging & Profiling

### Browser DevTools

1. Open Chrome DevTools
2. Go to Sources tab → WASM module
3. Set breakpoints in WASM code
4. Use Performance tab for profiling

### Console Logging

```rust
// In Rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

log(&format!("Debug: position = {:?}", pos));
```

### Performance Monitoring

```javascript
// FPS counter
let frameCount = 0;
let lastTime = performance.now();

function monitor_fps(time) {
  frameCount++;
  
  if (time - lastTime >= 1000) {
    console.log(`FPS: ${frameCount}`);
    frameCount = 0;
    lastTime = time;
  }
  
  requestAnimationFrame(monitor_fps);
}

requestAnimationFrame(monitor_fps);
```

---

## 📚 Step 11: Advanced Features

### Offscreen Canvas

```rust
pub struct OffscreenCanvas {
    canvas: web_sys::HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl OffscreenCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        let canvas = web_sys::HtmlCanvasElement::new().unwrap();
        canvas.set_width(width);
        canvas.set_height(height);
        
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into::<CanvasRenderingContext2d>();
        
        Self { canvas, ctx }
    }
    
    pub fn to_data_url(&self) -> String {
        self.canvas.to_data_url_with_format(&web_sys::DataUrlFmt::Png)
    }
}
```

### Image Loading

```rust
pub fn load_image(url: &str) -> Result<web_sys::HtmlImageElement, JsValue> {
    let img = web_sys::HtmlImageElement::new()?;
    img.set_src(url);
    
    // Wait for image to load
    Ok(img)
}
```

### Texture Atlasing

```rust
pub struct TextureAtlas {
    tiles: Vec<web_sys::HtmlImageElement>,
    tile_size: (u32, u32),
}

impl TextureAtlas {
    pub fn draw_tile(&self, ctx: &CanvasRenderingContext2d, tile_index: usize, x: f64, y: f64) -> Result<(), JsValue> {
        if tile_index >= self.tiles.len() {
            return Ok(());
        }
        
        let tile = &self.tiles[tile_index];
        let (width, height) = self.tile_size;
        
        ctx.draw_image_with_html_image_element_and_sw_and_sh(
            tile,
            0.0,
            0.0,
            width as f64,
            height as f64,
            x,
            y,
            width as f64,
            height as f64,
        )?;
        
        Ok(())
    }
}
```

---

## 🧹 Step 12: Cleaning Up

### Memory Management

```rust
pub fn cleanup(&mut self) {
    // Clear game state
    self.game_state.particles.clear();
    
    // Clear event listeners (managed by Rust ownership)
    // WASM memory will be garbage collected when module is unloaded
    
    log("Cleanup complete");
}
```

### Module Unloading

```javascript
function unloadModule() {
  // Stop game loop
  cancelAnimationFrame(animationFrameId);
  
  // Cleanup canvas application
  canvasApp.cleanup();
  
  // Remove from DOM
  canvasApp = null;
}
```

---

## 🎓 Next Steps

Now that you have the WASM renderer integrated:

1. **Add Terrain Rendering**: Implement tile-based terrain
2. **Add Structures**: Render buildings and units
3. **Implement Fog of War**: Create visibility masks
4. **Add Animation**: Smooth transitions and effects
5. **Implement Sound**: Use Web Audio API
6. **Add UI Components**: Menus and HUD
7. **Add Networking**: WebSocket for multiplayer
8. **Add Particles**: Enhanced visual effects

---

## 📖 Additional Resources

- [WASM_KNOWLEDGE_BASE.md](./WASM_KNOWLEDGE_BASE.md) - Comprehensive API reference
- [README.md](./README.md) - Project overview
- [wasm-bindgen Guide](https://wasm-bindgen.github.io/wasm-bindgen/)
- [rustwasm Book](https://rustwasm.github.io/docs/book/)
- [wasm-pack Book](https://rustwasm.github.io/wasm-pack/)

---

**Last Updated**: 2025-03-06  
**Terra-Deck WASM Integration**: Phase 1 Complete
