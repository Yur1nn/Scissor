//! ItemModel system for custom items and content

use ahash::AHashMap;

/// Represents a custom item, weapon, armor, tool, furniture, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemModel {
    pub id: String, // Unique identifier, e.g., "custom:sword_of_power"
    pub display_name: String,
    pub item_type: ItemType,
    pub properties: AHashMap<String, String>, // e.g., "damage" => "10", "durability" => "250"
    // Add more fields as needed (e.g., texture, model, lore)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Weapon,
    Armor,
    Tool,
    Furniture,
    Misc,
}

/// Registry for all custom item models
pub struct ItemModelRegistry {
    pub items: AHashMap<String, ItemModel>,
}

impl ItemModelRegistry {
    pub fn new() -> Self {
        Self { items: AHashMap::new() }
    }
    pub fn register(&mut self, item: ItemModel) {
        self.items.insert(item.id.clone(), item);
    }
    pub fn get(&self, id: &str) -> Option<&ItemModel> {
        self.items.get(id)
    }
}
