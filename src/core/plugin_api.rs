//! Simple plugin API for Scissor
//! Allows registering commands and event hooks.

use crate::core::block_event::{BlockEvent, BlockEventHandler};

pub trait Plugin {
    fn name(&self) -> &str;
    fn on_block_event(&self, _event: &BlockEvent) {}
    // Add more hooks as needed
}

pub struct PluginManager<'a> {
    plugins: Vec<&'a dyn Plugin>,
}

impl<'a> PluginManager<'a> {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }
    pub fn register(&mut self, plugin: &'a dyn Plugin) {
        self.plugins.push(plugin);
    }
    pub fn dispatch_block_event(&self, event: &BlockEvent) {
        for p in &self.plugins {
            p.on_block_event(event);
        }
    }
}
