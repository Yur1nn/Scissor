//! World save/autosave and backup for Scissor
//! Provides manual and periodic world saving.

use crate::core::mapfile::MapFile;
use std::time::{Duration, Instant};

pub struct SaveManager {
    last_save: Instant,
    interval: Duration,
}

impl SaveManager {
    pub fn new(interval_secs: u64) -> Self {
        Self {
            last_save: Instant::now(),
            interval: Duration::from_secs(interval_secs),
        }
    }
    pub fn maybe_save(&mut self, map: &MapFile, path: &str) {
        if self.last_save.elapsed() >= self.interval {
            let _ = map.save_to_file(path);
            self.last_save = Instant::now();
        }
    }
    pub fn save_now(&mut self, map: &MapFile, path: &str) {
        let _ = map.save_to_file(path);
        self.last_save = Instant::now();
    }
}
