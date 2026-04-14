# Architecture Overview

This document describes the high-level architecture of the Scissor Minecraft server shell.

## Main Components
- **Core:** Handles server lifecycle, networking, and world management.
- **Schematic Loader:** Loads and manages static maps from .schem/.schematic files.
- **World Control:** Manages time, PvP, block permissions, etc.
- **API:** Provides interfaces for plugins and extensions.
- **Plugins:** Dynamically loaded modules for extending server functionality.
- **Utils:** Utility functions and helpers.

## Expansion
- Each subsystem is modular and can be expanded by adding new files to the relevant directory.
- Follow the directory structure for organization.

See `docs/dev_guide.md` for more details.