# Clash Royale Engine - Codebase Analysis

## Project Overview

**Location:** `/Users/will/Documents/Projects/clash-royale-engine/`

This is a **JavaScript/Processing.js-based** simulation engine that recreates Clash Royale's 1v1 battle system. The project is a web-based tool built with **CodeSandbox/Parcel** that allows users to simulate Clash Royale battles with custom cards, decks, and game mechanics.

### Key Technologies
- **Processing.js** (not Node.js) - all game code written in Processing/Java-like syntax
- **HTML5 Canvas** for rendering (600x600)
- **URL-based configuration** for game parameters
- **Multiple versioned card databases** (legacy dates, retro, dev, current)
- **No external dependencies** - pure vanilla JavaScript/Processing

---

## Project Structure

```
clash-royale-engine/
├── index.html                 # Entry point - deck/settings selection UI
├── engine.html               # Game simulation canvas page
├── code.txt                  # MAIN GAME ENGINE (Processing.js code, ~1.1MB)
├── processing.txt            # Alias for code.txt
├── creator.html              # Custom card creation tool
├── translator.html           # Replay translation tool
├── blog.html                 # Update blog
├── package.json              # Parcel bundler config (minimal deps)
├── src/
│   ├── index.js             # Minimal entry
│   └── styles.css
└── [historical exports: codeOld.txt, codeRunning.txt]
```

**Entry Flow:**
1. User loads `index.html` - selects decks, configures settings
2. Form submission navigates to `engine.html` with URL parameters
3. `engine.html` loads Processing code from `code.txt`
4. Game runs in 600x600 canvas with real-time updates

---

## Game Architecture

### 1. HIGH-LEVEL GAME LOOP (Main Update System)

**Location:** Line 2658 - `void draw()`

The draw function is called every frame (~60 FPS in Processing) and handles:

```
draw() {
  ├─ Physics/Collision System
  │  ├─ Bound checking (keep units in arena)
  │  ├─ Mass-based collision resolution
  │  └─ Knockback handling
  ├─ Combat System
  │  ├─ Target acquisition
  │  ├─ Attack/reload logic
  │  ├─ Damage application
  │  └─ Effect application (slow, stun, etc.)
  ├─ Unit Movement System
  │  ├─ Pathfinding (basic tile-based)
  │  ├─ Speed modifiers (rage, slow)
  │  └─ River/bridge crossing logic
  ├─ Troop Management
  │  ├─ Spawner updates
  │  ├─ Unit lifetime tracking
  │  ├─ Death/split handling
  │  └─ Projectile updates
  ├─ Rendering
  │  ├─ Arena tiles (4 variants)
  │  ├─ Troops with effects
  │  ├─ Projectiles
  │  └─ UI overlay
  └─ Game State
     ├─ Elixir generation
     ├─ Card selection
     └─ Victory/defeat detection
}
```

### 2. DATA STRUCTURES

#### Troop/Card Data Format
```javascript
[
  "name",                    // 0: String identifier
  health,                    // 1: Current HP
  max_health,               // 2: Max HP
  attack,                   // 3: Damage per hit
  x, y,                     // 4-5: Position (0-32 x 0-18 grid)
  size_diameter,            // 6: Collision radius
  mass,                     // 7: For physics (0 = immovable)
  speed,                    // 8: Movement speed
  range,                    // 9: Attack range
  sight_range,             // 10: Detection range
  cooldown,                // 11: Current reload timer
  max_cooldown,            // 12: Max reload time
  ret,                     // 13: Retarget cooldown
  target,                  // 14: Current target index (-1 = none)
  lock,                    // 15: Locked target boolean
  shield,                  // 16: Active shield HP
  aoe,                     // 17: Area of effect radius
  type,                    // 18: "ground"|"air"|"building"|"bomb"|"spell"
  target_type,             // 19: "all"|"ground"|"buildings"|"air"
  penalty,                 // 20: Damage penalty
  load_time,               // 21: Initial load before first attack
  deploy_time,             // 22: Deployment animation timer
  [effects],               // 23: Active status effects array
  [special]                // 24+: Special abilities/modifiers
]
```

