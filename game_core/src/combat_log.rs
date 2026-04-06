/// Resource for tracking combat events/log
use bevy::prelude::Resource;
use std::default::Default;

#[derive(Resource)]
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
