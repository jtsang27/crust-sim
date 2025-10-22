# Documentation Index

This directory contains all project documentation organized by category.

## Quick Links

- **[Main README](../README.md)** - Project overview
- **[CLAUDE.md](../CLAUDE.md)** - Guide for Claude Code instances
- **[ROADMAP.md](../ROADMAP.md)** - Full 8-phase development plan

## Legacy Engine Analysis

Documentation from analyzing the original JavaScript Clash Royale engine:

- **[README](./legacy-analysis/README.md)** - Overview and navigation guide
- **[CLASH_ROYALE_ENGINE_ANALYSIS.md](./legacy-analysis/CLASH_ROYALE_ENGINE_ANALYSIS.md)** - Detailed technical analysis (630 lines)
- **[CLASH_ROYALE_ENGINE_EXTRACTION_GUIDE.md](./legacy-analysis/CLASH_ROYALE_ENGINE_EXTRACTION_GUIDE.md)** - Extraction templates and checklists (452 lines)
- **[EXPLORATION_SUMMARY.txt](./legacy-analysis/EXPLORATION_SUMMARY.txt)** - Quick reference with line numbers (264 lines)

## Phase Summaries

Progress reports for each development phase:

- **[Phase 1: Legacy Analysis & Data Extraction](./phases/PHASE_1_SUMMARY.md)** ✅ **COMPLETE**
  - Extracted 97 cards, 32×18 arena layout, game mechanics
  - Created JSON schemas and extraction tools
  - Built configuration system for patch versioning

## Project Structure

```
crust-sim/
├── docs/                      # All documentation (you are here)
│   ├── legacy-analysis/       # Legacy JS engine analysis
│   ├── phases/                # Phase completion summaries
│   └── README.md              # This file
├── config/                    # Configuration files
│   ├── patches/               # Version-specific game data
│   │   └── v2020_06/          # Legacy baseline (June 2020)
│   └── schemas/               # JSON validation schemas
├── scripts/                   # Data extraction tools
│   ├── extract_cards.py
│   ├── extract_arena.py
│   └── extract_mechanics.py
├── engine/                    # Core Rust simulation engine
├── shared/                    # Shared data structures
├── ROADMAP.md                 # 8-phase development plan
└── CLAUDE.md                  # Claude Code guide
```

## Development Phases

Follow the [ROADMAP.md](../ROADMAP.md) for the complete development plan:

1. **Phase 1:** Legacy Analysis & Data Extraction ✅
2. **Phase 2:** Core Engine Skeleton (in progress)
3. **Phase 3:** Cards and Elixir System
4. **Phase 4:** Collision & Targeting
5. **Phase 5:** Replay & Serialization
6. **Phase 6:** Browser/WASM Viewer
7. **Phase 7:** AI & WebSocket Bridge
8. **Phase 8:** Optimization & Extensibility

## Contributing

When adding new documentation:
- Phase summaries go in `docs/phases/`
- Technical analysis goes in `docs/legacy-analysis/` (if analyzing legacy code) or create new category
- Update this README.md with links to new documents

## External Resources

- [Clash Royale Official Site](https://clashroyale.com)
- [Legacy JS Engine Repository](https://github.com/[legacy-repo]) (if applicable)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Bevy Engine](https://bevyengine.org/) (for WASM viewer in Phase 6)
