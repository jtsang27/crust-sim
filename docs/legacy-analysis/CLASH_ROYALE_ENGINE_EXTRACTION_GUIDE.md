# Clash Royale Engine - Extraction & Modernization Guide

## Quick Reference: Key Code Locations

### 1. Main Game Loop Entry Point
**File:** `/Users/will/Documents/Projects/clash-royale-engine/code.txt`  
**Line:** 2658  
**Function:** `void draw()`

This is the core update loop called every frame (~60 FPS). Handles:
- Physics/collision
- Combat system updates
- Effect processing
- Unit spawning
- Rendering

### 2. Arena Definition
**File:** code.txt  
**Line:** 2334  
**Function:** `getTileInfo(i, j)` and `getTileFill(i, j)`

Defines tile types and arena layout:
- 32x18 grid
- 4 arena themes (default, dark, ice, royal)
- Zone types: normal, river, crown, princess, banned

### 3. Card Database (Multiple Versions)
**File:** code.txt  
**Lines:** 253+ (repeated blocks for each version)

Versions available:
- **Lines 253-1000:** "current" version (~99 cards)
- **Lines 1000-1700:** Multiple "legacy-MMYY" versions
- **Sections after 1700:** "retro" (42 cards) and "dev"

### 4. Collision System
**File:** code.txt  
**Lines:** 2859-3016

Three separate collision passes:
1. Lines 2909-2944: Red troops collide with Blue troops
2. Lines 2945-2980: Blue troops collide with Blue troops
3. Lines 2981-3016: Red troops collide with Red troops

### 5. Combat/Attack System
**File:** code.txt  
**Lines:** 3024+ (within draw loop)

Per-unit logic for:
- Target finding
- Range checking
- Projectile creation
- Damage application
- Effect application

### 6. Effect System
**File:** code.txt  
**Lines:** Throughout, managed via effects array `[23]` in unit data

Effects are stored as: `["effect_type", duration, ...params]`

---

## Extraction Checklist

### Phase 1: Data Extraction (Week 1)

- [ ] Parse card definitions from code.txt
  - Extract from each version block
  - Create schema validation
  - Build JSON files
  
- [ ] Extract constants
  - Arena dimensions
  - Physics parameters
  - Timing values
  
- [ ] Create card JSON schema
  - Troop template format
  - Card metadata
  - Version tracking

**Output:** `/data/cards/current.json`, `/data/cards/legacy-*.json`, `/data/constants.json`

### Phase 2: System Extraction (Week 2-3)

- [ ] Isolate physics engine
  - Collision detection
  - Movement system
  - Boundary checking
  
- [ ] Extract combat system
  - Target acquisition
  - Attack handling
  - Damage calculation
  
- [ ] Separate effect system
  - Effect definitions
  - Effect application logic
  - Stacking rules

**Output:** Modularized TypeScript files

### Phase 3: Rendering Separation (Week 3-4)

- [ ] Create render abstraction
  - Arena rendering
  - Unit rendering
  - Projectile rendering
  - UI layer
  
- [ ] Support multiple renderers
  - Canvas (current)
  - WebGL (Pixi.js/Three.js)
  - SVG (future)

**Output:** Renderer interface + implementations

### Phase 4: Configuration System (Week 4-5)

- [ ] Build config loader
  - Card database management
  - Versioning support
  - Validation
  
- [ ] Create balance sheet interface
  - JSON import/export
  - Diff tracking
  - Version comparison

**Output:** Configuration API + admin UI

---

## Code Extraction Templates

### Template 1: Parse Card from code.txt

