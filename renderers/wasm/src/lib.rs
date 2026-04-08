//! Terra-Deck WASM Canvas Renderer - HP System

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, MouseEvent, KeyboardEvent, HtmlCanvasElement};

pub mod canvases;
pub mod game_core_bridge;

use canvases::render::{draw_rect, draw_text, draw_text_centered};
use game_core_bridge::{GameState, BridgeHand};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct CanvasApplication {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    game_state: GameState,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl CanvasApplication {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<CanvasApplication, JsValue> {
        let document = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return Err("no document".into()),
        };

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

        let game_state = GameState::new_test_game();
        
        log("Terra-Deck: HP System Ready!");
        log(&format!("Player: {} cards | Opponent: {} cards",
                     game_state.player_hand.living_count(),
                     game_state.opponent_hand.living_count()));

        Ok(CanvasApplication {
            canvas, ctx, game_state, width, height,
        })
    }

    pub fn run(&self) {
        self.render();
        log("Click your cards to attack!");
    }

    pub fn render(&self) {
        let w = self.width as f64;
        let h = self.height as f64;

        let ctx = &self.ctx;
        ctx.clear_rect(0.0, 0.0, w, h);
        draw_rect(ctx, 0.0, 0.0, w, h, "#1a1a2e", "none");

        draw_text_centered(ctx, "Terra-Deck", w / 2.0, 20.0, "20px bold Arial", "#ffffff");
        
        self.draw_status_bar(ctx);
        self.draw_hand(ctx, &self.game_state.opponent_hand, 80.0, false);
        self.draw_hand(ctx, &self.game_state.player_hand, h - 200.0, true);
        self.draw_combat_log(ctx);
    }

    fn draw_status_bar(&self, ctx: &CanvasRenderingContext2d) {
        let w = self.width as f64;
        draw_rect(ctx, 0.0, 35.0, w, 25.0, "#0f0f23", "none");
        
        let player_alive = self.game_state.player_hand.living_count();
        let turn_str = if self.game_state.player_turn { "YOUR TURN" } else { "OPPonent TURN" };
        let player_str = format!("Player: {} cards | {}", player_alive, turn_str);
        let color = if self.game_state.player_turn { "#00ff00" } else { "#ff6600" };
        draw_text(ctx, &player_str, 10.0, 52.0, "14px Arial", color);
        
        let opponent_str = format!("Opponent: {} cards", self.game_state.opponent_hand.living_count());
        draw_text(ctx, &opponent_str, w - 150.0, 52.0, "14px Arial", "#ffffff");
    }

    fn draw_hand(&self, ctx: &CanvasRenderingContext2d, hand: &BridgeHand, y_base: f64, face_up: bool) {
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;
        let count = hand.cards.len();
        let total_w = (count as f64) * (card_w + spacing);
        let start_x = (self.width as f64 - total_w) / 2.0;

        for (i, card) in hand.cards.iter().enumerate() {
            let x = start_x + (i as f64) * (card_w + spacing);
            let is_selected = self.game_state.selected_card_index == Some(i);
            
            if is_selected && face_up {
                draw_rect(ctx, x - 5.0, y_base - 5.0, card_w + 10.0, card_h + 10.0, "#ffff00", "none");
            }
            card.render(ctx, x, y_base, card_w, card_h, face_up);
        }
    }

    fn draw_combat_log(&self, ctx: &CanvasRenderingContext2d) {
        let x = 10.0;
        let y = 100.0;
        let w = 220.0;
        let h = 180.0;
        draw_rect(ctx, x, y, w, h, "#0f0f23", "#333333");
        draw_text(ctx, "BATTLE LOG", x + 10.0, y + 20.0, "14px bold Arial", "#ffffff");
        
        let logs = &self.game_state.combat_log;
        let start = if logs.len() > 8 { logs.len() - 8 } else { 0 };
        for (i, entry) in logs.iter().enumerate() {
            if i >= start {
                let line_y = y + 35.0 + ((i - start) as f64) * 18.0;
                draw_text(ctx, entry, x + 10.0, line_y, "12px Arial", "#cccccc");
            }
        }
    }

    pub fn on_mousedown(&mut self, e: &MouseEvent) {
        let mx = e.offset_x() as f64;
        let my = e.offset_y() as f64;
        if self.handle_click(mx, my) {
            self.render();
        }
    }

    pub fn on_keydown(&self, _e: &KeyboardEvent) {}
    pub fn on_mousemove(&self, _e: &MouseEvent) {}
    pub fn on_mouseup(&mut self, _e: &MouseEvent) {}
}

impl CanvasApplication {
    fn handle_click(&mut self, mx: f64, my: f64) -> bool {
        let y_base = self.height as f64 - 200.0;
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;
        let count = self.game_state.player_hand.cards.len();
        let total_w = (count as f64) * (card_w + spacing);
        let start_x = (self.width as f64 - total_w) / 2.0;

        let rel_y = my - y_base;
        if rel_y < 0.0 || rel_y > card_h {
            return false;
        }

        for (i, card) in self.game_state.player_hand.cards.iter().enumerate() {
            let card_x = start_x + (i as f64) * (card_w + spacing);
            if mx >= card_x && mx <= card_x + card_w && !card.is_dead() {
                if self.game_state.selected_card_index == Some(i) {
                    self.game_state.selected_card_index = None;
                    log("Card deselected");
                    return true;
                } else if self.game_state.selected_card_index.is_some() {
                    self.attack();
                    return true;
                } else {
                    self.game_state.selected_card_index = Some(i);
                    log(&format!("Card {} selected - click another to attack!", i + 1));
                    return true;
                }
            }
        }
        false
    }

    fn attack(&mut self) {
        if let Some(attacker_idx) = self.game_state.selected_card_index {
            let _ = attacker_idx;
            
            if let Some(victory) = self.game_state.player_attack() {
                if victory {
                    log("VICTORY! All enemies destroyed!");
                }
                return;
            }

            if let Some(defeat) = self.game_state.opponent_attack() {
                if defeat {
                    log("DEFEAT! All your cards destroyed!");
                }
            }
        }
    }
}
