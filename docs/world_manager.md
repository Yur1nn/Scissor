# World Manager

## Overview
The world manager handles multiple loaded worlds (instances) in memory. Each world contains only block and semi-block (block entity) states—no inventory or storage data.

## Block Representation
- Each block stores its type (e.g., `minecraft:stone`) and all relevant properties (e.g., facing, waterlogged, wall states).
- Semi-blocks (block entities like item frames, paintings, etc.) store type, properties, and position, but do not include inventory or storage data.

## Features
- Add, remove, and access worlds by name.
- Efficient lookup of blocks by position.
- Designed for static schematic-based maps and fast resets.

## Limitations
- Does not store inventory or storage data for chests, barrels, shulkers, furnaces, etc.
- Only supports block and semi-block state for world restoration.

## Expansion
- Add more world rules or features as needed.
- Update this documentation with any changes to world management.