```javascript
// Current location in code.txt (line 256):
["Spear Goblins", "Three unarmored ranged attackers...", 2, [
  ["spear goblin", 110, 110, 67, 0, 0.5, 1, 1, 120, 5, 5.5, 0, 51, 15, -1, false, 0, 0, "ground", "all", 1, 30, 30, [], []],
  ["spear goblin", 110, 110, 67, 0.7, -0.2, 1, 1, 120, 5, 5.5, 0, 51, 15, -1, false, 0, 0, "ground", "all", 1, 36, 30, [], []],
  ["spear goblin", 110, 110, 67, 0.7, 1.2, 1, 1, 120, 5, 5.5, 0, 51, 15, -1, false, 0, 0, "ground", "all", 1, 33, 30, [], []]
]]

// Target JSON schema:
{
  "id": "spear_goblins",
  "name": "Spear Goblins",
  "description": "Three unarmored ranged attackers...",
  "elixir": 2,
  "troops": [
    {
      "name": "spear goblin",
      "hp": 110,
      "maxHp": 110,
      "damage": 67,
      "spawnOffset": [0, 0.5],
      "size": 1,
      "mass": 1,
      "speed": 120,
      "range": 5,
      "sightRange": 5.5,
      "cooldown": 0,
      "maxCooldown": 51,
      "retargetCooldown": 15,
      "type": "ground",
      "targetType": "all",
      "damagePenalty": 1,
      "loadTime": 30,
      "deployTime": 30,
      "effects": [],
      "special": []
    },
    // ... (2 more with offset positions)
  ]
}
```

### Template 2: Extract Physics Constants

```javascript
// Source: code.txt lines 112-126
const GAME_CONSTANTS = {
  ELIXIR: {
    initial: 5,
    max: 10,
    regenRate: 1,
    regenPerFrame: 1/84  // Converted to frames-per-second
  },
  ARENA: {
    width: 32,
    height: 18,
    centerX: 15.5,
    centerY: 8.5,
    tileSize: 15,  // pixels
    canvasSize: 600
  },
  TIMING: {
    fps: 60,
    cardSelectCooldown: 60,  // frames
    battleCooldown: 150,     // frames
    framesPerSecond: 60
  },
  PHYSICS: {
    speedMultiplier: 1800,   // pixels per frame multiplier
    boundaryPadding: 0       // How close to edge before clamping
  }
};
```

### Template 3: Arena Tile System

```javascript
// Source: code.txt lines 2334-2362
const ARENA_ZONES = {
  "normal": {
    traversable: true,
    canSpawn: true,
    color: "#fafafa"
  },
  "river": {
    traversable: false,  // except via bridges
    canSpawn: false,
    color: "#007fff"
  },
  "bridge": {
    traversable: true,
    canSpawn: false,
    color: "#8b6400"
  },
  "crown": {
    traversable: true,   // spells/air only
    canSpawn: false,
    color: "#ffff00"
  },
  "princess": {
    traversable: true,   // spells/air only
    canSpawn: false,
    color: "#ffff00"
  },
  "banned": {
    traversable: false,
    canSpawn: false,
    color: "#ffc8c8"
  }
};

// Helper to get tile type:
function getTileType(i, j) {
  const trueI = Math.abs(15.5 - i) - 0.5;
  const trueJ = Math.abs(8.5 - j) - 0.5;

  if (trueI === 15 && (j < 6 || j > 11)) return "banned";
  if (trueI > 10 && trueI < 15 && trueJ < 2) return "crown";
  if (trueI > 7 && trueI < 11 && trueJ > 3 && trueJ < 7) return "princess";
  if (trueI < 1) {
    if (trueJ > 4 && trueJ < 6) return "bridge";
    return "river";
  }
  return "normal";
}
```

### Template 4: Combat System Entry Point

