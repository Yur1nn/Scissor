//! Player/session management for Scissor (multiplayer-ready)
//! Tracks online players, join/leave, and player data.

use ahash::AHashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: Box<str>,
    pub uuid: Box<str>,
    // Add more player data as needed (position, inventory, etc.)
}

pub struct PlayerManager {
    players: AHashMap<Box<str>, Player>, // uuid -> Player
}

impl PlayerManager {
    pub fn new() -> Self {
        Self { players: AHashMap::new() }
    }

    pub fn join(&mut self, name: &str, uuid: &str) -> &Player {
        self.players.entry(uuid.into()).or_insert_with(|| Player {
            name: name.into(),
            uuid: uuid.into(),
        })
    }

    pub fn leave(&mut self, uuid: &str) {
        self.players.remove(uuid);
    }

    pub fn get(&self, uuid: &str) -> Option<&Player> {
        self.players.get(uuid)
    }

    pub fn online(&self) -> impl Iterator<Item = &Player> {
        self.players.values()
    }
}
