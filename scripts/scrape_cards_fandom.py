#!/usr/bin/env python3
"""
Clash Royale Card Scraper - Fandom Wiki
Scrapes all card data including stats for all levels and special effects.
"""

import json
import re
import time
from pathlib import Path
from typing import Dict, List, Optional

try:
    import requests
    from bs4 import BeautifulSoup
except ImportError:
    print("ERROR: Required packages not installed")
    print("Run: pip install requests beautifulsoup4")
    exit(1)

BASE_URL = "https://clashroyale.fandom.com"
CARDS_LIST_URL = f"{BASE_URL}/wiki/Cards"
OUTPUT_PATH = Path(__file__).parent.parent / "config/patches/v2025_current/cards_complete.json"

# Rate limiting
REQUEST_DELAY = 1.0  # seconds between requests


def fetch_page(url: str) -> Optional[BeautifulSoup]:
    """Fetch and parse a wiki page."""
    try:
        print(f"Fetching: {url}")
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        time.sleep(REQUEST_DELAY)  # Be respectful to the server
        return BeautifulSoup(response.content, 'html.parser')
    except Exception as e:
        print(f"  ERROR fetching {url}: {e}")
        return None


def extract_card_links(soup: BeautifulSoup) -> List[Dict[str, str]]:
    """Extract all card links from the main Cards page."""
    cards = []

    # Find card tables/sections - they usually have class 'article-table' or 'wikitable'
    card_tables = soup.find_all('table', class_=['article-table', 'wikitable'])

    # Also look for card galleries/grids
    card_sections = soup.find_all('div', class_=['card-gallery', 'card-list'])

    all_containers = card_tables + card_sections

    # Exclude these from card names
    excluded_terms = [
        'edit', 'history', 'purge', 'action=', 'view source', 'talk',
        'cards', 'card', 'category', 'template', 'file', 'special:',
        'help:', 'main page', 'clash royale', 'click here', 'more'
    ]

    # Exclude variant cards (Evolution, Merge Tactics, etc.)
    variant_paths = ['/Evolution', '/Merge_Tactics', '/Tower_Troop']

    for container in all_containers:
        for link in container.find_all('a', href=True):
            href = link['href']
            name = link.get_text().strip()

            # Filter out non-card links
            if not name or len(name) < 2:
                continue

            # Must be a wiki link
            if '/wiki/' not in href:
                continue

            # Check against excluded terms
            if any(term in href.lower() or term in name.lower() for term in excluded_terms):
                continue

            # Must not have query parameters or anchors
            if '?' in href or '#' in href:
                continue

            # Clean href
            clean_href = href.split('?')[0].split('#')[0]

            # Filter out variant cards
            if any(variant in clean_href for variant in variant_paths):
                continue

            cards.append({
                'name': name,
                'url': f"{BASE_URL}{clean_href}" if clean_href.startswith('/') else clean_href
            })

    # Deduplicate by URL
    seen = set()
    unique_cards = []
    for card in cards:
        if card['url'] not in seen:
            seen.add(card['url'])
            unique_cards.append(card)

    return unique_cards


def parse_stat_value(text: str) -> Optional[float]:
    """Parse a stat value, handling commas and special cases."""
    if not text or text == 'N/A' or text == '-':
        return None
    # Remove commas and parse
    cleaned = text.replace(',', '').strip()
    try:
        return float(cleaned)
    except ValueError:
        return None


def parse_number_with_units(text: str) -> Optional[float]:
    """Extract numeric value from text with units (e.g., '1.5sec' -> 1.5, 'Fast (1000)' -> 1000.0)."""
    if not text or text == 'N/A' or text == '-':
        return None

    # Try to extract number from parentheses first (e.g., "Fast (1000)")
    paren_match = re.search(r'\(([0-9.,]+)\)', text)
    if paren_match:
        return parse_stat_value(paren_match.group(1))

    # Extract first numeric value (handles "1.5sec", "3 tiles", etc.)
    number_match = re.search(r'([0-9]+\.?[0-9]*)', text)
    if number_match:
        return float(number_match.group(1))

    return None


def parse_speed_category(text: str) -> Optional[str]:
    """Extract speed category (slow/medium/fast/very fast) from text."""
    text_lower = text.lower()
    if 'very fast' in text_lower:
        return 'very_fast'
    elif 'fast' in text_lower:
        return 'fast'
    elif 'medium' in text_lower:
        return 'medium'
    elif 'slow' in text_lower:
        return 'slow'
    return None


