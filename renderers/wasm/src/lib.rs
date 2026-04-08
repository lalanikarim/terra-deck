//! Terra-Deck WASM Canvas Renderer - HP System

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, MouseEvent, KeyboardEvent, HtmlCanvasElement};

pub mod canvases;
pub mod game_core_bridge;

use canvases::render::{draw_rect, draw_text};
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
        let doc = web_sys::window()
            .ok_or("No global `window` exists - this code should only run in a browser")?
            .document()
            .ok_or("No global `document` exists - this code should only run in a browser")?;

        let canvas = doc
            .get_element_by_id(canvas_id)
            .ok_or("Canvas element not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;

        canvas.set_width(960);
        canvas.set_height(600);

        let ctx = canvas
            .get_context("2d")?
            .ok_or("Could not get 2D context")?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| "Context is not CanvasRenderingContext2d")?;

        Ok(CanvasApplication {
            canvas,
            ctx,
            game_state: GameState::new(),
            width: 960,
            height: 600,
        })
    }

    pub fn run(&mut self) {
        self.render();
    }

    pub fn render(&self) {
        let width = self.width;
        let height = self.height;
        let game_state = &self.game_state;
        
        self.draw_background(&self.ctx);
        self.draw_ui(&self.ctx, width, height, game_state);
        self.draw_opponent_hand(&self.ctx, &game_state.opponent_hand, 50.0, width);
        self.draw_player_hand(&self.ctx, &game_state.player_hand, height as f64 - 190.0, width);
        self.draw_combat_log(&self.ctx);
    }

    fn draw_background(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#1a1a2e"));
        ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    fn draw_ui(&self, ctx: &CanvasRenderingContext2d, width: u32, height: u32, game_state: &GameState) {
        let (turn_str, color) = match game_state.turn {
            game_core_bridge::GameTurn::Player => ("YOUR TURN", "#00ff00"),
            game_core_bridge::GameTurn::Opponent => ("OPPONENT TURN", "#ff0000"),
        };

        let w = width as f64;
        let h = height as f64;

        // Status bar
        let status_w = 200.0;
        let status_h = 30.0;
        draw_rect(ctx, (w - status_w) / 2.0, h - 35.0, status_w, status_h, "#333333", "#555555");
        draw_text(ctx, turn_str, (w - status_w) / 2.0 + 10.0, h - 12.0, "16px bold Arial", color);

        // Player info
        let player_cards = game_state.player_hand.living_count();
        let player_str = format!("Your cards: {}", player_cards);
        draw_text(ctx, &player_str, 10.0, 52.0, "14px Arial", "#ffffff");

        // Opponent info
        let opponent_str = format!("Opponent: {} cards", game_state.opponent_hand.living_count());
        draw_text(ctx, &opponent_str, w - 150.0, 52.0, "14px Arial", "#ffffff");
    }

    fn draw_opponent_hand(&self, ctx: &CanvasRenderingContext2d, hand: &BridgeHand, y_base: f64, width: u32) {
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;
        let count = hand.cards.len();
        let total_w = (count as f64) * (card_w + spacing);
        let start_x = (width as f64 - total_w) / 2.0;

        for (i, card) in hand.cards.iter().enumerate() {
            let x = start_x + (i as f64) * (card_w + spacing);
            let is_selected = self.game_state.target_card_index == Some(i);
            card.render(ctx, x, y_base, card_w, card_h, false, is_selected);
        }
    }

    fn draw_player_hand(&self, ctx: &CanvasRenderingContext2d, hand: &BridgeHand, y_base: f64, width: u32) {
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;
        let count = hand.cards.len();
        let total_w = (count as f64) * (card_w + spacing);
        let start_x = (width as f64 - total_w) / 2.0;

        for (i, card) in hand.cards.iter().enumerate() {
            let x = start_x + (i as f64) * (card_w + spacing);
            let is_selected = self.game_state.selected_card_index == Some(i);
            card.render(ctx, x, y_base, card_w, card_h, true, is_selected);
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
        self.handle_click(mx, my);
        self.render();
    }

    pub fn on_keydown(&self, _e: &KeyboardEvent) {}
    pub fn on_mousemove(&self, _e: &MouseEvent) {}
    pub fn on_mouseup(&mut self, _e: &MouseEvent) {}

    fn handle_click(&mut self, mx: f64, my: f64) {
        let opponent_y = 50.0;
        let player_y = self.height as f64 - 190.0;
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;

        // Check opponent cards first (target selection)
        {
            let hand = &self.game_state.opponent_hand;
            let count = hand.cards.len();
            let total_w = (count as f64) * (card_w + spacing);
            let start_x = (self.width as f64 - total_w) / 2.0;

            for (i, card) in hand.cards.iter().enumerate() {
                if card.is_dead() { continue; }
                let x = start_x + (i as f64) * (card_w + spacing);
                if mx >= x && mx <= x + card_w && my >= opponent_y && my <= opponent_y + card_h {
                    self.game_state.select_target_card(i);
                    return;
                }
            }
        }

        // Check player cards
        {
            let hand = &self.game_state.player_hand;
            let count = hand.cards.len();
            let total_w = (count as f64) * (card_w + spacing);
            let start_x = (self.width as f64 - total_w) / 2.0;

            for (i, card) in hand.cards.iter().enumerate() {
                if card.is_dead() { continue; }
                let x = start_x + (i as f64) * (card_w + spacing);
                if mx >= x && mx <= x + card_w && my >= player_y && my <= player_y + card_h {
                    self.game_state.select_player_card(i);
                    return;
                }
            }
        }
    }
}
