# Scissor Project Copilot Instructions

## General Principles
- Always keep code modular and organized by subsystem (core, api, plugins, utils).
- All new features, modules, or major changes must be documented in the `docs/` folder.
- Use Rust 2021 idioms and best practices.
- All public functions, structs, and modules must have doc comments.
- When adding new files, update `docs/file_structure.md` and `docs/features.md` as needed.
- Never add world generation code; always use static schematic-based maps.
- All configuration must be handled via a config file (TOML/YAML/JSON).
- When in doubt, ask for clarification before making assumptions about game mechanics or API design.

## Documentation
- For every new feature or subsystem, create or update a corresponding Markdown file in `docs/`.
- Keep `README.md` and `docs/roadmap.md` up to date with project status and plans.

## Expansion
- When reorganizing or expanding, prefer creating new modules/files over growing existing ones too large.
- Use clear, descriptive names for all files and functions.

## Example Prompts
- "Add a new plugin system and document it."
- "Implement schematic loader and update docs/map_system.md."
- "Expand PvP mechanics and update docs/pvp_mechanics.md."

## Related Customizations to Consider
- Enforce code formatting with rustfmt.
- Add a PR template for contributions.
- Set up CI for build and test automation.