def extract_infobox_data(soup: BeautifulSoup, debug: bool = False) -> Dict:
    """Extract data from the card infobox (right side panel)."""
    data = {}

    infobox = soup.find('aside', class_='portable-infobox')
    if not infobox:
        if debug:
            print("    DEBUG: No infobox found with class 'portable-infobox'")
        return data

    if debug:
        print("    DEBUG: Found infobox")

    # Extract key-value pairs
    for row in infobox.find_all('div', class_='pi-item'):
        label_elem = row.find('h3', class_='pi-data-label')
        value_elem = row.find('div', class_='pi-data-value')

        if label_elem and value_elem:
            label = label_elem.get_text().strip()
            value = value_elem.get_text().strip()

            if debug:
                print(f"    DEBUG: Label='{label}', Value='{value}'")

            # Map labels to our schema with improved parsing
            # Note: Infobox "Elixir Cost" for champions is ability cost, not card cost
            # We'll get the real elixir cost from the stats table instead
            if 'Elixir' in label and 'Ability' not in label:
                # Only set if not already set by stats table
                if 'elixir_cost' not in data:
                    data['elixir_cost'] = parse_stat_value(value)
            elif 'Ability' in label and 'Elixir' in label:
                data['ability_cost'] = parse_stat_value(value)

            elif 'Rarity' in label:
                data['rarity'] = value.lower()

            elif 'Type' in label:
                data['card_type'] = value.lower()

            elif 'Target' in label:
                data['targets'] = value.lower()

            elif 'Range' in label:
                data['range'] = parse_number_with_units(value)

            elif 'Speed' in label and 'Hit' not in label and 'Projectile' not in label:
                # Movement speed (e.g., "Medium", "Fast", "Slow")
                speed_cat = parse_speed_category(value)
                if speed_cat:
                    data['movement_speed'] = speed_cat
                else:
                    data['movement_speed'] = value.lower()

            elif 'Deploy Time' in label or 'Spawn Time' in label:
                data['deploy_time'] = parse_number_with_units(value)

            elif 'Hit Speed' in label or 'Attack Speed' in label:
                data['attack_speed'] = parse_number_with_units(value)

            elif 'Damage Speed' in label or 'First Hit' in label:
                # Some cards have different first hit timing
                data['first_hit_speed'] = parse_number_with_units(value)

            elif 'Count' in label or 'Troop Count' in label:
                count_val = parse_number_with_units(value)
                if count_val:
                    data['count'] = int(count_val)

            elif 'Projectile Speed' in label:
                # Store both numeric value and category
                speed_num = parse_number_with_units(value)
                speed_cat = parse_speed_category(value)
                if speed_num:
                    data['projectile_speed'] = speed_num
                if speed_cat:
                    data['projectile_speed_category'] = speed_cat

            elif 'Transport' in label:
                data['transport'] = value.lower()

            elif 'Duration' in label and 'Lifetime' not in label:
                # For spells and effects
                data['duration'] = parse_number_with_units(value)

            elif 'Radius' in label or 'Area' in label and 'Damage' not in label:
                # Spell radius or area of effect
                data['radius'] = parse_number_with_units(value)

    return data


def extract_card_properties(soup: BeautifulSoup, debug: bool = False) -> Dict:
    """Extract card properties from the statistics table (not level-based stats)."""
    properties = {}

    # Find tables that contain card statistics
    tables = soup.find_all('table', class_='wikitable')

    if debug:
        print(f"    DEBUG: Found {len(tables)} wikitables for properties")

    for table in tables:
        rows = table.find_all('tr')

        # Check if this is a wide property table (headers in first row, values in second row)
        if len(rows) >= 2:
            header_cells = rows[0].find_all(['th', 'td'])
            value_cells = rows[1].find_all(['th', 'td']) if len(rows) > 1 else []

            # Wide format: multiple properties as columns
            if len(header_cells) > 4 and len(value_cells) >= len(header_cells):
                if debug:
                    print(f"    DEBUG: Found wide property table with {len(header_cells)} columns")

                for i, header_cell in enumerate(header_cells):
                    if i >= len(value_cells):
                        break

                    label_text = header_cell.get_text().strip()
                    value_text = value_cells[i].get_text().strip()

                    if debug:
                        print(f"    DEBUG PROP: '{label_text}' = '{value_text}'")

                    # Parse properties
                    _parse_property(label_text, value_text, properties)
                continue

        # Also check for two-column format (label | value)
        for row in rows:
            cells = row.find_all(['th', 'td'])
            if len(cells) == 2:
                label_text = cells[0].get_text().strip()
                value_text = cells[1].get_text().strip()

                if debug:
                    print(f"    DEBUG PROP: '{label_text}' = '{value_text}'")

                _parse_property(label_text, value_text, properties)

    return properties


