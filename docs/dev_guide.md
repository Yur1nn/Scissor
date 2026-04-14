# Developer Guide

## Getting Started
- Install Rust and clone the repository.
- Build with `cargo build`.
- Run with `cargo run`.

## Project Structure
- `src/core/`: Core server logic, world control, schematic loading
- `src/api/`: Public API for plugins
- `src/plugins/`: Plugin system and built-in plugins
- `src/utils/`: Utility functions
- `docs/`: Documentation

## Adding Features
- Add new modules/files in the appropriate directory.
- Document new features in `docs/features.md`.

## Coding Standards
- Use Rust 2021 edition.
- Document all public functions and modules.
- Write clear, maintainable code.

See `docs/architecture.md` for high-level design.