# Map System

## Overview
- The server uses static maps loaded from WorldEdit `.schem`/`.schematic` files.
- No world generation is performed; maps are restored on every restart or reset.

## Map Lifecycle
1. On startup or reset, the map is loaded from the schematic file.
2. All changes are discarded on reset; the map returns to its original state.

## Supported Formats
- `.schem` (Sponge Schematic)
- `.schematic` (Classic)

## Adding/Converting Maps
- Use WorldEdit to export maps.
- Place schematic files in the designated directory (to be defined in config).

## Future Expansion
- Add support for more formats as needed.
- Map rotation and multi-arena support.
