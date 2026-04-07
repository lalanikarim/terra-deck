//! Terrain system for Terra-Deck WASM renderer
//!
//! Provides tile-based terrain rendering with multiple biomes.

use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;

/// Terrain tile types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerrainType {
    Grass,
    Water,
    Mountain,
    Desert,
    Snow,
    Forest,
    Road,
    Unknown,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType::Grass
    }
}

impl TerrainType {
    /// Get the fill color for this terrain type
    pub fn fill_color(&self) -> &'static str {
        match self {
            TerrainType::Grass => "#4a7c4a",
            TerrainType::Water => "#4a7cb5",
            TerrainType::Mountain => "#8b7355",
            TerrainType::Desert => "#e6c288",
            TerrainType::Snow => "#d4d8d8",
            TerrainType::Forest => "#2d5a2d",
            TerrainType::Road => "#5a5a5a",
            TerrainType::Unknown => "#333333",
        }
    }

    /// Get the border/shadow color for this terrain type
    pub fn border_color(&self) -> &'static str {
        match self {
            TerrainType::Grass => "#3a5c3a",
            TerrainType::Water => "#3a5c85",
            TerrainType::Mountain => "#6b5335",
            TerrainType::Desert => "#c6a268",
            TerrainType::Snow => "#b4b8b8",
            TerrainType::Forest => "#1d3a1d",
            TerrainType::Road => "#4a4a4a",
            TerrainType::Unknown => "#222222",
        }
    }
}

/// A single terrain tile
#[derive(Debug, Clone)]
pub struct TerrainTile {
    pub terrain_type: TerrainType,
    pub x: u32,
    pub y: u32,
    pub tile_size: u32,
}

impl TerrainTile {
    pub fn new(x: u32, y: u32, terrain_type: TerrainType, tile_size: u32) -> Self {
        Self {
            x,
            y,
            terrain_type,
            tile_size,
        }
    }

    /// Render this tile on the canvas
    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        let px = (self.x as f64) * (self.tile_size as f64);
        let py = (self.y as f64) * (self.tile_size as f64);
        let ts = self.tile_size as f64;

        // Draw tile background
        ctx.set_fill_style_str(self.terrain_type.fill_color());
        ctx.fill_rect(px, py, ts, ts);

        // Draw tile border
        ctx.set_stroke_style_str(self.terrain_type.border_color());
        ctx.set_line_width(1.0);
        ctx.stroke_rect(px + 0.5, py + 0.5, ts - 1.0, ts - 1.0);

        // Draw terrain-specific decorations
        match self.terrain_type {
            TerrainType::Water => self.render_water(&px, &py, &ts, ctx),
            TerrainType::Mountain => self.render_mountain(&px, &py, &ts, ctx),
            TerrainType::Forest => self.render_forest(&px, &py, &ts, ctx),
            TerrainType::Desert => self.render_desert(&px, &py, &ts, ctx),
            TerrainType::Snow => self.render_snow(&px, &py, &ts, ctx),
            _ => {}
        }
    }

    fn render_water(&self, px: &f64, py: &f64, ts: &f64, ctx: &mut CanvasRenderingContext2d) {
        // Simple wave pattern
        ctx.begin_path();
        ctx.set_stroke_style_str("#6a9cc5");
        ctx.set_line_width(2.0);
        let cx = px + *ts / 2.0;
        let cy = py + *ts / 2.0;
        ctx.arc(cx, cy, *ts / 6.0, 0.0, PI * 2.0);
        ctx.stroke();
    }

    fn render_mountain(&self, px: &f64, py: &f64, ts: &f64, ctx: &mut CanvasRenderingContext2d) {
        // Triangle mountain peak
        ctx.begin_path();
        ctx.set_stroke_style_str("#6b5335");
        ctx.set_line_width(2.0);
        let cx = px + *ts / 2.0;
        ctx.move_to(cx - *ts / 4.0, *py + *ts - 5.0);
        ctx.line_to(cx, *py + 10.0);
        ctx.line_to(cx + *ts / 4.0, *py + *ts - 5.0);
        ctx.stroke();
    }

    fn render_forest(&self, px: &f64, py: &f64, ts: &f64, ctx: &mut CanvasRenderingContext2d) {
        // Tree trunk
        ctx.set_fill_style_str("#4a3728");
        ctx.set_line_width(4.0);
        let cx = px + *ts / 2.0;
        let cy = py + *ts / 2.0;
        ctx.stroke_rect(cx - 2.0, cy, 4.0, *ts / 4.0);

        // Tree foliage
        ctx.set_fill_style_str("#2d5a2d");
        ctx.arc(cx, cy, *ts / 3.0, 0.0, PI * 2.0);
        ctx.fill();
    }

    fn render_desert(&self, px: &f64, py: &f64, ts: &f64, ctx: &mut CanvasRenderingContext2d) {
        // Sand dune effect
        ctx.begin_path();
        ctx.set_stroke_style_str("#d6b278");
        ctx.set_line_width(2.0);
        let cx = px + *ts / 2.0;
        let cy = py + *ts / 2.0;
        ctx.arc(cx, cy, *ts / 5.0, 0.0, PI);
        ctx.stroke();
    }

    fn render_snow(&self, px: &f64, py: &f64, ts: &f64, ctx: &mut CanvasRenderingContext2d) {
        // Snowflake dots
        ctx.set_fill_style_str("#e4e8e8");
        ctx.set_line_width(1.0);
        let cx = px + *ts / 2.0;
        let cy = py + *ts / 2.0;
        
        // Small dots for snow effect
        ctx.arc(cx - 5.0, cy - 5.0, 2.0, 0.0, PI * 2.0);
        ctx.fill();
        ctx.arc(cx + 5.0, cy + 5.0, 2.0, 0.0, PI * 2.0);
        ctx.fill();
        ctx.arc(cx, cy - 8.0, 2.0, 0.0, PI * 2.0);
        ctx.fill();
    }
}

