//! Scissor: Custom Minecraft server shell
//! Loads static schematic-based maps, provides core world control, and is plugin-ready.


// mod schematic; // removed: no schematic support
mod world;
mod server;
pub mod core;

/// Entry point for the Scissor server
fn main() {
    println!("Scissor server starting...");
    // Load config
    let config = match core::config::Config::load("config.toml") {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Failed to load config: {}", e);
            return;
        }
    };
    println!("Loaded config: map_file={}, world_name={}", config.map_file, config.world_name);

    // Only load native mapfile
    match core::mapfile::MapFile::load_from_file(&config.map_file) {
        Ok(map) => {
            println!("Loaded mapfile: {}x{}x{} blocks", map.width, map.height, map.length);
            let world = map.to_world(&config.world_name);
            let mut manager = core::world_manager::WorldManager::new();
            manager.add_world(world);
            match manager.get_world(&config.world_name) {
                Some(w) => println!("World '{}' loaded with {} blocks.", config.world_name, w.blocks.len()),
                None => eprintln!("Warning: World '{}' not found after loading!", config.world_name),
            }
        },
        Err(e) => println!("Failed to load mapfile: {}", e),
    }
    // TODO: Start server loop, networking, etc.
}
