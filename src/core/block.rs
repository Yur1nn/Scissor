//! Block and semi-block representation for Scissor

use ahash::AHashMap;

/// Represents a block with type and properties (e.g., facing, waterlogged)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub name: String, // e.g., "minecraft:stone", "minecraft:chest"
    pub properties: AHashMap<String, String>, // e.g., "facing" => "north", "waterlogged" => "true"
}

/// Represents a semi-block (block entity) with minimal state (no inventory)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemiBlock {
    pub name: String, // e.g., "minecraft:item_frame", "minecraft:painting"
    pub properties: AHashMap<String, String>, // e.g., "facing" => "north"
    pub position: (i32, i32, i32), // x, y, z
}
