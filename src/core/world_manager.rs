//! World manager for handling multiple worlds

use crate::core::block::{Block, SemiBlock};
use std::collections::HashMap;

/// Represents a single world instance
pub struct World {
    pub name: String,
    pub blocks: HashMap<(i32, i32, i32), Block>,
    pub semi_blocks: Vec<SemiBlock>,
}

impl World {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            blocks: HashMap::new(),
            semi_blocks: Vec::new(),
        }
    }
}

/// Manages multiple loaded worlds
pub struct WorldManager {
    pub worlds: HashMap<String, World>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: HashMap::new(),
        }
    }
    pub fn add_world(&mut self, world: World) {
        self.worlds.insert(world.name.clone(), world);
    }
    pub fn get_world(&self, name: &str) -> Option<&World> {
        self.worlds.get(name)
    }
    pub fn get_world_mut(&mut self, name: &str) -> Option<&mut World> {
        self.worlds.get_mut(name)
    }
}
