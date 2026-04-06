/// Resource for tracking combat events/log
use bevy::prelude::Resource;
use std::default::Default;

#[derive(Resource, Clone)]
pub struct CombatLog {
    pub entries: Vec<String>,
    /// Maximum number of entries to keep (oldest are removed when exceeded)
    pub max_entries: usize,
}

impl Default for CombatLog {
    fn default() -> Self {
        CombatLog {
            entries: Vec::new(),
            max_entries: 100,
        }
    }
}

impl CombatLog {
    /// Adds a new entry to the combat log
    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    /// Clears the combat log
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns an iterator over the log entries
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.entries.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_log_empty_initially() {
        let log = CombatLog::default();
        assert!(log.entries.is_empty());
        assert_eq!(log.max_entries, 100);
    }

    #[test]
    fn test_combat_log_add_entry() {
        let mut log = CombatLog::default();
        log.add_entry("Test entry".to_string());
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0], "Test entry");
    }

    #[test]
    fn test_combat_log_add_multiple_entries() {
        let mut log = CombatLog::default();
        log.add_entry("Entry 1".to_string());
        log.add_entry("Entry 2".to_string());
        log.add_entry("Entry 3".to_string());
        assert_eq!(log.entries.len(), 3);
        assert_eq!(log.entries[0], "Entry 1");
        assert_eq!(log.entries[2], "Entry 3");
    }

    #[test]
    fn test_combat_log_clear() {
        let mut log = CombatLog::default();
        log.add_entry("Entry 1".to_string());
        log.add_entry("Entry 2".to_string());
        log.clear();
        assert!(log.entries.is_empty());
    }

    #[test]
    fn test_combat_log_iter() {
        let mut log = CombatLog::default();
        log.add_entry("First".to_string());
        log.add_entry("Second".to_string());
        log.add_entry("Third".to_string());

        let mut iter_count = 0;
        for _entry in log.iter() {
            iter_count += 1;
        }
        assert_eq!(iter_count, 3);
    }

    #[test]
    fn test_combat_log_max_entries_rotation() {
        let mut log = CombatLog {
            entries: Vec::new(),
            max_entries: 3,
        };

        log.add_entry("Entry 1".to_string());
        log.add_entry("Entry 2".to_string());
        log.add_entry("Entry 3".to_string());
        assert_eq!(log.entries.len(), 3);

        log.add_entry("Entry 4".to_string());
        assert_eq!(log.entries.len(), 3);
        assert_eq!(log.entries[0], "Entry 2");
        assert_eq!(log.entries[2], "Entry 4");

        log.add_entry("Entry 5".to_string());
        assert_eq!(log.entries.len(), 3);
        assert_eq!(log.entries[0], "Entry 3");
        assert_eq!(log.entries[2], "Entry 5");
    }

    #[test]
    fn test_combat_log_add_entry_respects_max() {
        let mut log = CombatLog {
            entries: Vec::new(),
            max_entries: 2,
        };
        log.add_entry("First".to_string());
        log.add_entry("Second".to_string());
        log.add_entry("Third".to_string());

        assert_eq!(log.entries.len(), 2);
        assert_eq!(log.entries[0], "Second");
        assert_eq!(log.entries[1], "Third");
    }

    #[test]
    fn test_combat_log_add_entry_empty_string() {
        let mut log = CombatLog::default();
        log.add_entry(String::new());
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0].len(), 0);
    }
}
