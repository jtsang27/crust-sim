# Crust-Sim: Clash Royale Rust Simulator

A deterministic, high-performance Clash Royale simulator written in Rust, designed for AI training and research.

## Overview

This project rebuilds a legacy JavaScript Clash Royale engine (circa 2020) into a modular, configuration-driven Rust simulation engine optimized for:

- **AI Training**: Reinforcement learning experiments with 1000+ simulations/second target
- **Deterministic Replay**: Reproducible matches with seeded RNG
- **Configuration-Driven**: All game mechanics externalized to JSON configs (no hardcoded values)
- **Multi-Interface**: CLI, WASM browser viewer, and WebSocket API for external agents

## Quick Start

```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Extract legacy game data (Python 3 required)
python3 scripts/extract_cards.py
python3 scripts/extract_arena.py
python3 scripts/extract_mechanics.py
```

## Project Status

**Current Phase:** Phase 1 Complete ✅
**Next Milestone:** Phase 2 - Core Engine Enhancement

- ✅ Pre-Phase 1: Rust workspace setup, RNG, basic GameState
- ✅ Phase 1: Data extraction (97 cards, arena, mechanics)
- 🔄 Phase 2: Core engine skeleton with 5+ test cards
- ⏳ Phase 3-8: See [ROADMAP.md](ROADMAP.md)

## Documentation

All documentation is organized in the [`docs/`](docs/) directory:

- **[docs/README.md](docs/README.md)** - Documentation index and navigation
- **[docs/phases/](docs/phases/)** - Phase completion summaries
- **[docs/legacy-analysis/](docs/legacy-analysis/)** - Legacy engine analysis

### Key Documents

- **[ROADMAP.md](ROADMAP.md)** - 8-phase development plan
- **[CLAUDE.md](CLAUDE.md)** - Guide for Claude Code instances working on this project
- **[Phase 1 Summary](docs/phases/PHASE_1_SUMMARY.md)** - Latest milestone completion report

## Architecture

```
crust-sim/
├── engine/          # Core Rust simulation engine (no I/O)
├── shared/          # Shared data structures
├── viewer/          # WASM browser UI (Phase 6)
├── bridge/          # WebSocket API (Phase 7)
├── config/          # Configuration files
│   ├── patches/     # Versioned game data (v2020_06, etc.)
│   └── schemas/     # JSON validation schemas
├── scripts/         # Data extraction tools
└── docs/            # All documentation
```

## Key Features

### Deterministic Simulation
- Fixed timestep (60 FPS)
- Seeded RNG (`oorandom` crate)
- Reproducible replays from action logs

### Configuration System
- Patch-based versioning (`config/patches/v2020_06/`, etc.)
- JSON schemas for validation
- No hardcoded game values

### AI-Ready
- OpenAI Gym-compatible environment (planned Phase 7)
- State serialization via `serde`
- WebSocket control for external agents

## Development

### Requirements
- Rust 1.70+ (`rustup install stable`)
- Python 3.8+ (for extraction scripts)
- Node.js 18+ (optional, for WASM viewer)

### Building
```bash
cargo build          # Debug build
cargo build --release # Optimized build
cargo test           # Run all tests
```

### Extracting Legacy Data
The `scripts/` directory contains tools to extract game data from the legacy JavaScript engine:

```bash
python3 scripts/extract_cards.py      # Extract 97 cards
python3 scripts/extract_arena.py      # Extract 32×18 arena layout
python3 scripts/extract_mechanics.py  # Extract game constants
```

Output: `config/patches/v2020_06/*.json`

## Contributing

See [ROADMAP.md](ROADMAP.md) for the full development plan. When adding documentation, update [docs/README.md](docs/README.md).

### Git Workflow
- Commit frequently after milestones
- Use descriptive commit messages
- Reference phase numbers in commits

## License

MIT License (to be formalized)

## Acknowledgments

- **Original Concept**: Supercell (Clash Royale)
- **Legacy JavaScript Engine**: Scholarly Gaming (circa 2020)
- **Rust Migration**: This project

---

**For detailed technical documentation, see [`docs/README.md`](docs/README.md)**
