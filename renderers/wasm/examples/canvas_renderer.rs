//! Complete Canvas Renderer Example
//! 
//! This example demonstrates a full Terra-Deck canvas rendering pipeline.
//! 
//! Run with: cargo run --example canvas_renderer

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::f64::consts::PI;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Game configuration
pub struct GameConfig {
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub particle_count: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            tile_size: 32,
            particle_count: 100,
        }
    }
}

/// Terrain tile type
#[derive(Debug, Clone, Copy)]
pub enum TerrainType {
    Grass,
    Sand,
    Water,
    Mountain,
    Forest,
}

impl TerrainType {
    fn color(&self) -> &'static str {
        match self {
            TerrainType::Grass => "rgb(100, 180, 100)",
            TerrainType::Sand => "rgb(240, 230, 150)",
            TerrainType::Water => "rgb(80, 120, 200)",
            TerrainType::Mountain => "rgb(120, 120, 120)",
            TerrainType::Forest => "rgb(60, 100, 60)",
        }
    }
}

/// Particle for visual effects
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub color: f32, // 0-255 for simple color
    pub alpha: f64,
    pub life: f64,
    pub max_life: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64, color: f32) -> Self {
        let angle = (x * y * 0.1) % 360.0 * std::f64::consts::PI / 180.0;
        let speed = 2.0 + (color as f64 * 0.05);
        
        Self {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            radius: 2.0 + color as f64 * 0.05,
            color,
            alpha: 0.5,
            life: 0.0,
            max_life: 100.0 + color as f64 * 0.5,
        }
    }
    
    pub fn update(&mut self, dt: f64) {
        self.life += dt * 60.0;
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        
        // Bounce off walls
        if self.x <= 0.0 || self.x >= 800.0 {
            self.vx = -self.vx;
        }
        if self.y <= 0.0 || self.y >= 600.0 {
            self.vy = -self.vy;
        }
    }
    
    pub fn is_dead(&self) -> bool {
        self.life >= self.max_life
    }
}

/// Terrain map
pub struct TerrainMap {
    tiles: Vec<[TerrainType; 10]>,
    tile_size: u32,
}

impl TerrainMap {
    pub fn new(width_tiles: usize, height_tiles: usize, tile_size: u32) -> Self {
        let mut tiles = Vec::with_capacity(width_tiles * height_tiles);
        
        for row in 0..height_tiles {
            for col in 0..width_tiles {
                // Generate simple terrain based on position
                let mut tile = TerrainType::Grass;
                
                if row as f64 * height_tiles as f64 > 0.7 {
                    tile = TerrainType::Water;
                } else if row as f64 * height_tiles as f64 < 0.2 {
                    tile = TerrainType::Mountain;
                } else if col as f64 * width_tiles as f64 > 0.8 {
                    tile = TerrainType::Forest;
                } else if (row + col) % 3 == 0 {
                    tile = TerrainType::Sand;
                }
                
                tiles.push([tile; 10]); // Each tile has 10 sub-tiles
            }
        }
        
        Self { tiles, tile_size }
    }
    
    pub fn draw(&self, ctx: &CanvasRenderingContext2d, offset_x: f64, offset_y: f64) -> Result<(), wasm_bindgen::JsValue> {
        let height = self.tiles.len();
        let width = if height > 0 { self.tiles[0].len() } else { 0 };
        
        for row in 0..height {
            for col in 0..width {
                let x = offset_x + col as f64 * self.tile_size as f64;
                let y = offset_y + row as f64 * self.tile_size as f64;
                
                self.draw_tile(ctx, x, y, &self.tiles[row])?;
            }
        }
        
        Ok(())
    }
    
    fn draw_tile(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, tile: &[TerrainType]) -> Result<(), wasm_bindgen::JsValue> {
        let width = self.tile_size as f64;
        let height = self.tile_size as f64;
        
        // Draw tile background
        ctx.set_fill_style_with_string(tile[0].color())?;
        ctx.fill_rect(x, y, width, height);
        
        // Draw tile detail
        ctx.set_stroke_style_with_string("rgba(0, 0, 0, 0.2)")?;
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x, y, width, height);
        
        Ok(())
    }
}

/// Fog of war rendering
pub struct FogOfWar {
    visible_rects: Vec<[f64; 4]>,
}

impl FogOfWar {
    pub fn new() -> Self {
        Self {
            visible_rects: Vec::new(),
        }
    }
    