#### Card Definition Format
```javascript
[
  "Card Name",             // 0: Display name
  "Description text",      // 1: Flavor text
  elixir_cost,            // 2: Elixir to deploy
  [                       // 3: Array of troop spawn templates
    [troop_data_array],
    [troop_data_array],
    ...
  ],
  "spell"                 // 4 (optional): Card type flag
]
```

#### Projectile/Attack Data
```javascript
bProj[]  // Blue team active targeting projectiles
rProj[]  // Red team active targeting projectiles
bNProj[] // Blue team non-targeting projectiles (spells, bombs)
rNProj[] // Red team non-targeting projectiles
```

### 3. CORE GAME SYSTEMS

#### A. ELIXIR SYSTEM
- **Initial:** 5 elixir (configurable via URL: `maxElixir`, `elixirR`)
- **Max:** 10 elixir (configurable)
- **Regeneration:** `regenX / 84` per frame (converts to seconds)
- **URL Parameters:**
  - `?maxElixir=10` (1-100)
  - `?elixirR=1` (0-10, multiplier)
  - `?order=true` (shuffle deck)

#### B. ARENA GEOMETRY (32x18 tiles)
**Location:** `getTileInfo()` function (Line 2334)

```
Arena Layout (absolute coordinates):
   0    8.5 (center)    16    32
   |     |              |      |
0  +-----+-----------+-+------+
   |     | Blue      | |Red   |
9  +-----| Deploy    +-+ Side |
   |     |           | |      |
   | River & Bridges | |      |
17 +-----+-----------+-+------+

Key Zones:
- Royal Side (bottom, x=0-10.75): Blue spawns here
- River (x=15-16, 10.75-21.25): Center line, bridges at y=4-6
- Opponent Side (x=21.25-32): Red spawns here

Tile Types:
├─ "normal"    - Traversable by all units
├─ "river"     - Water (only bridges allow crossing at y=4-6)
├─ "banned"    - Out of bounds (edges of map)
├─ "princess"  - Princess tower zone (protected)
└─ "crown"     - Crown tower zone (protected)

Distance Calculation:
trueI = abs(15.5 - i) - 0.5
trueJ = abs(8.5 - j) - 0.5
```

**Tower Positions (Fixed):**
- Crown Towers: `trueI > 10 && trueI < 15 && trueJ < 2`
- Princess Towers: `trueI > 7 && trueI < 11 && trueJ > 3 && trueJ < 7`

#### C. COLLISION SYSTEM
**Location:** Lines 2859-3016

Type-based collision matching:
```javascript
Valid collision pairs:
- bomb + ground
- building + ground
- uground + ground
- hovering + air
- Same type units (ground with ground, etc.)

Physics Resolution:
1. Calculate overlap distance
2. Normalize by combined mass
3. Apply proportional separation
4. Respects mass=0 (immovable objects)
```

Collision flags bypass:
- Spells (immune)
- Knockback effects (temporary)
- "noCol" effect

#### D. COMBAT/ATTACK SYSTEM

**Target Acquisition Logic:**
```javascript
Priority Order:
1. Type matching (buildings first if specified)
2. Range check (must be <= sight_range)
3. Proximity (closest valid target)
4. Air vs ground considerations
```

**Attack Flow per frame:**
1. Find target if cooldown = 0
2. Move toward target or building nearest lane
3. Check if in range (range <= distance)
4. If in range:
   - Trigger attack
   - Create projectile or instant damage
   - Reset cooldown timer
   - Apply effects (slow, stun, etc.)

**Damage Calculation:**
```
Base Damage: troop.attack value
Multipliers:
- penalty value (usually 0.3-1.0 for tower reduction)
- active "rage" effect (2x speed/damage)
- special effects (charge doubles, inferno ramp, etc.)

Status Effects Applied:
- slow: reduces speed/attack by X frames
- stun: freezes unit for X frames
- snare: root in place
- lifeline: expires unit after X frames
```

#### E. EFFECT SYSTEM

