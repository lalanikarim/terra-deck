# WASM Quick Reference Card

## Command Line Quick Reference

### Installing
```bash
rustup target add wasm32-unknown-unknown
cargo binstall wasm-pack
```

### Building
```bash
# Development
wasm-pack build --target bundler

# Production
wasm-pack build --target bundler --release

# With hot reload
wasm-pack serve --open
```

### Testing
```bash
cargo test --target wasm32-unknown-unknown
```

---

## Canvas Rendering Quick Reference

### Initialize Canvas
```rust
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().unchecked_into();
let ctx = canvas.get_context("2d").unwrap().unwrap().unchecked_into::<CanvasRenderingContext2d>();
```

### Common Draw Calls

#### Clear
```rust
ctx.clear_rect(x, y, width, height);
```

#### Rectangle
```rust
ctx.set_fill_style_with_string("rgb(255, 0, 0)")?;
ctx.fill_rect(x, y, width, height);
```

#### Circle
```rust
ctx.beginPath();
ctx.arc(cx, cy, radius, 0.0, std::f64::consts::PI * 2.0)?;
ctx.fill();
```

#### Line
```rust
ctx.set_stroke_style_with_string("#000000")?;
ctx.set_line_width(2.0);
ctx.beginPath();
ctx.moveTo(x1, y1)?;
ctx.lineTo(x2, y2)?;
ctx.stroke();
```

#### Text
```rust
ctx.set_font("14px sans-serif");
ctx.set_fill_style_with_string("#ffffff")?;
ctx.fill_text("Hello World", x, y)?;
```

### Transformations
```rust
ctx.save()?;
ctx.translate(x, y)?;
ctx.rotate(angle)?;
ctx.scale(sx, sy)?;
// ... draw ...
ctx.restore()?;
```

---

## web-sys Features Reference

Add these features to your Cargo.toml:

```toml
web-sys = { version = "0.3.94", features = [
    // Canvas
    "CanvasRenderingContext2d",
    "HtmlCanvasElement",
    
    // DOM Elements
    "HtmlElement",
    "Document",
    "Window",
    
    // Events
    "MouseEvent",
    "KeyboardEvent",
    "Event",
    
    // Colors
    "RgbColor",
    
    // Other
    "ResizeObserver",
    "DataUrlFmt",
] }
```

---

## Event Handling Reference

### Mouse Events
```rust
fn handle_mouse(e: MouseEvent) {
    let x = e.offset_x() as f64;
    let y = e.offset_y() as f64;
    
    if e.button() == 0 { // Left click
        // Handle click
    }
}

let closure = Closure::wrap(Box::new(handle_mouse));
canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
closure.forget();
```

### Keyboard Events
```rust
fn handle_key(e: KeyboardEvent) {
    match e.key().as_str() {
        "ArrowUp" => move_up(),
        "ArrowDown" => move_down(),
        "Escape" => exit(),
        _ => {}
    }
}
```

### Resize Observer
```rust
let closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
    let entry = entries.get(0).dyn_into::<web_sys::ResizeObserverEntry>().unwrap();
    let content_box = entry.content_rect();
    canvas.set_width(content_box.width() as u32);
    canvas.set_height(content_box.height() as u32);
}));

let observer = ResizeObserver::new(closure.as_ref().unchecked_ref())?;
observer.observe(canvas)?;
```

---

## Async/Await Reference

### RequestAnimationFrame Pattern
```rust
use wasm_bindgen_futures::JsFuture;

async fn request_animation_frame(callback: Box<dyn FnMut()> + 'static) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let callback = Closure::wrap(callback);
    let id = window.request_animation_frame(callback.as_ref().unchecked_ref())?;
    callback.forget();
    Ok(())
}
```

### Fetch Data
```rust
async fn fetch_json(url: &str) -> Result<serde_json::Value, JsValue> {
    let window = web_sys::window().unwrap();
    let response = window.fetch_with_str(url)?.send().await?;
    let json = response.json().await?;
    serde_wasm_bindgen::from_value(json.into())
}
```

---

## Serialization Reference

### Rust → JavaScript
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub health: f32,
}

let player = Player { name: "Hero".to_string(), health: 100.0 };
let js_value = serde_wasm_bindgen::to_value(&player).unwrap();
```

### JavaScript → Rust
```rust
let js_value = get_state_from_js(); // JsValue from Rust
let player: Player = serde_wasm_bindgen::from_value(js_value).unwrap();
```

---

## Performance Tips

### Avoid String Allocation
```rust
// ❌ Bad - allocates every frame
ctx.set_fill_style_with_string(&format!("rgb({},{},{})", r, g, b))?;

// ✅ Good - reuse string
static COLORS: LazyLock<Vec<String>> = LazyLock::new(|| build_colors());
ctx.set_fill_style_with_string(&COLORS[i])?;
```

### Use Offscreen Canvas
```rust
let offscreen = OffscreenCanvas::new(width, height).unwrap();
let ctx = offscreen.get_context("2d").unwrap().unwrap().unchecked_into();
```

### Batch Rendering
```rust
// Clear once per frame
ctx.clear_rect(0.0, 0.0, width, height);

// Draw all entities
for entity in entities.iter() {
    entity.draw(ctx);
}

// Draw UI once after game objects
ui.draw(ctx);
```

---

## Debugging Snippets

### Console Logging
```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

log(&format!("Debug value: {:?}", value));
```

### Profile Rendering
```rust
let start = std::time::Instant::now();
ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
let duration = start.elapsed();
log(&format!("Render took: {:?}", duration));
```

---

## Common Color Formats

| Format | Example |
|--------|---------|
| Named | `"red"`, `"blue"`, `"transparent"` |
| Hex | `"#ff0000"`, `"#f00"` |
| RGB | `"rgb(255, 0, 0)"` |
| RGBA | `"rgba(255, 0, 0, 0.5)"` |

---

## API Patterns

### Constructor
```rust
#[wasm_bindgen(constructor)]
pub fn new(param: String) -> Result<Self, JsValue> {
    // ...
}
```

### Method
```rust
#[wasm_bindgen]
impl MyClass {
    pub fn method(&self, param: f64) -> Result<f64, JsValue> {
        // ...
    }
}
```

### Getter/Setter
```rust
#[wasm_bindgen(getter)]
pub fn value(&self) -> f64 {
    self.value
}

#[wasm_bindgen(setter)]
pub fn set_value(&mut self, value: f64) {
    self.value = value;
}
```

### Async Method
```rust
#[wasm_bindgen]
impl MyClass {
    pub async fn async_method(&self) -> Result<f64, JsValue> {
        // ...
    }
}
```

---

## File References

| File | Purpose |
|------|---------|
| `Cargo.toml` | Dependencies & build config |
| `src/lib.rs` | Main WASM module |
| `web/index.html` | Test page |
| `pkg/` | Compiled output |
| `WASM_KNOWLEDGE_BASE.md` | Full API docs |
| `INTEGRATION_GUIDE.md` | Step-by-step guide |
| `QUICK_REFERENCE.md` | This file |

---

## Useful Links

- **wasm-bindgen**: https://rustwasm.github.io/wasm-bindgen/
- **web-sys**: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
- **wasm-pack**: https://github.com/rustwasm/wasm-pack
- **serde-wasm-bindgen**: https://github.com/rustwasm/serde-wasm-bindgen
- **Rust WASM Book**: https://rustwasm.github.io/docs/book/

---

## License

This Quick Reference is part of the Terra-Deck WASM Renderer.
Licensed under the same terms as the project.
