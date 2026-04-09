//! Bridge layer between game_core types and WASM rendering types

use super::canvases::render::{draw_rect, draw_text};
use game_core::types::{Archetype, Rank};
use web_sys::CanvasRenderingContext2d;

#[derive(Debug, Clone)]
pub struct BridgeCard {
    pub suit: game_core::types::Suit,
    pub rank: game_core::types::Rank,
    pub hp: u8,
    pub max_hp: u8,
}

impl BridgeCard {
    pub fn from_game_core(card: &game_core::Card) -> Self {
        BridgeCard {
            suit: card.suit,
            rank: card.rank,
            hp: card.hp,
            max_hp: card.max_hp,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }

    pub fn take_damage(&mut self, damage: u8) -> bool {
        let old_hp = self.hp;
        self.hp = self.hp.saturating_sub(damage);
        self.hp == 0 && old_hp > 0
    }

    fn get_suit_color(&self) -> &'static str {
        match self.suit {
            game_core::types::Suit::Hearts => "crimson",
            game_core::types::Suit::Diamonds => "crimson",
            game_core::types::Suit::Clubs => "black",
            game_core::types::Suit::Spades => "black",
        }
    }

    fn get_suit_symbol(&self) -> &'static str {
        match self.suit {
            game_core::types::Suit::Hearts => "♥",
            game_core::types::Suit::Diamonds => "♦",
            game_core::types::Suit::Clubs => "♣",
            game_core::types::Suit::Spades => "♠",
        }
    }

    fn get_rank_str(&self) -> &'static str {
        match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }

    fn get_rank_value(&self) -> u8 {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    pub fn render(
        &self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        card_width: f64,
        card_height: f64,
        face_up: bool,
        is_selected: bool,
    ) {
        if self.is_dead() {
            return;
        }

        // Draw selection highlight if selected
        if is_selected {
            draw_rect(
                ctx,
                x - 5.0,
                y - 28.0,
                card_width + 10.0,
                card_height + 28.0,
                "#ffd700",
                "none",
            );
        }

        // Draw HP bar above card
        self.render_hp_bar(ctx, x, y - 25.0, card_width, 8.0);

        // Draw HP text above HP bar
        let hp_str = self.get_hp_text();
        let text_center = x + card_width / 2.0;
        draw_text(
            ctx,
            &hp_str,
            text_center - hp_str.len() as f64 * 4.0,
            y - 33.0,
            "12px Arial",
            "#ffffff",
        );

        // Draw the card
        draw_rect(ctx, x, y, card_width, card_height, "#ffffff", "#000000");

        if face_up {
            let rank_str = self.get_rank_str();
            let color = self.get_suit_color();

            draw_text(ctx, rank_str, x + 8.0, y + 25.0, "16px bold Arial", color);
            draw_text(
                ctx,
                rank_str,
                x + card_width - 22.0,
                y + card_height - 5.0,
                "16px bold Arial",
                color,
            );
            draw_text(
                ctx,
                self.get_suit_symbol(),
                x + card_width / 2.0 - 8.0,
                y + card_height / 2.0,
                "48px Arial",
                color,
            );
        } else {
            draw_rect(
                ctx,
                x + 5.0,
                y + 5.0,
                card_width - 10.0,
                card_height - 10.0,
                "#222222",
                "#444444",
            );
        }
    }

    fn render_hp_bar(
        &self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        draw_rect(ctx, x, y, width, height, "#333333", "#555555");

        let fill_ratio = if self.max_hp > 0 {
            self.hp as f64 / self.max_hp as f64
        } else {
            0.0
        };
        let fill_width = width * fill_ratio;
        let hp_color = if fill_ratio > 0.66 {
            "#00ff00"
        } else if fill_ratio > 0.33 {
            "#ffff00"
        } else {
            "#ff0000"
        };
        draw_rect(ctx, x, y, fill_width, height, hp_color, "none");
    }

    fn get_hp_text(&self) -> String {
        format!("{} / {}", self.hp, self.max_hp)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BridgeHand {
    pub cards: Vec<BridgeCard>,
}

impl BridgeHand {
    pub fn new() -> Self {
        BridgeHand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: BridgeCard) {
        self.cards.push(card);
    }

    pub fn living_count(&self) -> usize {
        self.cards.iter().filter(|c| !c.is_dead()).count()
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub player_hand: BridgeHand,
    pub opponent_hand: BridgeHand,
    pub selected_card_index: Option<usize>,
    pub target_card_index: Option<usize>,
    pub combat_log: Vec<String>,
    pub turn: GameTurn,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum GameTurn {
    #[default]
    Player,
    Opponent,
}

impl GameState {
    pub fn new() -> Self {
        const MAX_HP: u8 = 10;
        let mut state = GameState {
            player_hand: BridgeHand::new(),
            opponent_hand: BridgeHand::new(),
            ..Default::default()
        };

        // Player cards (Hearts - Rock archetype)
        let player_ranks = [Rank::Two, Rank::Five, Rank::Eight, Rank::Jack, Rank::Ace];
        for rank in player_ranks.iter() {
            state.player_hand.add_card(BridgeCard {
                suit: game_core::types::Suit::Hearts,
                rank: *rank,
                hp: MAX_HP,
                max_hp: MAX_HP,
            });
        }

        // Opponent cards (various archetypes)
        let suit_arr = [
            game_core::types::Suit::Spades,   // Infantry
            game_core::types::Suit::Clubs,    // Scissors
            game_core::types::Suit::Diamonds, // Paper
            game_core::types::Suit::Hearts,   // Rock
            game_core::types::Suit::Spades,   // Infantry
        ];
        let rank_arr = [Rank::Jack, Rank::Queen, Rank::King, Rank::Ace, Rank::Ten];
        for i in 0..5 {
            state.opponent_hand.add_card(BridgeCard {
                suit: suit_arr[i],
                rank: rank_arr[i],
                hp: MAX_HP,
                max_hp: MAX_HP,
            });
        }

        state
    }

    pub fn select_player_card(&mut self, index: usize) {
        if index < self.player_hand.cards.len() && !self.player_hand.cards[index].is_dead() {
            self.selected_card_index = Some(index);
            let rank_str = self.player_hand.cards[index].get_rank_str();
            self.add_log(format!("Selected player card: {}♥", rank_str));
        }
    }

    pub fn select_target_card(&mut self, index: usize) {
        if index < self.opponent_hand.cards.len() && !self.opponent_hand.cards[index].is_dead() {
            self.target_card_index = Some(index);
            let rank_str = self.opponent_hand.cards[index].get_rank_str();
            let symbol = self.opponent_hand.cards[index].get_suit_symbol();
            self.add_log(format!("Target selected: {}{}", rank_str, symbol));
        }
    }

    pub fn execute_attack(&mut self) {
        // Clone indices before borrowing
        let src_idx = self.selected_card_index;
        let tgt_idx = self.target_card_index;

        if let Some(src_i) = src_idx {
            if let Some(tgt_i) = tgt_idx {
                // Clone card data before any borrows to avoid borrow issues
                let src = self.player_hand.cards[src_i].clone();
                let tgt = self.opponent_hand.cards[tgt_i].clone();
                let src_rank = src.get_rank_str();
                let tgt_rank = tgt.get_rank_str();
                let tgt_symbol = tgt.get_suit_symbol();

                let dmg = calculate_damage(&src, &tgt);
                let was_dead = self.opponent_hand.cards[tgt_i].take_damage(dmg);

                self.add_log(format!("{}♥ attacks {}{}", src_rank, tgt_rank, tgt_symbol));
                self.add_log(format!("Dealt {} damage", dmg));

                if was_dead {
                    self.add_log(format!("{}{} DESTROYED!", tgt_rank, tgt_symbol));
                }

                // Clear selections
                self.selected_card_index = None;
                self.target_card_index = None;

                // Switch turn
                self.turn = GameTurn::Opponent;
            }
        }
    }

    fn add_log(&mut self, msg: String) {
        if self.combat_log.len() > 50 {
            self.combat_log.remove(0);
        }
        self.combat_log.push(msg);
    }
}

// External damage calculation function
fn calculate_damage(attacker: &BridgeCard, defender: &BridgeCard) -> u8 {
    let base_damage = attacker.get_rank_value();
    let attacker_arch = attacker.suit.archetype();
    let defender_arch = defender.suit.archetype();

    if attacker_arch == defender_arch {
        return base_damage;
    }

    // Check if attacker has advantage
    let attacker_wins = match (attacker_arch, defender_arch) {
        (Archetype::Rock, Archetype::Scissors) => true,
        (Archetype::Scissors, Archetype::Paper) => true,
        (Archetype::Paper, Archetype::Rock) => true,
        _ => false,
    };

    if attacker_wins {
        (base_damage * 2).min(100)
    } else {
        base_damage / 2
    }
}