    pub fn add_visible_area(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.visible_rects.push([x, y, w, h]);
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d, width: f64, height: f64) -> Result<(), wasm_bindgen::JsValue> {
        // Fill with dark fog
        ctx.set_fill_style_with_string("rgba(0, 0, 0, 0.7)")?;
        ctx.fill_rect(0.0, 0.0, width, height);
        
        // Cut out visible areas
        ctx.set_composite_operation("destination-out")?;
        
        for &rect in &self.visible_rects {
            // Create gradient effect around edges
            let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
            
            ctx.set_fill_style_with_string("rgba(255, 255, 255, 1.0)")?;
            ctx.fill_rect(x, y, w, h);
        }
        
        ctx.set_composite_operation("source-over")?;
        
        Ok(())
    }
}

/// UI rendering components
pub struct UiRenderer {
    font_size: f64,
}

impl UiRenderer {
    pub fn new() -> Self {
        Self { font_size: 14.0 }
    }
    
    pub fn draw_stats(&self, ctx: &CanvasRenderingContext2d, particles: usize, timestamp: f64) -> Result<(), wasm_bindgen::JsValue> {
        // Draw background panel
        ctx.set_fill_style_with_string("rgba(0, 0, 0, 0.7)")?;
        ctx.fill_rect(10.0, 10.0, 200.0, 100.0);
        
        // Draw border
        ctx.set_stroke_style_with_string("#888888")?;
        ctx.set_line_width(1.0);
        ctx.stroke_rect(10.0, 10.0, 200.0, 100.0);
        
        // Title
        ctx.set_font(&format!("{}px monospace", self.font_size + 2.0))?;
        ctx.set_fill_style_with_string("#a0a0ff")?;
        ctx.fill_text("Terra-Deck Renderer", 15.0, 35.0)?;
        
        // Stats
        ctx.set_font(&format!("{}px monospace", self.font_size))?;
        ctx.set_fill_style_with_string("#ffffff")?;
        
        ctx.fill_text(&format!("Particles: {}", particles), 15.0, 60.0)?;
        ctx.fill_text(&format!("Time: {:.2}s", timestamp), 15.0, 85.0)?;
        
        Ok(())
    }
    
    pub fn draw_button(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64, text: &str, hovered: bool) -> Result<(), wasm_bindgen::JsValue> {
        let bg_color = if hovered { "#5a5a8a" } else { "#4a4a6a" };
        
        // Draw button
        ctx.set_fill_style_with_string(bg_color)?;
        
        // Rounded rectangle
        ctx.begin_path();
        ctx.round_rect(x, y, w, h, 8.0)?;
        ctx.fill();
        
        // Border
        ctx.set_stroke_style_with_string("#6a6a9a")?;
        ctx.set_line_width(2.0);
        ctx.stroke();
        
        // Text
        ctx.set_font(&format!("{}px sans-serif", self.font_size))?;
        ctx.set_fill_style_with_string("#ffffff")?;
        let text_w = text.len() as f64 * 8.0;
        ctx.fill_text(text, x + w / 2.0 - text_w / 2.0, y + h / 2.0 + 4.0)?;
        
        Ok(())
    }
}

/// Main game state
pub struct Game {
    config: GameConfig,
    particles: Vec<Particle>,
    terrain: TerrainMap,
    fog: FogOfWar,
    ui: UiRenderer,
    timestamp: f64,
    camera_x: f64,
    camera_y: f64,
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let terrain_width = config.width / config.tile_size;
        let terrain_height = config.height / config.tile_size;
        