**Active Effects Array Structure:**
```javascript
[
  ["effect_type", duration_frames, ...params],
  ...
]

Key Effects:
├─ slow[frames]           - Reduce speed
├─ stun[frames]           - Freeze unit
├─ snare[frames]          - Root in place
├─ lifetime[frames]       - Self-destruct timer
├─ resistance             - Reduced knockback
├─ spell                  - Instant area effect
├─ kamikaze               - Explode on contact
├─ spawner[interval,max,troops[]]  - Periodic spawning
├─ split[troop_array]     - Transform on death
├─ inferno[params]        - Ramping damage beam
├─ charge[params]         - Dash attack
├─ rage                   - Speed/damage multiplier
├─ rage[frames]           - Timed rage effect
├─ weak-stun              - Reduced stun duration
├─ weak-sleep             - Reduced sleep
├─ ps[speed]              - Projectile speed override
├─ jumping                - River crossing ability
├─ deathElixir            - Give elixir on death
├─ invis[initial,detection,range]  - Invisibility
├─ regen[rate]            - Health restoration
└─ [many more...]
```

#### F. SPAWNER SYSTEM (Buildings)
```javascript
// Each spawner effect:
["spawner", initial_cooldown, spawn_interval, [troop_templates]]

// Spawns troops periodically at parent location
// Used by: Goblin Hut, Tombstone, Witch, Barbarian Hut, Furnace, etc.
```

#### G. UNIT LIFETIME & SPLITTING
```javascript
// On death, if special contains split:
["split", [[troop1_template], [troop2_template], ...]]

// Unit transforms/spawns at location
// Used by: Balloon→Bomb, Lava Hound→Lava Pups, Elixir Golem→Golemites, etc.
```

---

## HARDCODED VALUES & CONSTANTS

### Global Game Settings
```javascript
Line 112-126:
regenX = 1                  // Elixir regen speed (1 = standard)
startE = 5                  // Starting elixir
maxE = 10                   // Max elixir
reorder = false             // Shuffle deck
viewAng = false             // Debug: show directions
viewReload = false          // Debug: show cooldowns
projLines = false           // Debug: show projectile paths
extGraphics = true          // Enhanced rendering
spdm = 1800                 // Speed multiplier (converts to pixels/frame)

Timing:
bCardMCool = 60             // Card select cooldown frames
btMaxCool = 150             // Battle cooldown frames
regenX /= 84                // Frames per second conversion
```

### Arena Physics Constants
```javascript
Line 2334-2362:
trueI = abs(15.5 - i) - 0.5    // Horizontal midpoint
trueJ = abs(8.5 - j) - 0.5     // Vertical midpoint
River at: trueI < 1
Bridges at: trueJ > 4 && trueJ < 6
Crown zone: trueI > 10 && trueI < 15
Princess zone: trueI > 7 && trueI < 11
Banned edges: trueI === 15 && (j < 6 || j > 11)
```

### Arena Scaling
```javascript
Canvas: 600x600 pixels
Tiles: 32x18 grid
Tile Size: 15 pixels rendered
Offset: 60,60 pixels (canvas origin)
```

### Card Database Versions
```javascript
Line 253+:
version = "current"         // Live stats
version = "legacy-MMYY"     // Historical (Oct 2019+)
version = "retro"           // ~March 2016 original release
version = "dev"             // Experimental/new mechanics

Supported Legacy Versions:
- legacy-1019, legacy-1119, legacy-1219 (2019)
- legacy-0120 through legacy-0920 (2020)
- More added over time
```

---

## CARD DEFINITIONS

**Location:** Lines 255-1770 (Multiple version blocks)

### Card Data Organization
- **Current version:** Line 253-1000 (~99 cards)
- **Legacy versions:** Repeated blocks with version-specific stats
- **Retro version:** Original 42-card set
- **Dev version:** Experimental cards

### Example Card Definition (Spear Goblins)
```javascript
["Spear Goblins", 
 "Three unarmored ranged attackers...",  // Description
 2,                                       // Elixir cost
 [                                        // Spawn array (3 goblins)
   ["spear goblin", 110, 110, 67,        // name, hp, maxhp, dmg
    0, 0.5, 1, 1, 120, 5, 5.5,           // x, y, size, mass, speed, range, sight
    0, 51, 15, -1, false, 0, 0,          // cool, maxcool, ret, target, lock, shield, aoe
    "ground", "all", 1, 30, 30, [], []],  // type, targettype, penalty, loadtime, deploytime, effects, special
   ["spear goblin", 110, 110, 67,        // Same, offset positions
    0.7, -0.2, 1, 1, 120, 5, 5.5, 0, 51, 15, -1, false, 0, 0,
    "ground", "all", 1, 36, 30, [], []],
   ["spear goblin", 110, 110, 67,
    0.7, 1.2, 1, 1, 120, 5, 5.5, 0, 51, 15, -1, false, 0, 0,
    "ground", "all", 1, 33, 30, [], []]
 ]
]
```

