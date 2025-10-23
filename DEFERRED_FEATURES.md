# Deferred Features & Future Implementation

This document tracks card features and mechanics that were intentionally excluded from the initial engine implementation. These will be added in later phases once the core engine is stable.

---

## Phase 9 (Future): Champion Abilities

**Status:** Deferred - Requires custom ability code per champion

**Champions Excluded (8 cards):**
- Archer Queen
- Golden Knight
- Skeleton King
- Mighty Miner
- Monk
- Phoenix
- Little Prince
- (Any other champions)

**Why Deferred:**
- Each champion has a unique ability with different mechanics
- Abilities have cooldowns, activation costs (1 elixir), and special effects
- Requires custom implementation per champion rather than config-driven approach
- Not available in low arenas where AI training will start

**Implementation Requirements:**
1. Add `ability_cost` field to Card struct
2. Create `ChampionAbility` trait with `activate()` method
3. Implement each champion's ability:
   - Archer Queen: Invisibility + ranged attack boost
   - Golden Knight: Dash attack
   - Skeleton King: Skeleton spawn on death
   - Mighty Miner: Dash/burrow mechanic
   - Monk: Deflection ability
   - Phoenix: Rebirth mechanic
   - Little Prince: Guardian angel mechanic
4. Add ability activation to Action enum
5. Test ability interactions and balance

**Data Available:**
- Champion stats scraped but filtered out
- Ability costs available in scraped data
- Will need to manually implement ability logic

---

## Phase 10 (Future): Evolution Mechanics

**Status:** Deferred - Special transformation mechanic

**Evolution Cards Excluded:**
- All `/Evolution` variant cards (Archers Evolution, Knight Evolution, etc.)

**Why Deferred:**
- Evolution cards transform after cycling through deck
- Different stats and abilities when evolved
- Requires deck cycling tracking
- Not available in early arenas

**Implementation Requirements:**
1. Track deck cycling per player
2. Add evolution state to card instances
3. Implement stat transformation on evolution
4. Add visual indicators for evolution state
5. Handle evolved card special abilities
6. Test evolution timing and balance

**Data Available:**
- Evolution card stats scraped but filtered out
- Need to implement transformation logic

---

## Phase 11 (Future): Special Tower Troops

**Status:** Deferred - Unique unlock mechanics

**Special Tower Troops Excluded (3 cards):**
- Cannoneer
- Dagger Duchess
- Royal Chef

**Note:** Tower Princess (base tower troop) is INCLUDED as it's available to all players.

**Why Deferred:**
- Special unlock requirements
- Replace default towers with unique abilities
- Different attack patterns and stats
- Not available in low arena gameplay

**Implementation Requirements:**
1. Add tower replacement mechanic
2. Implement unique attack patterns per tower troop
3. Handle tower troop special abilities
4. Test tower damage scaling with king level
5. Balance tower troop interactions

**Data Available:**
- Special tower troop stats scraped but filtered out
- Tower Princess included in base dataset

---


## Phase 12 (Future): Advanced Card Effects

**Status:** Partially implemented - Some effects need custom code

**Effects Identified but May Need Custom Implementation:**
- **Implemented (via config):**
  - freeze
  - knockback
  - spawn (troops on death/deploy)
  - shield
  - area (splash damage)
  - rage
  - heal

- **May Need Custom Code:**
  - slow (requires speed modification over time)
  - stun (requires disabling attacks)
  - clone (requires entity duplication)
  - invisibility (requires visibility state)
  - dash/charge (requires special movement)
  - death_damage (requires on-death trigger)

**Implementation Strategy:**
1. Start with simple config-driven effects (freeze duration, knockback distance)
2. Add effect system with timers and state tracking
3. Implement complex effects as custom behavior systems
4. Test effect stacking and interactions

---

## Phase 13 (Future): Card Level Scaling System

**Status:** Data ready - Implementation pending

**Current Status:**
- All card levels scraped (15 for Common, 13 for Rare, 10 for Epic, 7 for Legendary)
- Level-based stats available (HP, damage, DPS for each level)
- Card struct needs updating to support levels

**Implementation Requirements:**
1. Update Card struct to store all levels
2. Modify spawning to select level based on player's card levels
3. Add king tower level system
4. Implement level-dependent interactions
5. Test level scaling and balance

**Data Structure:**
```rust
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub elixir_cost: u32,
    pub rarity: Rarity,
    pub card_type: CardType,
    pub levels: Vec<CardLevelStats>, // NEW: All level data
    // ... other fields
}

pub struct CardLevelStats {
    pub level: u32,
    pub hp: Option<f32>,
    pub damage: Option<f32>,
    pub dps: Option<f32>,
    pub area_damage: Option<f32>,
    // ... other level-dependent stats
}
```

---

## Implementation Priority

**High Priority (Core Gameplay):**
1. Card level scaling system (Phase 14) - data ready, just needs Rust implementation
2. Advanced effects that are common (slow, stun, etc.)

**Medium Priority (Arena Progression):**
3. Evolution mechanics - needed for mid-high arena gameplay
4. Special tower troops - adds variety to gameplay

**Low Priority (Advanced Features):**
5. Champion abilities - high arena only, complex implementation

---

## Notes for Future Implementation

- Keep `DEFERRED_FEATURES.md` updated as features are implemented
- Move implemented features to `CHANGELOG.md`
- Each deferred feature should have its own branch when development starts
- Test deferred features against existing replays to ensure compatibility
- Document any breaking changes to game state or replay format

---

**Last Updated:** 2025-10-22 (After Phase 1 data extraction completion)
