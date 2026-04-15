//! Admin roles and permission groups for Scissor
//! Assigns admin status and groups to players.

use ahash::AHashSet;

pub struct AdminManager {
    admins: AHashSet<Box<str>>,
    groups: AHashSet<Box<str>>, // For future group support
}

impl AdminManager {
    pub fn new() -> Self {
        Self {
            admins: AHashSet::new(),
            groups: AHashSet::new(),
        }
    }

    pub fn add_admin(&mut self, player: &str) {
        self.admins.insert(player.into());
    }

    pub fn is_admin(&self, player: &str) -> bool {
        self.admins.contains(player)
    }
}