/// Terrain grid/map
#[derive(Debug, Clone, Default)]
pub struct TerrainMap {
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub tiles: Vec<Vec<TerrainType>>,
}

impl TerrainMap {
    pub fn new(width: u32, height: u32, tile_size: u32) -> Self {
        let tiles = vec![vec![TerrainType::Grass; width as usize]; height as usize];
        Self {
            width,
            height,
            tile_size,
            tiles,
        }
    }

    /// Get terrain at position (x, y)
    pub fn get_terrain(&self, x: u32, y: u32) -> TerrainType {
        if x < self.width && y < self.height {
            self.tiles[y as usize][x as usize]
        } else {
            TerrainType::Unknown
        }
    }

    /// Set terrain at position (x, y)
    pub fn set_terrain(&mut self, x: u32, y: u32, terrain_type: TerrainType) {
        if x < self.width && y < self.height {
            self.tiles[y as usize][x as usize] = terrain_type;
        }
    }

    /// Generate a simple test terrain pattern
    pub fn generate_test_terrain(&mut self) {
        // Fill with grass by default
        for y in 0..self.height {
            for x in 0..self.width {
                let terrain_type = if y < self.height / 4 {
                    // Top quarter: snow
                    TerrainType::Snow
                } else if y < 2 * self.height / 5 {
                    // Second section: forest
                    TerrainType::Forest
                } else if y < 3 * self.height / 5 {
                    // Middle: grass
                    TerrainType::Grass
                } else if y < 4 * self.height / 5 {
                    // Lower third: desert
                    TerrainType::Desert
                } else {
                    // Bottom: water
                    TerrainType::Water
                };

                self.set_terrain(x, y, terrain_type);
            }
        }

        // Add some mountains in the middle
        for x in self.width / 4..3 * self.width / 4 {
            if x % 2 == 0 {
                self.set_terrain(x, self.height / 2, TerrainType::Mountain);
            }
        }

        // Add a road through the middle
        for y in (self.height - self.height / 6)..(self.height / 6 + self.height / 2) {
            self.set_terrain(self.width / 2, y, TerrainType::Road);
        }
    }

    /// Render the entire terrain map
    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        for y in 0..self.height {
            for x in 0..self.width {
                let terrain_type = self.get_terrain(x, y);
                let tile = TerrainTile {
                    x,
                    y,
                    terrain_type,
                    tile_size: self.tile_size,
                };
                tile.render(ctx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_map_creation() {
        let mut map = TerrainMap::new(10, 10, 32);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.tile_size, 32);
        assert_eq!(map.get_terrain(0, 0), TerrainType::Grass);
    }

    #[test]
    fn test_set_terrain() {
        let mut map = TerrainMap::new(10, 10, 32);
        map.set_terrain(5, 5, TerrainType::Water);
        assert_eq!(map.get_terrain(5, 5), TerrainType::Water);
    }
}