```typescript
// Pseudo-code from code.txt lines 3024+
interface CombatContext {
  timestamp: number;
  ownTroops: Troop[];
  enemyTroops: Troop[];
  projectiles: Projectile[];
}

function updateCombat(context: CombatContext) {
  for (const troop of context.ownTroops) {
    if (troop.deployTime > 0) {
      troop.deployTime--;
      continue;
    }

    // Phase 1: Find target
    if (troop.cooldown <= 0) {
      troop.target = findBestTarget(troop, context.enemyTroops);
      if (troop.target !== null) {
        troop.cooldown = troop.maxCooldown;
      }
    }

    // Phase 2: Move toward target
    if (troop.target !== null) {
      moveToward(troop, context.enemyTroops[troop.target]);
    } else {
      moveTowardNearestBuilding(troop);
    }

    // Phase 3: Attack if in range
    if (troop.target !== null && isInRange(troop, context.enemyTroops[troop.target])) {
      performAttack(troop, context.enemyTroops[troop.target], context);
    }

    // Phase 4: Update cooldown
    if (troop.cooldown > 0) troop.cooldown--;
  }
}

function findBestTarget(attacker: Troop, enemies: Troop[]): number | null {
  let bestTarget = null;
  let bestDist = attacker.sightRange + 1;

  for (let i = 0; i < enemies.length; i++) {
    const enemy = enemies[i];
    
    // Type matching (buildings priority)
    if (attacker.targetType === "buildings" && enemy.type !== "building") continue;
    if (attacker.targetType === "ground" && enemy.type === "air") continue;
    
    // Distance check
    const dist = distance(attacker, enemy);
    if (dist > attacker.sightRange) continue;
    
    // Update best target
    if (dist < bestDist) {
      bestDist = dist;
      bestTarget = i;
    }
  }

  return bestTarget;
}
```

### Template 5: Effect System Registration

```typescript
// Effect type definitions needed for extraction
const EFFECT_DEFINITIONS = {
  "slow": {
    category: "debuff",
    properties: {
      duration: "frames",
      intensity: "number (0-1)"
    },
    application: (unit, duration, intensity) => {
      unit.speed *= (1 - intensity);
      unit.attackSpeed *= (1 - intensity);
    }
  },
  "stun": {
    category: "crowd_control",
    properties: {
      duration: "frames"
    },
    application: (unit, duration) => {
      unit.canMove = false;
      unit.canAttack = false;
    }
  },
  "spawner": {
    category: "spawning",
    properties: {
      initialCooldown: "frames",
      spawnInterval: "frames",
      troops: "TroopTemplate[]"
    },
    application: (parent, initialCool, interval, troops) => {
      // Spawn logic
    }
  },
  "split": {
    category: "transformation",
    properties: {
      troops: "TroopTemplate[]"
    },
    application: (parent, troops) => {
      // On death, spawn troops array
    }
  },
  // ... 20+ more effects
};
```

---

## File Locations Summary

| System | File | Lines | Priority |
|--------|------|-------|----------|
| Main Loop | code.txt | 2658-4000 | 1 |
| Card Data | code.txt | 253-1700 | 1 |
| Physics | code.txt | 2859-3016 | 2 |
| Combat | code.txt | 3024-3500 | 2 |
| Arena | code.txt | 2334-2535 | 2 |
| Effects | code.txt | scattered | 3 |
| Rendering | code.txt | 2700-3000, 3700+ | 3 |
| Input | code.txt | 1780-1868, 2648 | 4 |

---

## Quick Integration Path

### Option A: Minimal Changes (Fast)
1. Keep code.txt as-is
2. Extract card JSON separately
3. Build new UI on top
4. Keep game loop working

### Option B: Full Refactor (Better)
1. Extract all systems
2. Rebuild with TypeScript
3. Create modular architecture
4. Add testing framework
5. Replace rendering layer

### Option C: Hybrid (Recommended)
1. Extract card database (immediate value)
2. Create configuration system
3. Build new UI/renderer
4. Keep core logic intact initially
5. Gradually extract systems

---

## Important Notes

1. **Physics is Frame-Rate Dependent**
   - `regenX /= 84` assumes 60 FPS
   - Speed values use `spdm = 1800` multiplier
   - Timing constants are in frames, not milliseconds

2. **Card Data is Duplicated**
   - Each version block contains full card list
   - Need deduplication when extracting
   - Version history can be compressed

3. **Arena Coordinates**
   - Use (trueI, trueJ) not raw (i, j)
   - Grid is 32x18 tiles
   - Origin at center (15.5, 8.5)

4. **Collision Detection**
   - O(n²) complexity, 3 passes
   - Could be optimized with spatial hashing
   - Current implementation prioritizes accuracy

5. **UI Integration Points**
   - Deck selection: index.html → URL parameters
   - Game settings: maxElixir, elixirR, order, version
   - Could be extended to in-game UI

