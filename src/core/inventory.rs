//! Player inventory and item system for Scissor
//! Tracks player inventories and supports custom items.

use ahash::AHashMap;
use crate::core::item_model::ItemModel;

pub struct Inventory {
    pub items: AHashMap<String, u32>, // item id -> count
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: AHashMap::new() }
    }
    pub fn add(&mut self, item: &str, count: u32) {
        *self.items.entry(item.to_string()).or_insert(0) += count;
    }
    pub fn remove(&mut self, item: &str, count: u32) -> bool {
        if let Some(entry) = self.items.get_mut(item) {
            if *entry >= count {
                *entry -= count;
                if *entry == 0 { self.items.remove(item); }
                return true;
            }
        }
        false
    }
    pub fn count(&self, item: &str) -> u32 {
        *self.items.get(item).unwrap_or(&0)
    }
}

pub struct InventoryManager {
    inventories: AHashMap<String, Inventory>, // player uuid -> inventory
}

impl InventoryManager {
    pub fn new() -> Self {
        Self { inventories: AHashMap::new() }
    }
    pub fn get(&mut self, uuid: &str) -> &mut Inventory {
        self.inventories.entry(uuid.to_string()).or_insert_with(Inventory::new)
    }
}
