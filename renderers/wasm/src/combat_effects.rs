//! Combat visual effects for Terra-Deck
//!
//! Provides animations and feedback for combat actions:
//! - Damage numbers
//! - Card shake animations
//! - Critical hit effects
//! - Combat log display

use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;

/// Combat damage number floating above a target
#[derive(Debug, Clone)]
pub struct DamageNumber {
    pub x: f64,
    pub y: f64,
    pub damage: u8,
    pub is_critical: bool,
    pub alpha: f64,
    pub lifetime: f64,
    pub velocity_y: f64,
}

impl DamageNumber {
    pub fn new(x: f64, y: f64, damage: u8, is_critical: bool) -> Self {
        Self {
            x,
            y,
            damage,
            is_critical,
            alpha: 1.0,
            lifetime: 1.0, // 1 second lifetime
            velocity_y: -30.0, // Float upward
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Move upward
        self.y += self.velocity_y * delta_time;
        
        // Slow down as we go up
        self.velocity_y *= 0.95;
        
        // Fade out
        self.alpha -= delta_time * 0.3;
        self.lifetime -= delta_time;
        
        // Clamp alpha
        self.alpha = self.alpha.max(0.0);
    }

    pub fn is_alive(&self) -> bool {
        self.alpha > 0.0
    }
}

/// Card shake animation state
#[derive(Debug, Clone, Default)]
pub struct CardShake {
    pub index: usize,
    pub intensity: f64,
    pub duration: f64,
    pub remaining: f64,
}

impl CardShake {
    pub fn new(index: usize, intensity: f64, duration: f64) -> Self {
        Self {
            index,
            intensity,
            duration,
            remaining: duration,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.remaining -= delta_time;
        if self.remaining < 0.0 {
            self.remaining = 0.0;
        }
    }

    pub fn is_active(&self) -> bool {
        self.remaining > 0.0
    }

    pub fn get_offset(&self) -> f64 {
        if !self.is_active() {
            return 0.0;
        }
        
        // Intensity oscillates based on remaining time
        let progress = self.remaining / self.duration;
        (progress * 6.28 * PI).sin() * self.intensity
    }
}

/// Combat effect types
#[derive(Debug, Clone)]
pub enum CombatEffect {
    Damage(DamageNumber),
    Shake(CardShake),
}

/// Combat log entry for display
#[derive(Debug, Clone)]
pub struct CombatLogEntry {
    pub message: String,
    pub color: String,
    pub alpha: f64,
    pub y: f64,
}

impl CombatLogEntry {
    pub fn new(message: String, color: String, y: f64) -> Self {
        Self {
            message,
            color,
            alpha: 1.0,
            y,
        }
    }
}

/// Combat effects manager
#[derive(Debug, Clone, Default)]
pub struct CombatEffects {
    pub damage_numbers: Vec<DamageNumber>,
    pub card_shakes: Vec<CardShake>,
    pub log_entries: Vec<CombatLogEntry>,
    pub log_offset: f64,
}

impl CombatEffects {
    pub fn new() -> Self {
        Self {
            damage_numbers: Vec::new(),
            card_shakes: Vec::new(),
            log_entries: Vec::new(),
            log_offset: 350.0,
        }
    }

    /// Add damage number effect
    pub fn add_damage(&mut self, x: f64, y: f64, damage: u8, is_critical: bool) {
        let effect = DamageNumber::new(x, y, damage, is_critical);
        self.damage_numbers.push(effect);
    }

    /// Add card shake effect
    pub fn add_shake(&mut self, card_index: usize, intensity: f64, duration: f64) {
        let effect = CardShake::new(card_index, intensity, duration);
        self.card_shakes.push(effect);
    }

    /// Add combat log entry
    pub fn add_log_entry(&mut self, message: &str, color: &str) {
        if self.log_entries.len() >= 5 {
            self.log_entries.remove(0);
            self.log_offset -= 20.0;
        }
        
        let entry = CombatLogEntry::new(message.to_string(), color.to_string(), self.log_offset);
        self.log_entries.push(entry);
    }

    /// Clear all effects
    pub fn clear(&mut self) {
        self.damage_numbers.clear();
        self.card_shakes.clear();
        self.log_entries.clear();
        self.log_offset = 350.0;
    }

    /// Update all effects
    pub fn update(&mut self, delta_time: f64) {
        // Update damage numbers
        self.damage_numbers.retain_mut(|effect| {
            effect.update(delta_time);
            effect.is_alive()
        });

        // Update card shakes
        self.card_shakes.retain_mut(|effect| {
            effect.update(delta_time);
            effect.is_active()
        });

        // Fade log entries
        for entry in &mut self.log_entries {
            entry.alpha -= delta_time * 0.02;
        }
        self.log_entries.retain(|e| e.alpha > 0.0);
    }

    /// Render damage number
    pub fn render_damage(&self, damage: &DamageNumber, ctx: &mut CanvasRenderingContext2d) {
        // Text style
        let size = if damage.is_critical { 24 } else { 18 };
        let color = if damage.is_critical { "#ff6600" } else { "#ffffff" };
        
        // Draw damage text
        ctx.set_font(&format!("{}px sans-serif", size));
        ctx.set_fill_style_str(color);
        ctx.set_text_align("center");
        let text = if damage.is_critical {
            format!("CRIT {}!", damage.damage)
        } else {
            format!("{}", damage.damage)
        };
        let _ = ctx.fill_text(&text, damage.x, damage.y);
    }

    /// Render combat log
    pub fn render_log(&self, ctx: &mut CanvasRenderingContext2d) {
        if self.log_entries.is_empty() { return; }
        
        // Log panel on LEFT side
        let panel_x = 10.0;
        let panel_y = 140.0;  // Below the info box
        let panel_width = 240.0;
        let entry_height = 20.0;
        let panel_height = (self.log_entries.len() as f64 * 22.0).min(150.0);  // Max 150px height
        
        // Background
        let _ = ctx.set_fill_style_str("rgba(45, 45, 61, 0.9)");
        ctx.fill_rect(panel_x, panel_y, panel_width, panel_height);
        
        // Title
        ctx.set_font("bold 12px sans-serif");
        ctx.set_fill_style_str("#a0a0ff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("COMBAT LOG", panel_x + 10.0, panel_y + 4.0);

        // Entries - truncated to fit
        for (i, entry) in self.log_entries.iter().enumerate() {
            let y = panel_y + 24.0 + (i as f64 * 20.0);
            if y >= panel_y + panel_height - entry_height { break; }  // Don't overflow
            
            // Truncate long messages
            ctx.set_font("11px monospace");
            ctx.set_fill_style_str(&entry.color);
            let _ = ctx.fill_text(&entry.message, panel_x + 10.0, y);
        }
    }

    /// Get card shake offset
    pub fn get_card_shake(&self, card_index: usize) -> f64 {
        for shake in &self.card_shakes {
            if shake.index == card_index && shake.is_active() {
                return shake.get_offset();
            }
        }
        0.0
    }
}