def _parse_property(label_text: str, value_text: str, properties: Dict) -> None:
    """Parse a single property label/value pair and add to properties dict."""
    # Parse elixir cost from stats table (overrides infobox for champions)
    if label_text == 'Cost':
        properties['elixir_cost'] = parse_number_with_units(value_text)

    # Parse both Hit Speed and First Hit Speed - they're different!
    # First Hit Speed = initial attack delay after deployment
    # Hit Speed = time between subsequent attacks
    elif 'First Hit Speed' in label_text or 'First Hit' in label_text:
        properties['first_hit_speed'] = parse_number_with_units(value_text)
    elif 'Hit Speed' in label_text or 'Attack Speed' in label_text:
        properties['attack_speed'] = parse_number_with_units(value_text)
    elif 'Speed' in label_text and 'Hit' not in label_text and 'Projectile' not in label_text:
        speed_cat = parse_speed_category(value_text)
        speed_num = parse_number_with_units(value_text)
        if speed_cat:
            properties['movement_speed'] = speed_cat
        if speed_num:
            properties['movement_speed_value'] = speed_num
    elif 'Deploy Time' in label_text or 'Spawn Time' in label_text:
        properties['deploy_time'] = parse_number_with_units(value_text)
    elif 'Range' in label_text:
        properties['range'] = parse_number_with_units(value_text)
    elif 'Projectile Speed' in label_text:
        speed_num = parse_number_with_units(value_text)
        speed_cat = parse_speed_category(value_text)
        if speed_num:
            properties['projectile_speed'] = speed_num
        if speed_cat:
            properties['projectile_speed_category'] = speed_cat
    elif 'Target' in label_text:
        # Convert targets to list format: "Air & Ground" -> ["air", "ground"]
        targets_lower = value_text.lower()
        targets_list = []
        if 'air' in targets_lower:
            targets_list.append('air')
        if 'ground' in targets_lower:
            targets_list.append('ground')
        if 'building' in targets_lower:
            targets_list.append('buildings')
        if targets_list:
            properties['targets'] = targets_list
        else:
            # Fallback to original value if we can't parse it
            properties['targets'] = [targets_lower]
    elif 'Count' in label_text or 'Troop Count' in label_text:
        count_val = parse_number_with_units(value_text)
        if count_val:
            properties['count'] = int(count_val)
        # Also extract from "x2", "x3" format
        count_match = re.search(r'x(\d+)', value_text)
        if count_match:
            properties['count'] = int(count_match.group(1))
    elif 'Transport' in label_text:
        properties['transport'] = value_text.lower()
    elif 'Duration' in label_text or 'Lifetime' in label_text:
        properties['duration'] = parse_number_with_units(value_text)
    elif 'Radius' in label_text:
        properties['radius'] = parse_number_with_units(value_text)


def extract_stats_table(soup: BeautifulSoup) -> List[Dict]:
    """Extract the level-based stats table."""
    levels = []

    # Find the stats table (usually has headers: Level, Hitpoints, Damage, etc.)
    tables = soup.find_all('table', class_='wikitable')

    for table in tables:
        headers = []
        header_row = table.find('tr')
        if header_row:
            headers = [th.get_text().strip().lower() for th in header_row.find_all('th')]

        # Check if this looks like a stats table
        if 'level' not in headers:
            continue

        # Parse each row
        for row in table.find_all('tr')[1:]:  # Skip header
            cols = row.find_all('td')
            if not cols:
                continue

            level_data = {}
            for i, col in enumerate(cols):
                if i >= len(headers):
                    break

                header = headers[i]
                value_text = col.get_text().strip()

                if 'level' in header:
                    level_data['level'] = int(parse_stat_value(value_text) or 0)
                elif 'hitpoint' in header or 'hp' in header or 'health' in header:
                    level_data['hp'] = parse_stat_value(value_text)
                elif 'damage' in header and 'per second' not in header and 'area' not in header:
                    level_data['damage'] = parse_stat_value(value_text)
                elif 'area damage' in header:
                    level_data['area_damage'] = parse_stat_value(value_text)
                elif 'dps' in header or 'damage per second' in header:
                    level_data['dps'] = parse_stat_value(value_text)
                elif 'spawn damage' in header:
                    level_data['spawn_damage'] = parse_stat_value(value_text)
                elif 'shield' in header:
                    level_data['shield_hp'] = parse_stat_value(value_text)
                elif 'heal' in header or 'healing' in header:
                    level_data['healing'] = parse_stat_value(value_text)

            if level_data.get('level'):
                levels.append(level_data)

    return levels


