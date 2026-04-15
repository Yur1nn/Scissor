# File Structure

## Source Code
- `src/core/` — Core server logic (main loop, world manager, config, item model)
- `src/api/` — Public API for plugins and extensions
- `src/plugins/` — Built-in and external plugins
- `src/utils/` — Utility modules and helpers
- `src/schematic.rs` — Schematic loader and map management
- `src/world.rs` — World control logic
- `src/server.rs` — Server core, PvP mode, plugin API stub
- `src/main.rs` — Entry point

## Configuration
- `config.toml` — Main server configuration

## Documentation
- `docs/` — All project documentation

## Expansion
- Add new modules/files in the appropriate directory as features grow.
- Keep documentation up to date with code changes.
