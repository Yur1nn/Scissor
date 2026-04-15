//! Permission system for Scissor
//! Provides per-player permissions and build mode control.

use ahash::{AHashMap, AHashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    Build,
    // Add more permissions as needed
}

#[derive(Debug, Default)]
pub struct Permissions {
    player_perms: AHashMap<Box<str>, AHashSet<Permission>>, // player name -> permissions
    build_mode: AHashSet<Box<str>>, // players with build mode enabled
}

impl Permissions {
    pub fn new() -> Self {
        Self {
            player_perms: AHashMap::new(),
            build_mode: AHashSet::new(),
        }
    }

    #[inline]
    pub fn grant(&mut self, player: &str, perm: Permission) {
        self.player_perms.entry(player.into()).or_default().insert(perm);
    }

    #[inline]
    pub fn revoke(&mut self, player: &str, perm: &Permission) {
        if let Some(perms) = self.player_perms.get_mut(player) {
            perms.remove(perm);
        }
    }

    #[inline]
    pub fn has(&self, player: &str, perm: &Permission) -> bool {
        self.player_perms.get(player).map_or(false, |perms| perms.contains(perm))
    }

    #[inline]
    pub fn enable_build_mode(&mut self, player: &str) {
        self.build_mode.insert(player.into());
    }

    #[inline]
    pub fn disable_build_mode(&mut self, player: &str) {
        self.build_mode.remove(player);
    }

    #[inline]
    pub fn is_build_mode(&self, player: &str) -> bool {
        self.build_mode.contains(player)
    }
}
