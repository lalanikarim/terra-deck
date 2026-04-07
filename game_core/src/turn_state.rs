//! Turn state management resources

use bevy::prelude::Resource;

/// Tracks which card is currently selected by the player
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedCard {
    /// Index of selected card in hand, or None if nothing selected
    pub index: Option<usize>,
}

impl SelectedCard {
    /// Creates a new selected card tracker with no selection
    pub fn none() -> Self {
        SelectedCard { index: None }
    }

    /// Creates a selected card tracker with the given index
    pub fn new(index: usize) -> Self {
        SelectedCard { index: Some(index) }
    }

    /// Returns true if a card is currently selected
    pub fn is_selected(&self) -> bool {
        self.index.is_some()
    }

    /// Deselects the current card
    pub fn deselect(&mut self) {
        self.index = None;
    }

    /// Moves selection to the next card (wrapping)
    pub fn next(&mut self, max_index: usize) {
        if let Some(idx) = self.index {
            self.index = Some((idx + 1) % (max_index + 1));
        }
    }

    /// Moves selection to the previous card (wrapping)
    pub fn previous(&mut self, max_index: usize) {
        if let Some(idx) = self.index {
            if idx == 0 {
                self.index = Some(max_index);
            } else {
                self.index = Some(idx - 1);
            }
        }
    }

    /// Tries to update selection, returns true if valid
    pub fn try_set(&mut self, index: usize, max_index: usize) -> bool {
        if index <= max_index {
            self.index = Some(index);
            true
        } else {
            false
        }
    }

    /// Adjust selection after cards have been removed from hand
    /// Called when dead cards are removed to keep selection valid
    pub fn on_cards_removed(&mut self, cards_removed: usize) {
        if cards_removed == 0 {
            return;
        }
        if let Some(idx) = self.index {
            // If selection is beyond removal point, shift down
            // If selection was removed, clamp to new max
            self.index = Some(idx.saturating_sub(cards_removed));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selected_card_none() {
        let selected = SelectedCard::none();
        assert_eq!(selected.index, None);
        assert!(!selected.is_selected());
    }

    #[test]
    fn test_selected_card_new() {
        let selected = SelectedCard::new(2);
        assert_eq!(selected.index, Some(2));
        assert!(selected.is_selected());
    }

    #[test]
    fn test_selected_card_deselect() {
        let mut selected = SelectedCard::new(2);
        selected.deselect();
        assert_eq!(selected.index, None);
        assert!(!selected.is_selected());
    }

    #[test]
    fn test_selected_card_next() {
        let mut selected = SelectedCard::new(1);
        selected.next(4); // 5 cards total, max index = 4
        assert_eq!(selected.index, Some(2));
    }

    #[test]
    fn test_selected_card_next_wrap() {
        let mut selected = SelectedCard::new(4);
        selected.next(4); // At max index, should wrap to 0
        assert_eq!(selected.index, Some(0));
    }

    #[test]
    fn test_selected_card_previous() {
        let mut selected = SelectedCard::new(2);
        selected.previous(4);
        assert_eq!(selected.index, Some(1));
    }

    #[test]
    fn test_selected_card_previous_wrap() {
        let mut selected = SelectedCard::new(0);
        selected.previous(4); // At min, should wrap to max
        assert_eq!(selected.index, Some(4));
    }

    #[test]
    fn test_selected_card_try_set_valid() {
        let mut selected = SelectedCard::none();
        let result = selected.try_set(2, 4);
        assert!(result);
        assert_eq!(selected.index, Some(2));
    }

    #[test]
    fn test_selected_card_try_set_invalid() {
        let mut selected = SelectedCard::none();
        let result = selected.try_set(10, 4); // index > max
        assert!(!result);
        assert_eq!(selected.index, None);
    }

    #[test]
    fn test_selected_card_none_next() {
        let mut selected = SelectedCard::none();
        selected.next(4);
        // If no selection, next doesn't change anything
        assert_eq!(selected.index, None);
    }

    #[test]
    fn test_selected_card_none_previous() {
        let mut selected = SelectedCard::none();
        selected.previous(4);
        // If no selection, previous doesn't change anything
        assert_eq!(selected.index, None);
    }
}