        Self {
            particles: (0..config.particle_count)
                .map(|i| {
                    let x = (i as f64 * 7.3) % config.width as f64;
                    let y = (i as f64 * 5.7) % config.height as f64;
                    let color = (i as f64 * 3.7) % 255.0 as f32;
                    Particle::new(x, y, color)
                })
                .collect(),
            terrain: TerrainMap::new(terrain_width as usize, terrain_height as usize, config.tile_size),
            fog: FogOfWar::new(),
            ui: UiRenderer::new(),
            timestamp: 0.0,
            camera_x: 0.0,
            camera_y: 0.0,
        }
    }
    
    pub fn update(&mut self, dt: f64) {
        self.timestamp += dt;
        
        // Update particles
        self.particles.retain_mut(|p| {
            p.update(dt);
            !p.is_dead()
        });
        
        // Add new particles periodically
        if self.particles.len() < self.config.particle_count && self.timestamp % 0.5 < dt {
            let x = (self.timestamp * 50.0) % self.config.width as f64;
            let y = (self.timestamp * 37.0) % self.config.height as f64;
            let color = (self.particles.len() as f64 * 2.5) % 255.0 as f32;
            self.particles.push(Particle::new(x, y, color));
        }
        
        // Update fog
        self.fog = FogOfWar::new();
        self.fog.add_visible_area(
            self.camera_x,
            self.camera_y,
            self.config.width as f64,
            self.config.height as f64,
        );
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), wasm_bindgen::JsValue> {
        let width = self.config.width as f64;
        let height = self.config.height as f64;
        
        // Clear canvas
        ctx.clear_rect(0.0, 0.0, width, height);
        
        // Save context for transformations
        ctx.save()?;
        
        // Apply camera transformation
        ctx.translate(self.camera_x, self.camera_y)?;
        
        // Draw terrain
        if let Err(e) = self.terrain.draw(ctx, 0.0, 0.0) {
            log(&format!("Failed to draw terrain: {:?}", e));
        }
        
        // Draw particles
        for particle in &self.particles {
            ctx.begin_path()?;
            ctx.arc(particle.x, particle.y, particle.radius, 0.0, PI * 2.0)?;
            
            let color = format!(
                "rgba({},{},{},{:.1})",
                particle.color,
                particle.color * 0.5,
                particle.color * 0.5,
                particle.alpha * (1.0 - particle.life / particle.max_life)
            );
            
            ctx.set_fill_style_with_string(&color)?;
            ctx.fill();
        }
        
        // Restore context
        ctx.restore()?;
        
        // Apply fog of war
        if let Err(e) = self.fog.render(ctx, width, height) {
            log(&format!("Failed to draw fog: {:?}", e));
        }
        
        // Draw UI
        if let Err(e) = self.ui.draw_stats(ctx, self.particles.len(), self.timestamp) {
            log(&format!("Failed to draw UI: {:?}", e));
        }
        
        // Draw a button as demo
        let _ = self.ui.draw_button(ctx, 10.0, 130.0, 120.0, 40.0, "Settings", false);
        let _ = self.ui.draw_button(ctx, 140.0, 130.0, 70.0, 40.0, "Quit", false);
        
        Ok(())
    }
}

/// Canvas application wrapper
#[wasm_bindgen]
pub struct CanvasRenderer {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    game: Game,
}

#[wasm_bindgen]
impl CanvasRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: String) -> Result<CanvasRenderer, wasm_bindgen::JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(&canvas_id)
            .ok_or("Canvas element not found")?
            .dyn_into::<HtmlCanvasElement>()?;
        
        let config = GameConfig::default();
        
        // Set canvas size
        let dpr = web_sys::window().unwrap().device_pixel_ratio() as u32;
        canvas.set_width(config.width * dpr);
        canvas.set_height(config.height * dpr);
        canvas.set_width(config.width * dpr); // Fixed: was setting height again
        canvas.set_style("width", &format!("{}px", config.width));
        canvas.set_style("height", &format!("{}px", config.height));
        
        let ctx = canvas
            .get_context("2d")?
            .ok_or("No 2D context")?
            .unchecked_into::<CanvasRenderingContext2d>();
        
        let game = Game::new(config);
        
        log(&format!("CanvasRenderer initialized: {}x{}", canvas.width(), canvas.height()));
        
        Ok(CanvasRenderer { canvas, ctx, game })
    }
    
    pub fn update(&mut self, delta_time: f64) {
        self.game.update(delta_time);
    }
    
    pub fn render(&self) -> Result<(), wasm_bindgen::JsValue> {
        self.game.render(&self.ctx)
    }
    
    pub fn get_particle_count(&self) -> usize {
        self.game.particles.len()
    }
    
    pub fn on_click(&mut self, x: f64, y: f64) {
        log(&format!("Click at: ({}, {})", x, y));
        
        // Add a burst of particles
        for i in 0..10 {
            let angle = (i as f64 / 10.0) * PI * 2.0;
            let px = x + angle.cos() * 50.0;
            let py = y + angle.sin() * 50.0;
            let color = 200.0 + (i as f64 * 5.0) as f32;
            self.game.particles.push(Particle::new(px, py, color));
        }
    }
}

// Example of running this as a standalone example (not WASM)
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This example is designed for WASM targets.");
    println!("Run with:");
    println!("  cargo run --example canvas_renderer --target wasm32-unknown-unknown");
    println!("\nOr build with wasm-pack and serve the web/ directory:");
    println!("  wasm-pack build --example canvas_renderer --target bundler");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_particle_creation() {
        let particle = Particle::new(100.0, 200.0, 128.0);
        assert!(!particle.is_dead());
    }
    
    #[test]
    fn test_game_initialization() {
        let config = GameConfig::default();
        let game = Game::new(config);
        assert!(game.particles.len() > 0);
    }
}