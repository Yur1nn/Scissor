//! Custom map format for Scissor
//! This format is designed for fast loading and saving, and is independent of Minecraft's NBT or schematic formats.


use std::collections::HashMap;
use crate::core::block::{Block, SemiBlock};
use crate::core::world_manager;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use anyhow::Result;
use serde::{Serialize, Deserialize};


/// Represents a serializable map file (Scissor native format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapFile {
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub blocks: HashMap<(i32, i32, i32), Block>,
    pub semi_blocks: Vec<SemiBlock>,
}

impl MapFile {
    /// Load a map from a .scissormap file (e.g., using bincode or serde_json)
    pub fn load_from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let map: MapFile = serde_json::from_reader(reader)?;
        Ok(map)
    }

    /// Save a map to a .scissormap file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// Convert to a World instance
    pub fn to_world(&self, name: &str) -> world_manager::World {
        world_manager::World {
            name: name.to_string(),
            blocks: self.blocks.clone(),
            semi_blocks: self.semi_blocks.clone(),
        }
    }

    /// Create from a World instance
    pub fn from_world(world: &world_manager::World, width: u16, height: u16, length: u16) -> Self {
        MapFile {
            width,
            height,
            length,
            blocks: world.blocks.clone(),
            semi_blocks: world.semi_blocks.clone(),
        }
    }
}