### Key Card Types

**Troops (Ground/Air):**
- Melee (Knight, Goblin, P.E.K.K.A.)
- Ranged (Spear Goblins, Musketeer, Archer)
- Air (Minions, Baby Dragon, Lava Hound)
- Spawners (Witch, Night Witch)

**Buildings (Stationary):**
- Defensive (Cannon, Inferno Tower, Tesla)
- Spawner (Goblin Hut, Furnace, Tombstone)

**Spells (Instant Effect):**
- Damage (Fireball, Rocket, Lightning)
- Control (Freeze, Tornado)
- Buff (Rage, Heal)
- Deployable (Goblin Barrel, Graveyard)

### Stat Ranges (Current Version)
```
HP: 67 (Ice Spirit) to 4256 (Golem)
Damage: 0 (buildings) to 1232 (Rocket)
Speed: 0 (buildings) to 120 (fast units)
Range: 0 (melee) to 11.5 (X-Bow, Mortar)
Elixir: 1 to 10
```

---

## TICKET/UPDATE LOOP IMPLEMENTATION

### Frame-by-Frame Update Sequence

**Per Frame (draw() function):**

```
1. INITIALIZATION
   ├─ cardsInit()          // Load cards once
   ├─ frame++              // Increment frame counter
   ├─ mX, mY = mousePos    // Get mouse position
   └─ scaling adjustments

2. INPUT HANDLING
   ├─ Mouse drag tracking
   ├─ Keyboard (card selection)
   └─ UI interaction

3. GAME STATE UPDATES
   ├─ Elixir regeneration
   │  └─ elixir += regenX * 84 / 60  (per frame)
   ├─ Deck/card cooldowns
   └─ Battle timer

4. PHYSICS PHASE
   ├─ Boundary checking (keep units in bounds)
   ├─ River/bridge enforcement
   ├─ Crown tower protection
   └─ Collision system (3 passes for Red→Blue, Blue→Blue, Red→Red)

5. COMBAT PHASE (per team)
   ├─ For each Red troop:
   │  ├─ Check deploy time
   │  ├─ Find target
   │  ├─ Move toward target
   │  ├─ Check attack range
   │  ├─ Fire projectile or instant effect
   │  ├─ Apply effects to self/nearby
   │  └─ Handle death/split
   ├─ For each Blue troop: (same)
   └─ Process projectiles (movement, collision, expiration)

6. EFFECT PROCESSING
   ├─ Decrement effect timers
   ├─ Apply slows (speed *= slow_multiplier)
   ├─ Apply stuns (freeze movement)
   ├─ Apply status effects
   └─ Remove expired effects

7. SPAWNER UPDATES
   ├─ Decrement spawn cooldown
   ├─ When ready, spawn new units
   └─ Move units slightly from parent

8. RENDERING PHASE
   ├─ Clear background
   ├─ Draw arena tiles (colored by zone)
   ├─ Draw debug info (if enabled)
   ├─ For each troop:
   │  ├─ Draw health indicator
   │  ├─ Apply effect coloring (slow=blue, stun=yellow, etc.)
   │  ├─ Draw unit circle
   │  ├─ Draw attack direction (if viewAng)
   │  ├─ Draw reload arc (if viewReload)
   │  └─ Draw special graphics
   ├─ Draw projectiles
   ├─ Draw UI (elixir, cards, messages)
   └─ Draw game-over state

9. VICTORY/DEFEAT CHECK
   ├─ If either tower destroyed → end game
   ├─ Display winner
   └─ Freeze simulation
```

### Key Timing Constants (Frames)

