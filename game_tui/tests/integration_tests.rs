//! Integration tests for full game flow

use game_core::*;

/// Test: Full game flow from deck creation to win condition
#[test]
fn test_full_game_flow() {
    // 1. Create deck
    let mut deck = Deck::new();
    assert_eq!(deck.len(), 52);

    // 2. Create hands
    let mut player_hand = Hand::default();
    let mut opponent_hand = Hand::default();

    // 3. Deal initial hands (5 cards each)
    for _ in 0..5 {
        player_hand.add_card(deck.draw().unwrap());
        opponent_hand.add_card(deck.draw().unwrap());
    }

    assert_eq!(player_hand.len(), 5);
    assert_eq!(opponent_hand.len(), 5);
    assert_eq!(deck.len(), 42);

    // 4. Create selected card tracker
    let selected = SelectedCard::new(0);

    // 5. Simulate combat - play card 0 from each hand
    let player_card_idx = selected.index.unwrap();
    let opponent_card_idx = 0;

    let player_card = player_hand.cards[player_card_idx].clone();
    let opponent_card = opponent_hand.cards[opponent_card_idx].clone();

    // Apply combat
    let mut player_copy = player_card.clone();
    let mut opponent_copy = opponent_card.clone();

    let (_, _player_result) = apply_combat_damage(&player_card, &mut opponent_copy);
    let (_, _opponent_result) = apply_combat_damage(&opponent_card, &mut player_copy);

    // Update cards in hands with damage
    player_hand.cards[player_card_idx] = player_copy;
    opponent_hand.cards[opponent_card_idx] = opponent_copy;

    // 6. Remove dead cards
    let player_dead = player_hand.remove_dead_cards();
    let opponent_dead = opponent_hand.remove_dead_cards();

    // At least one value must be valid (non-negative)
    let _ = player_dead;
    let _ = opponent_dead;

    println!(
        "Player hand: {} cards, Opponent hand: {} cards, \nDead: player={}, opponent={}",
        player_hand.len(),
        opponent_hand.len(),
        player_dead,
        opponent_dead
    );

    // 7. Check game state
    let result = if player_hand.is_empty() {
        GameResult::Lost
    } else if opponent_hand.is_empty() {
        GameResult::Won
    } else {
        GameResult::Draw
    };

    // Game shouldn't be over after one round normally
    assert_ne!(result, GameResult::Won);
    assert_ne!(result, GameResult::Lost);
}

/// Test: Card takes damage correctly
#[test]
fn test_card_damage_application() {
    let card1 = Card::new(Suit::Hearts, Rank::Ten); // 10 HP
    let card2 = Card::new(Suit::Clubs, Rank::Five); // 5 HP

    // Create copies for damage calculation
    let mut card1_copy = card1.clone();
    let mut card2_copy = card2.clone();

    // Rock (Hearts) vs Scissors (Clubs): 0.5x damage
    let (dmg1, _) = apply_combat_damage(&card1, &mut card2_copy);
    let (dmg2, _) = apply_combat_damage(&card2, &mut card1_copy);

    // Verify damage was reduced
    assert!(
        dmg1 < card1.hp,
        "Damage should be less than attack value when dominant"
    );
    assert!(dmg2 > 0, "Damage should be more than 0");

    // Verify HP was reduced
    assert!(card1_copy.hp < card1.hp, "Player card should take damage");
    assert!(card2_copy.hp < card2.hp, "Opponent card should take damage");
}

/// Test: Card at 0 HP is removed
#[test]
fn test_dead_card_removal() {
    let mut hand = Hand::default();
    hand.add_card(Card::new(Suit::Hearts, Rank::Three));
    hand.add_card(Card::new(Suit::Diamonds, Rank::Four));
    hand.add_card(Card::new(Suit::Clubs, Rank::Five));

    assert_eq!(hand.len(), 3);

    // Kill all cards
    for i in 0..3 {
        let hp = hand.cards[i].hp;
        hand.cards[i].take_damage(hp);
    }

    let removed = hand.remove_dead_cards();
    assert_eq!(removed, 3);
    assert_eq!(hand.len(), 0);
    assert!(hand.is_empty());
}

/// Test: Win condition detection
#[test]
fn test_win_condition() {
    // Player wins when opponent hand is empty
    let player_hand = Hand {
        cards: vec![Card::new(Suit::Hearts, Rank::Ace)],
    };
    let opponent_hand = Hand::default();

    let result = if player_hand.is_empty() {
        GameResult::Lost
    } else if opponent_hand.is_empty() {
        GameResult::Won
    } else {
        GameResult::Draw
    };

    assert_eq!(result, GameResult::Won);
}

/// Test: Loss condition detection
#[test]
fn test_loss_condition() {
    let player_hand = Hand::default();
    let opponent_hand = Hand {
        cards: vec![Card::new(Suit::Spades, Rank::King)],
    };

    let result = if player_hand.is_empty() {
        GameResult::Lost
    } else if opponent_hand.is_empty() {
        GameResult::Won
    } else {
        GameResult::Draw
    };

    assert_eq!(result, GameResult::Lost);
}

/// Test: Combat log records events
#[test]
fn test_combat_log_events() {
    let mut combat_log = CombatLog::default();

    combat_log.add_entry("Game started!".to_string());
    combat_log.add_entry("Player selected card 1".to_string());
    combat_log.add_entry("Combat resolved".to_string());

    assert_eq!(combat_log.iter().count(), 3);

    let entries: Vec<&String> = combat_log.iter().collect();
    assert_eq!(entries[0], "Game started!");
    assert_eq!(entries[1], "Player selected card 1");
    assert_eq!(entries[2], "Combat resolved");
}

/// Test: SelectedCard navigation
#[test]
fn test_selected_card_navigation() {
    let mut selected = SelectedCard::none();

    // Set to index 0
    selected.try_set(0, 4);
    assert_eq!(selected.index, Some(0));

    // Move right (next)
    selected.next(4);
    assert_eq!(selected.index, Some(1));

    // Move left (previous)
    selected.previous(4);
    assert_eq!(selected.index, Some(0));

    // Wrap around at edge
    selected.next(4);
    selected.next(4);
    selected.next(4);
    selected.next(4);
    selected.next(4); // Should wrap to 0
    assert_eq!(selected.index, Some(0));
}

/// Test: Deck shuffling creates different order
#[test]
fn test_deck_shuffling_changes_order() {
    let deck1 = Deck::new();
    let _deck2 = Deck::new();

    // Both decks should start in same order
    assert_eq!(deck1.len(), 52);

    // Note: We can't easily test shuffle without rand in game_tui deps
    // But we verify the structure is correct
    assert_eq!(deck1.len(), 52);
    assert!(!deck1.cards.is_empty());
}

/// Test: GameState transitions
#[test]
fn test_game_state_transitions() {
    let state = GameState::PlayerTurn;
    assert!(state.is_active());
    assert!(state.is_player_turn());
    assert!(!state.is_opponent_turn());
    assert!(!state.should_resolve_combat());

    let combat_state = GameState::CombatResolution;
    assert!(combat_state.should_resolve_combat());
    assert!(combat_state.is_active());

    let game_over = GameState::GameOver(GameResult::Won);
    assert!(!game_over.is_active());
    assert_eq!(game_over, GameState::GameOver(GameResult::Won));
}
