//! Simple AI opponent logic

use rand::prelude::*;

/// Selects a card index for the opponent to play
/// Currently uses simple random selection
pub fn select_opponent_card(hand_size: usize) -> Option<usize> {
    if hand_size == 0 {
        return None;
    }
    
    Some(rand::rng().random_range(0..hand_size))
}

/// Updates the selected card state for opponent turn
pub fn opponent_select_card(opponent_hand_size: usize, mut selected: crate::turn_state::SelectedCard) -> crate::turn_state::SelectedCard {
    if opponent_hand_size == 0 {
        selected.deselect();
        return selected;
    }
    
    let card_index = select_opponent_card(opponent_hand_size).unwrap_or(0);
    selected.index = Some(card_index);
    selected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_opponent_card_empty_hand() {
        let result = select_opponent_card(0);
        assert!(result.is_none());
    }

    #[test]
    fn test_select_opponent_card_valid_range() {
        // Run multiple times to check distribution
        for _ in 0..10 {
            let index = select_opponent_card(5).unwrap();
            assert!(index < 5);
        }
    }

    #[test]
    fn test_opponent_select_card_with_hand() {
        let mut selected = crate::turn_state::SelectedCard::none();
        let result = opponent_select_card(3, selected);
        assert!(result.is_selected());
        assert!(result.index.unwrap() < 3);
    }

    #[test]
    fn test_opponent_select_card_empty_hand() {
        let selected = crate::turn_state::SelectedCard::new(2);
        let result = opponent_select_card(0, selected);
        assert!(!result.is_selected());
    }
}