def extract_effects(soup: BeautifulSoup) -> List[str]:
    """Extract special effects from card description (slow, stun, rage, etc.)."""
    effects = []

    # Common effect keywords to look for
    effect_keywords = {
        'slow': 'slows',
        'stun': 'stuns',
        'rage': 'rage',
        'freeze': 'freeze',
        'heal': 'heal',
        'spawn': 'spawn',
        'death_damage': 'death',
        'shield': 'shield',
        'clone': 'clone',
        'invisibility': 'invisible',
        'knockback': 'knockback',
        'dash': 'dash',
        'charge': 'charge',
        'splash': 'splash',
        'area': 'area',
    }

    # Get card description from infobox or main content
    description = ''

    # Try to find description in infobox
    infobox = soup.find('aside', class_='portable-infobox')
    if infobox:
        desc_elem = infobox.find('div', class_='pi-data-value')
        if desc_elem:
            description += ' ' + desc_elem.get_text().lower()

    # Also check the main content paragraphs
    content = soup.find('div', class_='mw-parser-output')
    if content:
        for p in content.find_all('p', limit=3):  # Check first 3 paragraphs
            description += ' ' + p.get_text().lower()

    # Search for effect keywords
    for effect_name, keyword in effect_keywords.items():
        if keyword in description:
            effects.append(effect_name)

    return effects


def extract_card_data(card_url: str, card_name: str, debug: bool = False) -> Optional[Dict]:
    """Extract all data for a single card."""
    soup = fetch_page(card_url)
    if not soup:
        return None

    print(f"  Parsing: {card_name}")

    card_data = {
        'name': card_name,
        'url': card_url,
    }

    # Get infobox data (basic properties like elixir, rarity, type)
    infobox_data = extract_infobox_data(soup, debug=debug)
    card_data.update(infobox_data)

    # Get card properties (range, speed, targets, etc. from statistics table)
    properties = extract_card_properties(soup, debug=debug)
    card_data.update(properties)

    # Get stats table (per-level stats)
    stats_by_level = extract_stats_table(soup)
    card_data['levels'] = stats_by_level

    # Extract special effects
    effects = extract_effects(soup)
    if effects:
        card_data['effects'] = effects

    return card_data


def main():
    print("=== Clash Royale Card Scraper (Fandom Wiki) ===\n")

    # Step 1: Get list of all cards
    print("Step 1: Fetching card list...")
    soup = fetch_page(CARDS_LIST_URL)
    if not soup:
        print("ERROR: Could not fetch cards list page")
        return

    card_links = extract_card_links(soup)
    print(f"Found {len(card_links)} potential cards\n")

    # Step 2: Scrape each card
    print("Step 2: Scraping individual card pages...")
    print(f"(Scraping {len(card_links)} cards - excluding champions and evolutions)\n")

    all_cards = []
    for i, card_link in enumerate(card_links):
        # Enable debug mode for first card only
        card_data = extract_card_data(card_link['url'], card_link['name'], debug=(i == 0))
        if card_data:
            all_cards.append(card_data)
        print(f"  Progress: {i+1}/{len(card_links)}")

    # Filter out champions and special tower troops (they require special ability implementation)
    # Keep Tower Princess as it's the base tower troop everyone has
    special_tower_troops = ['Cannoneer', 'Dagger Duchess', 'Royal Chef']
    base_cards = [
        card for card in all_cards
        if card.get('rarity') != 'champion'
        and card.get('name') not in special_tower_troops
    ]
    excluded_count = len(all_cards) - len(base_cards)
    if excluded_count > 0:
        print(f"\n  Excluded {excluded_count} champion/special tower troop cards (require custom ability code)")

    # Step 3: Save to JSON
    print(f"\nStep 3: Saving {len(base_cards)} cards to JSON...")
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(json.dumps(base_cards, indent=2))

    print(f"✅ Scraping complete! Saved to {OUTPUT_PATH}")

    # Print summary
    print("\n=== Summary ===")
    print(f"Total cards scraped: {len(base_cards)}")
    for card in base_cards[:3]:  # Show first 3
        print(f"\n{card['name']}:")
        print(f"  Elixir: {card.get('elixir_cost', 'N/A')}")
        print(f"  Type: {card.get('card_type', 'N/A')}")
        print(f"  Rarity: {card.get('rarity', 'N/A')}")
        print(f"  Levels: {len(card.get('levels', []))}")

        # Show property details
        if card.get('range'):
            print(f"  Range: {card.get('range')}")
        if card.get('attack_speed'):
            print(f"  Attack Speed: {card.get('attack_speed')}")
        if card.get('movement_speed'):
            print(f"  Movement Speed: {card.get('movement_speed')}")
        if card.get('targets'):
            print(f"  Targets: {card.get('targets')}")
        if card.get('count'):
            print(f"  Count: {card.get('count')}")
        if card.get('effects'):
            print(f"  Effects: {', '.join(card.get('effects', []))}")


if __name__ == '__main__':
    main()
