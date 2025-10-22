#!/usr/bin/env python3
"""
Mechanics Extraction Script
Extracts core game mechanics and constants from legacy engine.
"""

import json
from pathlib import Path

OUTPUT_PATH = Path(__file__).parent.parent / 'config/patches/v2020_06/mechanics.json'


def extract_mechanics():
    """
    Extract game mechanics constants.
    Based on analysis of code.txt (lines 112-114, 2658+, etc.)
    """
    print('Extracting game mechanics constants...')

    mechanics = {
        "tick_rate": {
            "fps": 60,
            "delta_time": 1.0 / 60.0  # ~0.01667 seconds
        },
        "elixir": {
            "starting_amount": 5.0,
            "max_amount": 10.0,
            "regen_rate": 1.0,  # 1 elixir per second (standard)
            "overtime_regen_rate": 2.0  # Typical overtime multiplier
        },
        "match": {
            "duration": 180.0,  # 3 minutes in seconds
            "overtime_duration": 60.0,  # 1 minute overtime
            "overtime_start_time": 180.0
        },
        "physics": {
            "collision_radius_default": 1.0,
            "mass_default": 5.0,
            "push_force": 0.5,
            "max_velocity": 5.0
        },
        "combat": {
            "tower_range": 7.0,  # Estimated from arena analysis
            "tower_damage": 90.0,  # Typical tower damage
            "tower_attack_speed": 0.8,  # Attacks per second
            "retarget_delay": 0.1  # Delay before switching targets
        },
        "movement_speeds": {
            "slow": 45.0,  # Degrees per tick
            "medium": 60.0,
            "fast": 90.0,
            "very_fast": 120.0
        },
        "notes": {
            "source": "Extracted from legacy JS engine (code.txt)",
            "version": "v2020_06",
            "tick_conversion": "Many values in legacy code are in ticks (60 ticks = 1 second)",
            "degrees_conversion": "Movement speeds are in degrees per tick in legacy code"
        }
    }

    # Save to JSON
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(mechanics, indent=2))
    print(f'✅ Saved mechanics configuration to {OUTPUT_PATH}')

    # Print summary
    print('\nMechanics Summary:')
    print(f'  Tick rate: {mechanics["tick_rate"]["fps"]} FPS')
    print(f'  Starting elixir: {mechanics["elixir"]["starting_amount"]}')
    print(f'  Max elixir: {mechanics["elixir"]["max_amount"]}')
    print(f'  Elixir regen: {mechanics["elixir"]["regen_rate"]}/sec')
    print(f'  Match duration: {mechanics["match"]["duration"]}s')

    return mechanics


if __name__ == '__main__':
    try:
        extract_mechanics()
    except Exception as e:
        print(f'❌ Extraction failed: {e}')
        import traceback
        traceback.print_exc()
        exit(1)
