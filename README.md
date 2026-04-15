# Scissor

A custom Minecraft server shell written in Rust.

## Features
- Loads static maps from WorldEdit `.schem`/`.schematic` files
- Cross-platform: Windows & Linux
- Core world control (time, PvP, block permissions, etc.)
- Plugin-ready API (planned)
- Switchable PvP mechanics (1.8/modern)
- No world generation; always uses static schematic-based maps
- Designed for lobbies, arenas, mini-games, and more
- Permission system: per-player permissions and build mode toggle
- /buildmode command: enable/disable build mode for a player

## Project Structure
- `src/main.rs`: Entry point
- `src/world.rs`: World control logic
- `src/server.rs`: Server core, PvP mode, plugin API stub


## Schematic Format Requirements
- Only classic `.schematic` (NBT) files are supported.
- Export your schematic using WorldEdit or MCEdit as `.schematic` (not `.schem`).
- `.schem` (Sponge) format is **not supported yet**.

## Getting Started
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Build: `cargo build`
3. Run: `cargo run`

## Roadmap
- Implement schematic parsing
- Add networking and protocol support
- Develop plugin API
- Add configuration and admin commands
