//! Terra-Deck WASM Canvas Renderer - Optimized
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent};

pub mod cards;
pub mod combat_effects;

use cards::Deck;
use combat_effects::CombatEffects;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct GameState {
    pub player_hand: Deck,
    pub opponent_hand: Deck,
    pub combat_effects: CombatEffects,
    pub width: u32,
    pub height: u32,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        // Player cards at bottom
        let mut player_hand = Deck::new(100, 150);
        player_hand.offset_y = height as f64 - 180.0;
        player_hand.add_test_cards();
        player_hand.center_in_width(width);
        player_hand.recalculate_positions();

        // Opponent cards at top
        let mut opponent_hand = Deck::new(100, 150);
        opponent_hand.offset_y = 50.0;
        opponent_hand.add_test_cards();
        for card in &mut opponent_hand.cards {
            card.is_face_up = false;  // Face down
        }
        opponent_hand.center_in_width(width);
        opponent_hand.recalculate_positions();

        GameState {
            player_hand,
            opponent_hand,
            combat_effects: CombatEffects::new(),
            width,
            height,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.combat_effects.update(delta_time);
    }
}

fn draw_rect(ctx: &mut CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64, color: &str) {
    let _ = ctx.set_fill_style_str(color);
    ctx.fill_rect(x, y, w, h);
}

#[wasm_bindgen]
pub struct CanvasApplication {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    game_state: GameState,
}

#[wasm_bindgen]
impl CanvasApplication {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<CanvasApplication, JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        let document = web_sys::window()
            .ok_or("no window")?
            .document()
            .ok_or("no document")?;

        let canvas: HtmlCanvasElement = document
            .get_element_by_id(canvas_id)
            .ok_or("canvas not found")?
            .unchecked_into();

        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;

        canvas.set_width(width);
        canvas.set_height(height);

        let ctx = canvas
            .get_context("2d")?
            .ok_or("no 2d context")?
            .unchecked_into::<CanvasRenderingContext2d>();

        let game_state = GameState::new(width, height);
        
        log("Terra-Deck initialized - Cards centered!");

        Ok(CanvasApplication { canvas, ctx, game_state })
    }

    pub fn run(&mut self) {
        self.render();
        log("Ready!");
    }

    pub fn render(&mut self) {
        let w = self.game_state.width as f64;
        let h = self.game_state.height as f64;

        // Clear
        self.ctx.clear_rect(0.0, 0.0, w, h);
        draw_rect(&mut self.ctx, 0.0, 0.0, w, h, "#1a1a2e");

        // Opponent hand (centered top)
        self.game_state.opponent_hand.render(&mut self.ctx);

        // Player hand (centered bottom) with shake
        for (i, card) in self.game_state.player_hand.cards.iter().enumerate() {
            let shake = self.game_state.combat_effects.get_card_shake(i);
            let draw_card = cards::Card {
                suit: card.suit,
                rank: card.rank,
                x: card.x + shake,
                y: card.y,
                width: card.width,
                height: card.height,
                is_face_up: card.is_face_up,
                is_selected: card.is_selected,
            };
            draw_card.render(&mut self.ctx);
        }

        // Damage numbers
        for damage in &self.game_state.combat_effects.damage_numbers {
            self.game_state.combat_effects.render_damage(damage, &mut self.ctx);
        }

        // Combat log
        self.game_state.combat_effects.render_log(&mut self.ctx);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.game_state.update(delta_time);
    }

    pub fn on_keydown(&self, e: &web_sys::KeyboardEvent) {
        log(&format!("Key: {}", e.key()));
    }

    pub fn on_mousemove(&self, _e: &MouseEvent) {}

    pub fn on_mousedown(&mut self, e: &MouseEvent) {
        let mx = e.offset_x() as f64;
        let my = e.offset_y() as f64;
        self.handle_attack(mx, my);
    }

    pub fn on_mouseup(&mut self, _e: &MouseEvent) {}

    pub fn get_particle_count(&self) -> usize { 0 }
    pub fn get_canvas_width(&self) -> u32 { self.canvas.width() }
    pub fn get_canvas_height(&self) -> u32 { self.canvas.height() }
}

impl CanvasApplication {
    fn handle_attack(&mut self, mx: f64, my: f64) {
        let threshold = self.game_state.height as f64 * 0.6;
        
        if my > threshold {
            for (i, card) in self.game_state.player_hand.cards.iter().enumerate() {
                if mx >= card.x && mx <= card.x + card.width as f64 &&
                   my >= card.y && my <= card.y + card.height as f64 {
                    
                    let dmg = ((i % 8) + 2) as u8;
                    let crit = i % 3 == 0;
                    
                    self.game_state.combat_effects.add_damage(
                        card.x + card.width as f64 / 2.0,
                        card.y - 30.0,
                        dmg, crit,
                    );
                    self.game_state.combat_effects.add_shake(i, 5.0, 0.25);
                    
                    let msg = if crit {
                        format!("CRITICAL! {} dmg", dmg)
                    } else {
                        format!("Card #{}: {} dmg", i + 1, dmg)
                    };
                    
                    self.game_state.combat_effects.add_log_entry(
                        &msg,
                        if crit { "#ff6600" } else { "#ffffff" },
                    );
                    
                    log(&msg);
                    break;
                }
            }
        }
    }
}
