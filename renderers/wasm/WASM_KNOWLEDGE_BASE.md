# WASM Rust Knowledge Base for Terra-Deck

This document consolidates the latest information on Rust WebAssembly tooling for building the Terra-Deck browser game renderer.

---

## 📦 Latest Crates & Versions (2025)

| Crate | Latest Version | Purpose | Documentation |
|-------|----------------|---------|---------------|
| `wasm-bindgen` | 0.2.117 | FFI between Rust and JavaScript | [docs.rs](https://docs.rs/wasm-bindgen) |
| `web-sys` | 0.3.94 | Web APIs bindings (DOM, Canvas, Events) | [docs.rs](https://docs.rs/web-sys) |
| `js-sys` | 0.3.94 | Low-level JavaScript interop | [docs.rs](https://docs.rs/js-sys) |
| `wasm-bindgen-futures` | 0.4.67 | Async/await support for Web APIs | [docs.rs](https://docs.rs/wasm-bindgen-futures) |
| `serde-wasm-bindgen` | 0.6.5 | Serialization for WASM context | [docs.rs](https://docs.rs/serde-wasm-bindgen) |
| `console_error_panic_hook` | 0.1.7 | Panic handling in browser (dev) | [docs.rs](https://docs.rs/console_error_panic_hook) |
| `wasm-pack` | 0.14.0 | Build tool for WASM packages | [Book](https://rustwasm.github.io/wasm-pack/) |

### MSRV (Minimum Supported Rust Version)
- **Libraries**: Rust 1.71
- **CLI Tools**: Rust 1.82

---

## 🏗️ Architecture Patterns

### 1. Standard Project Layout

```
rustwasm-canvas/
├── Cargo.toml
├── wasm/
│   └── hello_world/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs      # Core game logic
│       │   └── canvas.rs   # Canvas rendering
├── web/
│   ├── index.html          # HTML shell
│   └── package.json        # Node.js tooling
└── README.md
```

### 2. Core Components

#### lib.rs
```rust
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent};
use std::f64::consts::PI;
use winit::event_loop::EventLoop;

#[wasm_bindgen]
pub struct CanvasApp {
    canvas: HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    width: u32,
    height: u32,
    game_state: GameState,
}
```

#### GameState
```rust
pub struct GameState {
    particles: Vec<Particle>,
    time: f64,
    input: InputState,
}

pub struct Particle {
    pos: [f64; 2],
    vel: [f64; 2],
    color: [f32; 4], // RGBA
    radius: f64,
}
```

---

## 🎨 Canvas Rendering with web-sys

### 1. Canvas Initialization

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Window, HtmlCanvasElement, CanvasRenderingContext2d};

impl CanvasApp {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let document = web_sys::window()
            .ok_or("no global window")?
            .document()
            .ok_or("no document")?;

        let canvas: HtmlCanvasElement = document
            .get_element_by_id(canvas_id)
            .ok_or("canvas not found")?
            .unchecked_into();

        canvas.set_width(800);
        canvas.set_height(600);

        let ctx = canvas
            .get_context("2d")?
            .ok_or("no 2d context")?
            .unchecked_into::<CanvasRenderingContext2d>();

        Ok(Self {
            canvas,
            ctx,
            width: 800,
            height: 600,
            game_state: GameState::default(),
        })
    }
}
```

### 2. Clearing Canvas
```rust
let ctx = &self.ctx;
ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
```

### 3. Drawing Rectangles
```rust
use web_sys::CanvasRenderingContext2d;

pub fn draw_rect(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: &str,
) -> Result<(), JsValue> {
    ctx.set_fill_style_with_string(color)?;
    ctx.fill_rect(x, y, width, height);
    Ok(())
}
```

### 4. Drawing Circles
```rust
pub fn draw_circle(
    ctx: &CanvasRenderingContext2d,
    cx: f64,
    cy: f64,
    radius: f64,
    color: &str,
) -> Result<(), JsValue> {
    ctx.begin_path();
    ctx.arc(cx, cy, radius, 0.0, 2.0 * PI)?;
    ctx.set_fill_style_with_string(color)?;
    ctx.fill();
    Ok(())
}
```

### 5. Drawing Lines
```rust
pub fn draw_line(
    ctx: &CanvasRenderingContext2d,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: &str,
    width: f64,
) -> Result<(), JsValue> {
    ctx.set_stroke_style_with_string(color)?;
    ctx.set_line_width(width);
    ctx.beginPath();
    ctx.line_to(x1, y1)?;
    ctx.line_to(x2, y2)?;
    ctx.stroke();
    Ok(())
}
```

### 6. Saving/Restoring State
```rust
// Before transformation
self.ctx.save();

// Apply transformations
self.ctx.translate(x, y)?;
self.ctx.rotate(angle)?;

// Draw object
draw_object(&self.ctx);

// Restore previous state
self.ctx.restore();
```

### 7. Setting Styling Properties
```rust
// Fill style (color)
ctx.set_fill_style_with_string("rgb(255, 0, 0)")?;
// or with color object
let rgb = RgbColor::rgba(255, 0, 0, 1.0);
ctx.set_fill_style(&rgb)?;

// Stroke style
ctx.set_stroke_style_with_string("#000000")?;

// Line width
ctx.set_line_width(2.0);

// Line join
ctx.set_line_join_with_str("round")?;

// Font
ctx.set_font("14px sans-serif");
```

---

## 💾 Serialization & Data Types

### 1. Converting Rust to JS

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    health: f32,
    position: [f32; 2],
}

pub fn send_to_js(player: &Player) -> js_sys::Object {
    wasm_bindgen::JsCast::unchecked_from_js_value(
        serde_wasm_bindgen::to_value(&player).unwrap()
    )
}
```

### 2. Common Type Mappings
| Rust | JavaScript | Notes |
|------|-----------|-------|
| `String` | `string` | UTF-8 automatically |
| `i32`, `u32`, `f32`, `f64` | `number` | Numeric types |
| `bool` | `boolean` | Booleans |
| `&str` | `string` | String slices |
| `Vec<T>` | `array` | For serializable types |
| `Option<T>` | `T \| undefined` | Null handling |
| `Cow<'a, str>` | `string` | Borrowed strings |

### 3. Custom Type Conversion
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Point")]
    pub type JsPoint;
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
}
```

---

## 🔄 Async/Await Patterns

### 1. Using RequestAnimationFrame

```rust
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;
use std::cell::Cell;

impl CanvasApp {
    pub fn start_loop(&self, frame: fn(&CanvasApp)) {
        let this = Rc::new(Cell::new(self.clone()));
        
        loop {
            let frame = Rc::clone(&frame);
            let this = Rc::clone(&this);
            
            // Call rendering logic
            // Note: Actual frame loop implementation needs proper closure handling
        }
    }
}
```

### 2. Example Frame Loop
```rust
use std::time::Instant;

pub fn run_frame_loop(canvas_app: Rc<CanvasApp>) {
    let mut last_time = Instant::now();
    
    let window = web_sys::window().unwrap();
    
    // Use a recursive requestAnimationFrame or web_sys animation frame API
    // This is simplified - actual implementation needs proper async handling
}

impl CanvasApp {
    pub fn update(&mut self, delta_ms: f64) {
        // Update game state
        self.game_state.particles.iter_mut().for_each(|p| {
            p.pos[0] += p.vel[0] * delta_ms as f64;
            p.pos[1] += p.vel[1] * delta_ms as f64;
        });
    }
    
    pub fn render(&self) {
        // Clear canvas
        // Draw game state
    }
}
```

### 3. Async Web APIs Example (Fetch)
```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Uint8Array;

async fn fetch_data(url: String) -> Result<UInt8Array, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = window.fetch_with_str(&url)?.send().await?;
    let resp: web_sys::Response = JsCast::from_js_value(resp_value)?;
    let buf = resp.array_buffer().await?;
    Ok(Uint8Array::new(&buf?))
}
```

---

## 🎯 Event Handling

### 1. Mouse Events
```rust
use web_sys::{MouseEvent, Element};

impl CanvasApp {
    pub fn setup_event_handlers(&self) {
        let canvas_clone = self.canvas.clone();
        
        let closure = Closure::wrap(Box::new(move |e: MouseEvent| {
            let x = e.offset_x();
            let y = e.offset_y();
            // Handle mouse interaction
        }) as Box<dyn FnMut(_)>);
        
        canvas_clone.add_event_listener_with_callback(
            "mousedown",
            closure.as_ref().unchecked_ref()
        ).unwrap();
        
        closure.forget(); // Prevent closure from being dropped
    }
}
```

### 2. Keyboard Events
```rust
use web_sys::{KeyboardEvent};

let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
    println!("Key: {}, Code: {}", e.key(), e.code());
    self.game_state.input.handle_keydown(&e);
}) as Box<dyn FnMut(_)>);

