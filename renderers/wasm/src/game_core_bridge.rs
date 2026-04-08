//! Bridge layer between game_core types and WASM rendering types

use web_sys::CanvasRenderingContext2d;
use super::canvases::render::{draw_rect, draw_text};

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
            game_core::types::Rank::Two => "2",
            game_core::types::Rank::Three => "3",
            game_core::types::Rank::Four => "4",
            game_core::types::Rank::Five => "5",
            game_core::types::Rank::Six => "6",
            game_core::types::Rank::Seven => "7",
            game_core::types::Rank::Eight => "8",
            game_core::types::Rank::Nine => "9",
            game_core::types::Rank::Ten => "10",
            game_core::types::Rank::Jack => "J",
            game_core::types::Rank::Queen => "Q",
            game_core::types::Rank::King => "K",
            game_core::types::Rank::Ace => "A",
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
    ) {
        if self.is_dead() {
            return;
        }

        draw_rect(ctx, x, y, card_width, card_height, "#ffffff", "#000000");
        
        if face_up {
            let rank_str = self.get_rank_str();
            let color = self.get_suit_color();
            
            draw_text(ctx, rank_str, x + 5.0, y - 5.0, "16px bold Arial", color);
            draw_text(ctx, rank_str, x + card_width - 20.0, y + card_height - 5.0, "16px bold Arial", color);
            draw_text(ctx, self.get_suit_symbol(), x + card_width / 2.0 - 8.0, y + card_height / 2.0, "48px Arial", color);
            
            self.render_hp_bar(ctx, x, y - 18.0, card_width, 8.0);
            draw_text(ctx, &self.get_hp_text(), x + card_width / 2.0 - 12.0, y - 11.0, "12px Arial", "#ffffff");
        } else {
            draw_rect(ctx, x + 5.0, y + 5.0, card_width - 10.0, card_height - 10.0, "#222222", "#444444");
        }
    }
    
    fn render_hp_bar(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64) {
        draw_rect(ctx, x, y, width, height, "#333333", "#555555");
        
        let fill_ratio = self.hp as f64 / self.max_hp as f64;
        let fill_width = width * fill_ratio;
        let hp_color = if fill_ratio > 0.66 { "#00ff00" } else if fill_ratio > 0.33 { "#ffff00" } else { "#ff0000" };
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
    
    pub fn get_living_indices(&self) -> Vec<usize> {
        self.cards.iter().enumerate().filter(|(_, c)| !c.is_dead()).map(|(i, _)| i).collect()
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub player_hand: BridgeHand,
    pub opponent_hand: BridgeHand,
    pub selected_card_index: Option<usize>,
    pub player_turn: bool,
    pub combat_log: Vec<String>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_hand: BridgeHand::new(),
            opponent_hand: BridgeHand::new(),
            selected_card_index: None,
            player_turn: true,
            combat_log: Vec::new(),
        }
    }
    
    pub fn log(&mut self, message: String) {
        while self.combat_log.len() >= 10 {
            self.combat_log.remove(0);
        }
        self.combat_log.push(message);
    }
    
    pub fn player_attack(&mut self) -> Option<bool> {
        let attacker_idx = self.selected_card_index?;
        let target_idx = self.opponent_hand.cards.iter().enumerate().find(|(_, c)| !c.is_dead()).map(|(i, _)| i)?;
        
        let attacker = &self.player_hand.cards[attacker_idx];
        let damage = attacker.rank as u8;
        let died = self.opponent_hand.cards[target_idx].take_damage(damage);
        
        self.selected_card_index = None;
        
        let suit = match attacker.suit {
            game_core::types::Suit::Hearts => "Hearts",
            game_core::types::Suit::Diamonds => "Diamonds",
            game_core::types::Suit::Clubs => "Clubs",
            game_core::types::Suit::Spades => "Spades",
        };
        let rank = attacker.get_rank_str();
        let result = if died { "DESTROYED" } else { "damaged" };
        self.log(format!("Your {} {} {} opponent for {} damage", suit, rank, result, damage));
        
        if self.opponent_hand.living_count() == 0 {
            return Some(true);
        }
        
        self.player_turn = false;
        None
    }
    
    pub fn opponent_attack(&mut self) -> Option<bool> {
        let attacker_idx = self.opponent_hand.cards.iter().enumerate().find(|(_, c)| !c.is_dead()).map(|(i, _)| i)?;
        let target_idx = self.player_hand.cards.iter().enumerate().find(|(_, c)| !c.is_dead()).map(|(i, _)| i)?;
        
        let attacker = &self.opponent_hand.cards[attacker_idx];
        let damage = attacker.rank as u8;
        self.player_hand.cards[target_idx].take_damage(damage);
        
        self.log(format!("Opponent attacked for {} damage", damage));
        
        if self.player_hand.living_count() == 0 {
            return Some(true);
        }
        
        self.player_turn = true;
        None
    }
    
    pub fn new_test_game() -> Self {
        let mut player_hand = BridgeHand::new();
        let mut opponent_hand = BridgeHand::new();

        let player_cards = [
            (game_core::types::Rank::Ten, game_core::types::Suit::Hearts),
            (game_core::types::Rank::Nine, game_core::types::Suit::Diamonds),
            (game_core::types::Rank::Eight, game_core::types::Suit::Clubs),
            (game_core::types::Rank::Seven, game_core::types::Suit::Spades),
            (game_core::types::Rank::Six, game_core::types::Suit::Hearts),
        ];
        
        for (rank, suit) in player_cards {
            let core_card = game_core::Card::new(suit, rank);
            player_hand.add_card(BridgeCard::from_game_core(&core_card));
        }

        let opponent_cards = [
            (game_core::types::Rank::King, game_core::types::Suit::Hearts),
            (game_core::types::Rank::Queen, game_core::types::Suit::Diamonds),
            (game_core::types::Rank::Jack, game_core::types::Suit::Clubs),
            (game_core::types::Rank::Ten, game_core::types::Suit::Spades),
            (game_core::types::Rank::Nine, game_core::types::Suit::Hearts),
        ];
        
        for (rank, suit) in opponent_cards {
            let core_card = game_core::Card::new(suit, rank);
            opponent_hand.add_card(BridgeCard::from_game_core(&core_card));
        }

        GameState {
            player_hand,
            opponent_hand,
            selected_card_index: None,
            player_turn: true,
            combat_log: Vec::new(),
        }
    }
}
