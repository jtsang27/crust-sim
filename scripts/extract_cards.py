#!/usr/bin/env python3
"""
Card Extraction Script
Extracts card definitions from the legacy JavaScript engine into JSON format.
"""

import json
import re
from pathlib import Path

LEGACY_CODE_PATH = Path('/Users/will/Documents/Projects/clash-royale-engine/code.txt')
OUTPUT_PATH = Path(__file__).parent.parent / 'config/patches/v2020_06/cards.json'


def parse_js_string(s):
    """Extract string from JavaScript string literal."""
    if s.startswith('"') and s.endswith('"'):
        return s[1:-1]
    if s.startswith("'") and s.endswith("'"):
        return s[1:-1]
    return s


def extract_cards():
    """Extract all cards from code.txt using regex parsing."""
    print('Reading legacy code.txt...')
    code = LEGACY_CODE_PATH.read_text()

    # Find the cards array - match line by line to avoid issues
    cards_start = code.find('cardsInit = function(){')
    if cards_start == -1:
        raise ValueError('Could not find cardsInit function')

    cards_start = code.find('cards = [', cards_start)
    if cards_start == -1:
        raise ValueError('Could not find cards array')

    # Find the end of the array (matching brackets)
    bracket_count = 0
    i = cards_start + len('cards = ')
    start_bracket = i
    while i < len(code):
        if code[i] == '[':
            bracket_count += 1
        elif code[i] == ']':
            bracket_count -= 1
            if bracket_count == 0:
                cards_array_str = code[start_bracket:i+1]
                break
        i += 1

    print(f'Found cards array ({len(cards_array_str)} characters)')

    # Extract individual card entries using regex
    # Pattern: ["Name", "Description", number, [[troop data...]]]
    # We'll extract them one at a time

    cards = []

    # Find all top-level card entries
    # Each card starts with ["
    pattern = r'\["([^"]*)",\s*"([^"]*)",\s*(\d+),\s*\[(.*?)\]\s*(?:,\s*"(spell)")?\](?=\s*,\s*//|\s*\])'

    # This is complex - let's use a simpler line-by-line approach
    lines = cards_array_str.split('\n')

    card_id = 0
    for line in lines:
        line = line.strip()
        if not line or line.startswith('//') or line == '[' or line == '];':
            continue

        # Match card pattern: ["Name", "Description"
        if line.startswith('["'):
            # Extract name and description
            match = re.match(r'\["([^"]*)",\s*"([^"]*)",\s*(\d+)', line)
            if match:
                name = match.group(1)
                description = match.group(2)
                elixir_cost = int(match.group(3))

                # Basic card data (we'll enhance this later by reading the actual troop data)
                card = {
                    'id': card_id,
                    'name': name,
                    'type': 'troop',  # Default, will be refined
                    'rarity': infer_rarity(name, elixir_cost),
                    'elixir_cost': elixir_cost,
                    'description': description,
                    'stats': {},
                    'legacy_data': {
                        'card_index': card_id,
                        'line': line[:100]  # Store snippet for debugging
                    }
                }

                cards.append(card)
                card_id += 1

    print(f'Extracted {len(cards)} cards')

    # Save to JSON
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(cards, indent=2))
    print(f'✅ Saved {len(cards)} cards to {OUTPUT_PATH}')

    # Print summary
    by_type = {}
    for card in cards:
        card_type = card['type']
        by_type[card_type] = by_type.get(card_type, 0) + 1

    print('\nCard Summary:')
    for card_type, count in sorted(by_type.items()):
        print(f'  {card_type}: {count}')

    return cards


def infer_rarity(name, cost):
    """Infer rarity from elixir cost (can be corrected manually later)."""
    # Known legendaries
    legendaries = ['Ice Wizard', 'Princess', 'Lumberjack', 'Miner', 'Sparky',
                   'Lava Hound', 'Electro Wizard', 'Inferno Dragon', 'Graveyard',
                   'The Log', 'Bandit', 'Night Witch', 'Royal Ghost', 'Magic Archer',
                   'Mega Knight', 'Ram Rider', 'Fisherman']

    if name in legendaries:
        return 'legendary'

    if cost <= 2:
        return 'common'
    if cost <= 4:
        return 'rare'
    if cost <= 6:
        return 'epic'
    return 'legendary'


if __name__ == '__main__':
    try:
        extract_cards()
    except Exception as e:
        print(f'❌ Extraction failed: {e}')
        import traceback
        traceback.print_exc()
        exit(1)