document.add_event_listener_with_callback(
    "keydown",
    closure.as_ref().unchecked_ref()
).unwrap();
```

### 3. Window Resize
```rust
use web_sys::{ResizeObserver, ResizeObservation};

pub fn setup_resize_listener(canvas: &HtmlCanvasElement) {
    let closure = Closure::wrap(Box::new(move |_entries: js_sys::Array| {
        // Adjust canvas size
        let window = web_sys::window().unwrap();
        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
    }) as Box<dyn FnMut(_)>);
    
    // Use ResizeObserver or window.onresize
    let resize_observer = ResizeObserver::new(&Closure::wrap(Box::new(move |entries: js_sys::Array| {
        // Handle resize
    }) as Box<dyn FnMut(_)>))?;
    
    resize_observer.observe_with_options(canvas, None).unwrap();
}
```

---

## 🛠️ Build Tooling

### 1. wasm-pack Commands

```bash
# Build for production (optimized)
wasm-pack build --target bundler --release

# Build for development (debug + source maps)
wasm-pack build --target bundler

# Build for Node.js
wasm-pack build --target nodejs

# Build for web workers
wasm-pack build --target no-modules

# Build and pack for npm
wasm-pack build --dev

# Clean build artifacts
wasm-pkg clean --release

# Publish to npm (packaged with name)
wasm-pack publish --registry https://registry.npmjs.org
```

### 2. wasm-bindgen CLI (Alternative)
```bash
# Build to specific directory
wasm-bindgen --out-dir pkg --target bundler target/wasm32-unknown-unknown/release/*.wasm

# Generate TypeScript definitions
wasm-bindgen --out-name types --out-dir pkg
```

### 3. Recommended .cargo/config.toml
```toml
[target.wasm32-unknown-unknown]
rustflags = [
    "-C", "link-arg=--export-table",
    "-C", "link-arg=--export=memory",
]
```

### 4. Package.json Scripts
```json
{
  "scripts": {
    "build:wasm": "wasm-pack build --target bundler --release",
    "build:wasm:dev": "wasm-pack build --target bundler",
    "serve": "npx parcel serve index.html",
    "start": "npm run build:wasm:dev && npm run serve"
  },
  "dependencies": {
    "terradeck-wasm": "file:./pkg"
  }
}
```

---

## 🧪 Testing & Debugging

### 1. Dev Tools Setup
```toml
# Cargo.toml dependencies
[dependencies]
console_error_panic_hook = "0.1.7"

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

### 2. Panic Hook (Dev Only)
```rust
#[cfg(debug_assertions)]
pub fn panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(not(debug_assertions))]
pub fn panic_hook() {}
```

### 3. Browser Test Configuration
```rust
use wasm_bindgen_test::*;

#[wasm_bindgen_test::wasm_bindgen_test]
fn test_game_state() {
    let mut game = GameState::default();
    assert!(game.particles.is_empty());
    // Add test logic
}
```

### 4. Console Logging
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

// Usage
log(&format!("Position: {:?}", particle.pos));
```

### 5. Debug Info Build Flag
```bash
wasm-pack build --dev --features debug
```

```toml
[features]
debug = ["wasm-bindgen/xxx_debug_only_print_generated_code"]
```

---

## 🚀 Performance Optimizations

### 1. Memory Management
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl CanvasApp {
    pub fn cleanup(&mut self) {
        // Explicitly clear data structures
        self.game_state.particles.clear();
        
        // Drop closures if no longer needed
        // (wasm-bindgen handles lifecycle automatically)
    }
}
```

### 2. Buffer Pooling Pattern
```rust
use std::collections::VecDeque;

pub struct RenderContext {
    draw_commands: VecDeque<DrawCommand>,
    dirty_rects: Vec<DirtyRect>,
    // Pool memory for common operations
}

impl RenderContext {
    pub fn init_pool(&mut self) {
        // Pre-allocate common buffer sizes
        self.draw_commands.shrink_to_fit();
    }
}
```

### 3. Avoid String Allocation in Loop
```rust
// ❌ Bad: Creates new string each frame
ctx.set_fill_style_with_string(&format!("rgb({},{},{})", r, g, b))?;

// ✅ Good: Reuse color object or use numeric API
let color = web_sys::RgbColor::rgba(r as f64, g as f64, b as f64, 1.0);
ctx.set_fill_style(&color);
```

### 4. Minimize JS ↔ Rust Calls
```rust
// ❌ Bad: Many small calls
for particle in &self.particles {
    draw_particle(ctx, particle)?;
}

// ✅ Better: Batch operations
ctx.save()?;

let particles = self.game_state.particles.as_slice();

// Batch rendering
for particle in particles {
    // Minimal context change
    ctx.translate(particle.pos[0], particle.pos[1])?;
    // Draw
}

ctx.restore();
```

### 5. Use Typed Arrays
```rust
use js_sys::{Float32Array, Int32Array};

pub fn get_particles_data(particles: &[Particle]) -> Vec<f32> {
    let mut data = Vec::with_capacity(particles.len() * 4);
    
    for p in particles {
        data.push(p.pos[0] as f32);
        data.push(p.pos[1] as f32);
        data.push(p.vel[0] as f32);
        data.push(p.vel[1] as f32);
    }
    
    data
}
```

---

## ⚠️ Common Pitfalls & Solutions

### 1. Panic in Release
**Problem**: Panics are silenced in production builds.

**Solution**:
```toml
# Cargo.toml
[profile.release]
lto = true
opt-level = "s"
# Don't strip debug info in dev
debug = true
```

### 2. Memory Leaks
**Problem**: Closures and event handlers not being cleaned up.

**Solution**:
```rust
// Track closure references
self.handlers.push(Box::new(closure));

// Clear on cleanup
for handler in &self.handlers {
    handler.cancel();
}
```

### 3. Incorrect Canvas Coordinates
**Problem**: Offset coordinates vs screen coordinates.

**Solution**:
```rust
let bounds = canvas.get_element_bounding_client_rect()?;
let x = event.offset_x() - bounds.x();
let y = event.offset_y() - bounds.y();
```

### 4. High-DPI Screens
**Problem**: Canvas looks blurred on retina displays.

**Solution**:
```rust
impl CanvasApp {
    pub fn set_canvas_size(&self, width: u32, height: u32) {
        // Get device pixel ratio
        let dpr = web_sys::window()
            .unwrap()
            .device_pixel_ratio() as u32;
        
        canvas.set_width(width * dpr);
        canvas.set_height(height * dpr);
        canvas.set_style("width", &format!("{}px", width));
        canvas.set_style("height", &format!("{}px", height));
        
        // Scale context
        self.ctx.scale(dpr as f64, 1.0);
    }
}
```

### 5. Async/await in Main Thread
**Problem**: Blocking the main thread.

**Solution**:
```rust
// Use web_sys animation frame API properly
pub fn request_animation_frame(callback: Box<dyn FnMut() + 'static>) {
    let window = web_sys::window().unwrap();
    
    // Use requestAnimationFrame via web-sys or similar
    // This is more complex - see web_sys documentation
}
```

### 6. Large Payload Size
**Problem**: Unoptimized WASM bundles bloated.

**Solution**:
```bash
# Enable WasmOpt for further optimization
wasm-pack build --release
wasm-opt input.wasm -O4 -o output.wasm

# Use wasm-pack with optimization flags
wasm-pack build --target bundler \
  -- --release \
  --rustc-cfg \
    wasm-bindgen_unstable_debug \
  --lto
```

---

## 📚 Resource Links

### Official Documentation
- [wasm-bindgen Guide](https://wasm-bindgen.github.io/wasm-bindgen/)
- [wasm-pack Book](https://rustwasm.github.io/wasm-pack/)
- [The Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [std::web Guide](https://rustwasm.github.io/docs/std/)

### Crates Documentation
| Crate | Docs.rs |
|-------|---------|
| wasm-bindgen | [docs.rs/wasm-bindgen](https://docs.rs/wasm-bindgen) |
| web-sys | [docs.rs/web-sys](https://docs.rs/web-sys) |
| js-sys | [docs.rs/js-sys](https://docs.rs/js-sys) |
| wasm-bindgen-futures | [docs.rs/wasm-bindgen-futures](https://docs.rs/wasm-bindgen-futures) |
| serde-wasm-bindgen | [docs.rs/serde-wasm-bindgen](https://docs.rs/serde-wasm-bindgen) |

### Example Projects
- [rustwasm/wasm-bindgen Examples](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys/examples)
- [wasm-pack Examples](https://github.com/rustwasm/wasm-pack/tree/master/examples)
- [The Bevy Engine](https://github.com/bevyengine/bevy) - Game engine with WASM support

### Discord & Community
- [Rust WASM Discord](https://discord.gg/rustwasm)
- [WASM-Bindgen Discord Channel](https://discord.gg/xMZ7CCY)

---

## 🎮 Terra-Deck Integration Notes

### Canvas Renderer Structure
For Terra-Deck's canvas-based rendering:

1. **Use web-sys Canvas APIs** for 2D rendering
2. **Implement particle systems** for dust and fog of war effects
3. **Use off-screen canvas** for sprite rendering (terrain, structures)
4. **Implement delta time** for smooth animation
5. **Use offscreen canvas pooling** for performance

### Key Features to Implement
- [ ] Canvas-based particle renderer
- [ ] Terrain rendering with sprite tiles
- [ ] Fog of war with canvas masking
- [ ] Off-screen canvas pooling for sprites
- [ ] Delta-time based game loop
- [ ] WASM serialization for save/load
- [ ] Event handling for player input
- [ ] High-DPI screen support

### Recommended Dependencies
```toml
[dependencies]
wasm-bindgen = "0.2.117"
web-sys = { version = "0.3.94", features = [
    "CanvasRenderingContext2d",
    "HtmlCanvasElement",
    "Window",
    "Document",
    "MouseEvent",
    "KeyboardEvent",
] }
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6.5"

[dev-dependencies]
console_error_panic_hook = "0.1.7"
wasm-bindgen-test = "0.3"
```

---

## 📝 Code Examples Reference

See `/code_examples/` folder for:
- Canvas rendering examples
- Particle system examples
- Event handling setup
- Serialization examples
- Async patterns

---

**Last Updated**: 2025-03-06  
**Terra-Deck WASM Integration**: Phase 1

---

This knowledge base will be updated as we discover new patterns, optimize performance, and integrate additional features for Terra-Deck's browser renderer.
---

## 🎮 Bevy WASM Support

**Status**: ✅ **Officially Supported** (Bevy 0.13+)

Bevy has native WASM support since version 0.13, making it possible to run full Bevy applications in the browser.

### Bevy WASM Target

```bash
# Compile Bevy for WASM
cargo build --target wasm32-unknown-unknown

# For Terra-Deck with proper features
cargo build --target wasm32-unknown-unknown --features web
```

### Bevy WASM Features

`bevy` provides platform-specific feature flags for WASM:

| Feature | Description | Platforms |
|---------|-----|-----|
| `web` | Browser APIs and WebGL/WebGPU | `wasm32-unknown-unknown` only |
| `webgl` | WebGL rendering (fallback) | WASM with older browsers |
| `webgpu` | WebGPU rendering (preferred) | WASM with modern browsers |
| `default` | Full Bevy (includes Vulkan) | Native targets only |

### Bevy WASM Cargo.toml Configuration

**Option A: Platform-Specific**
```toml
# game_core/Cargo.toml
[dependencies]
bevy = { version = "0.18", default-features = false }

# Platform-specific configuration
[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy = { version = "0.18", default-features = false, features = [
    "web",
    "bevy_window",
    "bevy_render",
] }

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
bevy = { version = "0.18", default-features = true }
```

**Option B: Feature Flags**
```toml
[features]
default = ["bevy/full-features"]
web = ["bevy/web"]  # For WASM
native = ["bevy/full-features"]  # For native
```

### Bevy ECS in WASM

**What Works:**
- ✅ ECS `Component`, `Resource`, `System` traits
- ✅ Game loop and state management
- ✅ Asset loading (with WebGPU)
- ✅ Rendering (WebGL/WebGPU)
- ✅ 2D rendering pipeline

**What to Avoid:**
- ❌ Platform-specific backends (Vulkan)
- ❌ File system I/O (use wasm-bindgen)
- ❌ Threading (use async/await in WASM)
- ❌ Audio (limited WASM support)

### Bevy vs. Pure WASM Rendering Comparison

| Metric | Bevy WASM | Pure web-sys |
|--------|-----|-----|
| **Bundle Size** | ~3-5 MB | ~50-100 KB |
| **Game Loop** | Built-in | Manual (RAF) |
| **ECS** | Full ECS | None |
| **Rendering** | WebGL/WebGPU | Canvas 2D |
| **Features** | Complete engine | Minimal |
| **Complexity** | High | Low |

### When to Use Which

| Use Case | Recommended | Reason |
|----------|--|-----|
| Complex 3D games | Bevy WASM | Full engine capabilities |
| Terminal-based game (TUI) | `game_tui` (ratatui) | No Bevy needed |
| Simple 2D browser game | Pure `web-sys` + Canvas | Lightweight |
| Performance-critical | Pure `web-sys` | Minimal overhead |
| ECS-based architecture | Bevy WASM | Already using Bevy |

### Example: Pure ECS vs. Full Engine

```rust
// Pure ECS (game_core - Bevy traits only)
use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct Player { hp: u32 }

impl Player {
    pub fn new(hp: u32) -> Self {
        Self { hp }
    }
}

// Full Bevy engine (bevygame - Desktop)
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_system)
        .run();
}
```

### TUI Compatibility Check

**Important:** `game_tui` uses `ratatui`, not Bevy, so they are completely independent:

**Current Architecture:**
```toml
# game_tui/Cargo.toml
# Uses ratatui + crossterm (terminal UI)
ratatui = "0.26"
crossterm = "0.27"

# game_core/Cargo.toml
# Uses Bevy ECS traits only (Component, Resource)
bevy = { version = "0.18", default-features = false }

# renderers/wasm/Cargo.toml
# Uses web-sys + wasm-bindgen (browser rendering)
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["CanvasRenderingContext2d"] }
```

**Key Points:**
- ✅ `game_tui` has **no** Bevy dependency
- ✅ `game_core` uses **only** ECS traits (no rendering)
- ✅ WASM renderer uses **pure** web-sys (no Bevy)
- ✅ All three can coexist without breaking each other

**Adding Web Feature Won't Break TUI:**
```toml
# Adding this to game_core will NOT affect game_tui:
[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy = { version = "0.18", features = ["web"] }
```

Because `game_tui` doesn't use Bevy at all! 🎉

### Future Integration Path

If you want to integrate `game_core` with WASM:

1. **Strip Bevy from game_core** (current approach - cleanest)
```rust
// Remove:
use bevy::prelude::Component;

// Replace with:
#[derive(Clone, Debug)]
pub struct Player { hp: u32 }
```

2. **Or use platform-specific Bevy**
```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy = { version = "0.18", features = ["web"], default-features = false }
```

### Bevy WASM Build Size Optimization

```toml
[profile.release]
opt-level = "s"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization

[target.'cfg(target_arch = "wasm32")'.rustflags]
rustflags = [
    "-C", "link-arg=--no-entry",
    "-C", "link-arg=--export-table"
]
```

---

