//! World manager for handling multiple worlds

use crate::core::block::{Block, SemiBlock};
use crate::core::mapfile;
use ahash::AHashMap;

/// Represents a single world instance
pub struct World {
    pub name: String,
    pub blocks: AHashMap<(i32, i32, i32), Block>,
    pub semi_blocks: Vec<SemiBlock>,
}

impl World {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            blocks: AHashMap::new(),
            semi_blocks: Vec::new(),
        }
    }

    /// Export this world to a MapFile (Scissor native format)
    pub fn to_mapfile(&self, width: u16, height: u16, length: u16) -> mapfile::MapFile {
        mapfile::MapFile::from_world(self, width, height, length)
    }
}

/// Manages multiple loaded worlds
pub struct WorldManager {
    pub worlds: AHashMap<Box<str>, World>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: AHashMap::new(),
        }
    }
    #[inline]
    pub fn add_world(&mut self, world: World) {
        self.worlds.insert(world.name.clone().into_boxed_str(), world);
    }
    #[inline]
    pub fn get_world(&self, name: &str) -> Option<&World> {
        self.worlds.get(name)
    }
    #[inline]
    pub fn get_world_mut(&mut self, name: &str) -> Option<&mut World> {
        self.worlds.get_mut(name)
    }
}
