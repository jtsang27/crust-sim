# Configuration Files

This directory contains all configuration data for the game engine, organized by patch version.

## Structure

```
config/
├── patches/           # Version-specific game data
│   └── v2020_06/      # Legacy baseline (June 2020)
│       ├── cards.json      # Card definitions and stats
│       ├── arena.json      # Arena geometry and tile types
│       └── mechanics.json  # Core game mechanics and constants
├── schemas/           # JSON schemas for validation
│   ├── card.schema.json
│   ├── arena.schema.json
│   └── mechanics.schema.json
└── patch_manifest.json # Version tracking
```

## Patch Versioning

Each patch folder (e.g., `v2020_06/`) represents a snapshot of game balance and mechanics.
This allows:
- Replays to be played back with historical game data
- A/B testing of balance changes
- Gradual migration from legacy to modern game versions

## Adding a New Patch

1. Create a new folder: `patches/vYYYY_MM/`
2. Copy files from the previous patch as a starting point
3. Update `patch_manifest.json` with the new version
4. Modify card stats, mechanics, or arena as needed
5. Validate against schemas in `schemas/`

## File Formats

All configuration files use JSON for:
- Human readability
- Easy version control
- Validation via JSON schemas
- Language-agnostic parsing
