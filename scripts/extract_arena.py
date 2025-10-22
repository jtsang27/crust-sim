#!/usr/bin/env python3
"""
Arena Extraction Script
Extracts arena geometry and tile layout from legacy engine.
"""

import json
from pathlib import Path

OUTPUT_PATH = Path(__file__).parent.parent / 'config/patches/v2020_06/arena.json'

# Arena dimensions (from getTileInfo function analysis)
WIDTH = 32
HEIGHT = 18


def get_tile_type(i, j):
    """
    Port of getTileInfo function from legacy code (line 2334).
    Returns tile type for coordinates (i, j).
    """
    # i distance from middle
    true_i = abs(15.5 - i) - 0.5
    # j distance from middle
    true_j = abs(8.5 - j) - 0.5

    # Banned tiles (edges where troops can't go)
    if true_i == 15:
        if j < 6 or j > 11:
            return "wall"

    # Crown Tower (King Tower area)
    if 10 < true_i < 15 and true_j < 2:
        return "tower"

    # Princess Tower
    if 7 < true_i < 11 and 3 < true_j < 7:
        return "tower"

    # River (middle dividing line)
    if true_i < 1:
        # Bridge in middle
        if true_j == 5:
            return "bridge"
        # Water on sides
        else:
            return "river"

    # Default grass tiles
    return "grass"


def extract_arena():
    """Generate arena configuration from tile logic."""
    print(f'Generating {WIDTH}x{HEIGHT} arena layout...')

    # Generate tile grid
    tiles = []
    for j in range(HEIGHT):
        row = []
        for i in range(WIDTH):
            tile_type = get_tile_type(i, j)
            row.append(tile_type)
        tiles.append(row)

    # Define tower positions (based on tile analysis)
    # Player 1 is bottom, Player 2 is top
    arena_data = {
        "width": WIDTH,
        "height": HEIGHT,
        "tile_size": 1.0,
        "tiles": tiles,
        "towers": {
            "player1": {
                "king": {
                    "x": 15.5,
                    "y": 2.5,
                    "tile_x": 15,
                    "tile_y": 2
                },
                "left_princess": {
                    "x": 8.5,
                    "y": 5.5,
                    "tile_x": 8,
                    "tile_y": 5
                },
                "right_princess": {
                    "x": 22.5,
                    "y": 5.5,
                    "tile_x": 22,
                    "tile_y": 5
                }
            },
            "player2": {
                "king": {
                    "x": 15.5,
                    "y": 15.5,
                    "tile_x": 15,
                    "tile_y": 15
                },
                "left_princess": {
                    "x": 22.5,
                    "y": 12.5,
                    "tile_x": 22,
                    "tile_y": 12
                },
                "right_princess": {
                    "x": 8.5,
                    "y": 12.5,
                    "tile_x": 8,
                    "tile_y": 12
                }
            }
        },
        "spawn_zones": {
            "player1": {
                "min_x": 0.0,
                "min_y": 0.0,
                "max_x": 32.0,
                "max_y": 9.0
            },
            "player2": {
                "min_x": 0.0,
                "min_y": 9.0,
                "max_x": 32.0,
                "max_y": 18.0
            }
        }
    }

    # Save to JSON
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(arena_data, indent=2))
    print(f'✅ Saved arena configuration to {OUTPUT_PATH}')

    # Print summary
    tile_counts = {}
    for row in tiles:
        for tile in row:
            tile_counts[tile] = tile_counts.get(tile, 0) + 1

    print('\nTile Summary:')
    for tile_type, count in sorted(tile_counts.items()):
        print(f'  {tile_type}: {count}')

    return arena_data


if __name__ == '__main__':
    try:
        extract_arena()
    except Exception as e:
        print(f'❌ Extraction failed: {e}')
        import traceback
        traceback.print_exc()
        exit(1)
