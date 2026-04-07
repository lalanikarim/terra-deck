//! Card system for Terra-Deck - optimized
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum Suit { Hearts, Diamonds, Clubs, Spades }

impl Suit {
    fn color(&self) -> &'static str {
        match self {
            Suit::Hearts | Suit::Diamonds => "#e63946",
            Suit::Clubs | Suit::Spades => "#1d3557",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace,
}

impl Rank {
    fn text(&self) -> &'static str {
        match self {
            Rank::Two => "2", Rank::Three => "3", Rank::Four => "4",
            Rank::Five => "5", Rank::Six => "6", Rank::Seven => "7",
            Rank::Eight => "8", Rank::Nine => "9", Rank::Ten => "10",
            Rank::Jack => "J", Rank::Queen => "Q", Rank::King => "K", Rank::Ace => "A",
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub x: f64, pub y: f64,
    pub width: u32, pub height: u32,
    pub is_face_up: bool,
    pub is_selected: bool,
}

impl Card {
    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        let w = self.width as f64;
        let h = self.height as f64;

        // Card background
        ctx.begin_path();
        let r = 8.0;
        ctx.arc(self.x + r, self.y + r, r, PI, PI + PI / 2.0);
        ctx.arc(self.x + w - r, self.y + r, r, PI * 1.5, 0.0);
        ctx.arc(self.x + w - r, self.y + h - r, r, 0.0, PI / 2.0);
        ctx.arc(self.x + r, self.y + h - r, r, PI / 2.0, PI);
        let _ = ctx.set_fill_style_str("#ffffff");
        ctx.fill();
        let _ = ctx.set_stroke_style_str("#cccccc");
        ctx.stroke();

        if self.is_face_up {
            // Card face - center suit symbol
            let color = self.suit.color();
            ctx.set_fill_style_str(color);
            ctx.set_font("42px Arial");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(self.suit.symbol(), self.x + w / 2.0, self.y + h / 2.0);
            
            // Top-left rank/suit
            ctx.set_font("18px Arial");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(self.rank.text(), self.x + 6.0, self.y + 6.0);
            let _ = ctx.fill_text(self.suit.symbol(), self.x + 6.0, self.y + 24.0);
            
            // Bottom-right (inverted)
            ctx.set_text_align("right");
            ctx.set_text_baseline("bottom");
            let _ = ctx.fill_text(self.rank.text(), self.x + w - 6.0, self.y + h - 6.0);
            let _ = ctx.fill_text(self.suit.symbol(), self.x + w - 6.0, self.y + h - 24.0);
        } else {
            // Card back - patterned purple
            for i in 0..4 {
                for j in 0..5 {
                    let px = self.x + 10.0 + (i as f64) * 18.0;
                    let py = self.y + 10.0 + (j as f64) * 22.0;
                    ctx.set_fill_style_str("#8b008b");
                    ctx.fill_rect(px, py, 10.0, 10.0);
                }
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub card_width: u32,
    pub card_height: u32,
    pub spacing: f64,       // Space between cards
    pub offset_x: f64,      // Starting X position
    pub offset_y: f64,      // Starting Y position
}

impl Deck {
    pub fn new(card_width: u32, card_height: u32) -> Self {
        Deck {
            cards: Vec::new(),
            card_width,
            card_height,
            spacing: 20.0,      // Increased spacing from 0 to 20px
            offset_x: 0.0,      // Will be calculated in center()
            offset_y: 50.0,
        }
    }

    /// Add card and calculate position
    pub fn add_card(&mut self, suit: Suit, rank: Rank) {
        let count = self.cards.len() as f64;
        let card_x = self.offset_x + (count * (self.card_width as f64 + self.spacing));
        
        self.cards.push(Card {
            suit,
            rank,
            x: card_x,
            y: self.offset_y,
            width: self.card_width,
            height: self.card_height,
            is_face_up: true,
            is_selected: false,
        });
    }

    /// Center deck horizontally within given width
    pub fn center_in_width(&mut self, container_width: u32) {
        let card_w = self.card_width as f64;
        let count = self.cards.len() as f64;
        
        if count == 0.0 {
            self.offset_x = 0.0;
            return;
        }
        
        // Total width = cards + spacing between them
        let total_width = (count * card_w) + ((count - 1.0) * self.spacing);
        self.offset_x = (container_width as f64 - total_width) / 2.0;
    }

    /// Recalculate positions after centering
    pub fn recalculate_positions(&mut self) {
        for (i, card) in self.cards.iter_mut().enumerate() {
            let idx = i as f64;
            card.x = self.offset_x + (idx * (self.card_width as f64 + self.spacing));
        }
    }

    /// Add test cards with good suits
    pub fn add_test_cards(&mut self) {
        let cards = [(Suit::Hearts, Rank::Ace),
                     (Suit::Spades, Rank::King),
                     (Suit::Diamonds, Rank::Queen),
                     (Suit::Clubs, Rank::Jack),
                     (Suit::Hearts, Rank::Ten)];
        for (suit, rank) in cards {
            self.add_card(suit, rank);
        }
    }

    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        for card in &self.cards {
            card.render(ctx);
        }
    }

    pub fn len(&self) -> usize { self.cards.len() }
}
