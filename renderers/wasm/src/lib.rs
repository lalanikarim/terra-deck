//! Terra-Deck WASM Canvas Renderer with Attack Button

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, MouseEvent};

pub mod canvases;
pub mod game_core_bridge;

use canvases::render::{draw_rect, draw_text};
use game_core_bridge::{BridgeHand, GameState};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct CanvasApplication {
    #[allow(dead_code)]
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    game_state: std::cell::RefCell<GameState>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl CanvasApplication {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<CanvasApplication, JsValue> {
        let doc = web_sys::window()
            .ok_or("No global `window` exists")?
            .document()
            .ok_or("No global `document` exists")?;

        let canvas = doc
            .get_element_by_id(canvas_id)
            .ok_or("Canvas element not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;

        canvas.set_width(1024);
        canvas.set_height(600);

        let ctx = canvas
            .get_context("2d")?
            .ok_or("Could not get 2D context")?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| "Context is not CanvasRenderingContext2d")?;

        Ok(CanvasApplication {
            canvas,
            ctx,
            game_state: std::cell::RefCell::new(GameState::new()),
            width: 1024,
            height: 600,
        })
    }

    pub fn run(&self) {
        self.render();
        self.update_attack_button();
    }

    pub fn render(&self) {
        let width = self.width;
        let height = self.height;
        let game_state = self.game_state.borrow();

        self.draw_background(&self.ctx);
        self.draw_ui(&self.ctx, width, height, &game_state);
        self.draw_attack_button(&self.ctx, width, height);
        self.draw_opponent_hand(&self.ctx, &game_state.opponent_hand, 50.0, width);
        self.draw_player_hand(
            &self.ctx,
            &game_state.player_hand,
            height as f64 - 170.0,
            width,
        );
        self.draw_combat_log(&self.ctx);
    }

    pub fn execute_attack(&self) {
        let state = &mut *self.game_state.borrow_mut();
        state.execute_attack();
        self.render();
        self.update_attack_button();
    }

    pub fn reset_game(&self) {
        *self.game_state.borrow_mut() = GameState::new();
        self.render();
        self.update_attack_button();
    }

    pub fn update_attack_button(&self) {
        let state = self.game_state.borrow();
        let has_both_selected =
            state.selected_card_index.is_some() && state.target_card_index.is_some();
        let is_player_turn = matches!(state.turn, game_core_bridge::GameTurn::Player);

        let btn = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("attack-btn"))
            .and_then(|b| b.dyn_into::<web_sys::HtmlButtonElement>().ok());

        if let Some(ref button) = btn {
            button.set_disabled(!has_both_selected || !is_player_turn);
        }
    }

    fn draw_background(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style_str("#1a1a2e");
        ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    fn draw_ui(
        &self,
        ctx: &CanvasRenderingContext2d,
        width: u32,
        _height: u32,
        _game_state: &GameState,
    ) {
        let player_cards = self.game_state.borrow().player_hand.living_count();
        let opponent_cards = self.game_state.borrow().opponent_hand.living_count();

        let w = width as f64;

        // Instructions
        let instruction = match (
            self.game_state.borrow().selected_card_index,
            self.game_state.borrow().target_card_index,
        ) {
            (None, _) => "1️⃣ Select your card to attack",
            (Some(_), None) => "2️⃣ Select opponent target",
            (Some(_), Some(_)) => "3️⃣ Click ATTACK button!",
        };
        draw_text(ctx, instruction, w - 300.0, 80.0, "16px Arial", "#ffff00");

        // Player info
        let player_str = format!("Your cards: {}", player_cards);
        draw_text(ctx, &player_str, 10.0, 52.0, "14px Arial", "#ffffff");

        // Opponent info
        let opponent_str = format!("Opponent: {} cards", opponent_cards);
        draw_text(ctx, &opponent_str, w - 150.0, 52.0, "14px Arial", "#ffffff");
    }

    fn draw_attack_button(&self, ctx: &CanvasRenderingContext2d, width: u32, height: u32) {
        let w = width as f64;
        let h = height as f64;

        let btn_x = w / 2.0 - 80.0;
        let btn_y = h - 110.0;
        let btn_w = 160.0;
        let btn_h = 50.0;

        // Draw button background
        draw_rect(ctx, btn_x, btn_y, btn_w, btn_h, "#4CAF50", "#2E7D32");

        // Draw button border
        ctx.set_line_width(2.0);
        ctx.set_stroke_style_str("#2E7D32");
        ctx.stroke_rect(btn_x, btn_y, btn_w, btn_h);

        // Draw button text
        draw_text(
            ctx,
            "⚔️ ATTACK",
            w / 2.0 - 50.0,
            btn_y + 18.0,
            "18px bold Arial",
            "#ffffff",
        );
    }

    fn draw_opponent_hand(
        &self,
        ctx: &CanvasRenderingContext2d,
        hand: &BridgeHand,
        y_base: f64,
        width: u32,
    ) {
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;
        let count = hand.cards.len();
        let total_w = (count as f64) * (card_w + spacing);
        let start_x = (width as f64 - total_w) / 2.0;
        let selected_target = self.game_state.borrow().target_card_index;

        for (i, card) in hand.cards.iter().enumerate() {
            let x = start_x + (i as f64) * (card_w + spacing);
            let is_selected = selected_target == Some(i);
            card.render(ctx, x, y_base, card_w, card_h, false, is_selected);
        }
    }

    fn draw_player_hand(
        &self,
        ctx: &CanvasRenderingContext2d,
        hand: &BridgeHand,
        y_base: f64,
        width: u32,
    ) {
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;
        let count = hand.cards.len();
        let total_w = (count as f64) * (card_w + spacing);
        let start_x = (width as f64 - total_w) / 2.0;
        let selected_player = self.game_state.borrow().selected_card_index;

        for (i, card) in hand.cards.iter().enumerate() {
            let x = start_x + (i as f64) * (card_w + spacing);
            let is_selected = selected_player == Some(i);
            card.render(ctx, x, y_base, card_w, card_h, true, is_selected);
        }
    }

    fn draw_combat_log(&self, ctx: &CanvasRenderingContext2d) {
        let x = 10.0;
        let y = 100.0;
        let w = 220.0;
        let h = 180.0;
        draw_rect(ctx, x, y, w, h, "#0f0f23", "#333333");
        draw_text(
            ctx,
            "BATTLE LOG",
            x + 10.0,
            y + 20.0,
            "14px bold Arial",
            "#ffffff",
        );

        let logs = &self.game_state.borrow().combat_log;
        let start = if logs.len() > 8 { logs.len() - 8 } else { 0 };
        for (i, entry) in logs.iter().enumerate() {
            if i >= start {
                let line_y = y + 35.0 + ((i - start) as f64) * 18.0;
                draw_text(ctx, entry, x + 10.0, line_y, "12px Arial", "#cccccc");
            }
        }
    }

    pub fn on_mousedown(&self, e: &MouseEvent) {
        let mx = e.offset_x() as f64;
        let my = e.offset_y() as f64;
        log(&format!("Mouse click: x={:.0}, y={:.0}", mx, my));
        self.handle_click(mx, my);
    }

    pub fn on_keydown(&self, _e: &KeyboardEvent) {}
    pub fn on_mousemove(&self, _e: &MouseEvent) {}
    pub fn on_mouseup(&mut self, _e: &MouseEvent) {}

    fn handle_click(&self, mx: f64, my: f64) {
        let opponent_y = 50.0;
        let player_y = self.height as f64 - 170.0;
        let card_w = 80.0;
        let card_h = 120.0;
        let spacing = 10.0;

        // Check opponent cards (target selection)
        {
            let hand = &self.game_state.borrow().opponent_hand;
            let count = hand.cards.len();
            let total_w = (count as f64) * (card_w + spacing);
            let start_x = (self.width as f64 - total_w) / 2.0;

            for (i, card) in hand.cards.iter().enumerate() {
                if card.is_dead() {
                    continue;
                }
                let x = start_x + (i as f64) * (card_w + spacing);
                if mx >= x && mx <= x + card_w && my >= opponent_y && my <= opponent_y + card_h {
                    let state = &mut *self.game_state.borrow_mut();
                    state.select_target_card(i);
                    self.render();
                    self.update_attack_button();
                    return;
                }
            }
        }

        // Check player cards (source selection)
        {
            let hand = &self.game_state.borrow().player_hand;
            let count = hand.cards.len();
            let total_w = (count as f64) * (card_w + spacing);
            let start_x = (self.width as f64 - total_w) / 2.0;

            for (i, card) in hand.cards.iter().enumerate() {
                if card.is_dead() {
                    continue;
                }
                let x = start_x + (i as f64) * (card_w + spacing);
                if mx >= x && mx <= x + card_w && my >= player_y && my <= player_y + card_h {
                    let state = &mut *self.game_state.borrow_mut();
                    state.select_player_card(i);
                    self.render();
                    self.update_attack_button();
                    return;
                }
            }
        }
    }
}