```javascript
1 Second = ~60 frames (Processing.js default)
Common Durations:
- Stun: 15-30 frames (0.25-0.5 sec)
- Slow: 60-120 frames (1-2 sec)
- Lifetime: 600-2100 frames (10-35 sec)
- Elixir regen: 2.5 sec (150 frames) per elixir
- Deploy animation: 30 frames typical
- Load time: 30-51 frames (0.5-0.85 sec) average
```

---

## CONFIGURATION POINTS (URL PARAMETERS)

### Game Initialization (from index.html form)
```
engine.html?
  blueDeck=URL_encoded_deck_link   // Clash Royale deck link
  &redDeck=URL_encoded_deck_link
  &version=legacy-0920              // Card version
  &order=true|false                // Randomize deck
  &maxElixir=10                     // (1-100)
  &elixirR=1                        // Regen rate (0-10)
```

### Card Creator Parameters
```
?v0=name&v1=desc&v2=elixir&...vN=param
  (26+ parameters per card)
  - v0-v3: Name, desc, elixir, (unused)
  - v4-v6: HP, attack, size
  - v7-v10: Mass, speed, range, sight
  - v11-v13: Load, deploy, cooldown
  - v14-v16: (unused)
  - v17-v20: Special effects (slow, stun, snare, lifetime)
  - v21-v27: Boolean flags & multipliers
```

### Replay Mode
```
?result=encoded_actions
  (Encodes previous battles for replay)
  Timestamp-based action sequence
```

---

## WHAT NEEDS EXTRACTION INTO CONFIG FILES

### Priority 1 (Immediate Impact)
1. **Card Database** → JSON files by version
   - Extract 99+ card definitions
   - Create schema/validator
   
2. **Game Constants** → Config file
   - Elixir values, arena dimensions
   - Physics parameters (mass, collision)
   
3. **Balance Values** → Spreadsheet-friendly format
   - HP, damage, speed, cooldown
   - Per-version snapshots

### Priority 2 (Structural Improvement)
4. **Arena Geometry** → JSON map data
   - Tile types, tower positions
   - Spawn zones, protected areas

5. **Effect System** → Declarative config
   - Effect types & parameters
   - Duration formulas

6. **AI/Behavior** → Strategy configs
   - Target priority rules
   - Placement algorithms

### Priority 3 (Enhancement)
7. **UI Strings** → Translation file
8. **Sound/Music** → Asset registry
9. **Visual Themes** → Arena/card styling

---

## MAJOR CODE SECTIONS

| Section | Lines | Purpose |
|---------|-------|---------|
| Setup/Init | 1-250 | URL parsing, initial vars |
| Card Defs (current) | 253-1000 | 99 card stats |
| Card Defs (legacy) | 1000-1700 | Historical versions |
| Utility Functions | 2009-2177 | Distance, collision, shuffle |
| Arena Rendering | 2334-2535 | Tile drawing, colors |
| **Main Game Loop** | **2658-3700+** | **Core simulation** |
| Rendering Loop | 3000-4500 | Draw troops, effects |
| Input Handling | 1780-1868 | Keyboard, mouse |
| UI/HUD | 4500-end | Messages, stats |

---

## KEY FINDINGS

### Strengths
- Single monolithic file enables easy sharing/modification
- Declarative card format (arrays) is easy to parse
- Physics engine handles complex interactions
- Multiple arena themes & card versions supported
- Comprehensive effect system

### Weaknesses Requiring Refactoring
- All game logic in one 1.1MB file (hard to maintain)
- Hardcoded values scattered throughout
- No separation of concerns (simulation vs. rendering)
- Manual unit update loops (no component system)
- Card data duplicated across versions
- Complex nested arrays (error-prone)
- No validation/schema for card definitions
- Collision system has redundant O(n²) passes

### Modernization Opportunities
1. Extract to structured classes/modules
2. Configuration-driven card loading
3. Event-based combat system
4. Render abstraction layer
5. Type definitions/validation
6. Test suite for card stats
7. Asset management system

---

## REPRODUCTION STEPS FOR NEW ENGINE

1. Extract card JSON from each version block
2. Create card loader/validator
3. Port game loop to modern framework (Three.js/Pixi.js)
4. Implement configuration system
5. Migrate physics engine
6. Build new UI/visualization layer
7. Add unit tests for mechanics
8. Create admin panel for balance changes

