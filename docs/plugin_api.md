# Plugin API

## Goals
- Allow external plugins to extend server functionality.
- Provide safe, documented interfaces for plugins.

## Design
- Plugins loaded from `src/plugins/` or a `plugins/` directory.
- API exposed via `src/api/`.
- Core events: player join/leave, block place/break, command, etc.

## Safety
- Plugins run in sandboxed environment (planned).
- Document all public API functions.

## Expansion
- Add more hooks/events as needed.
- Document API changes in this file.
