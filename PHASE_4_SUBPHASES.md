# Phase 4 Sub-Phases: Combat Systems

Phase 4 was originally scoped as "Collision & Targeting" but includes several complex systems. Breaking it into sub-phases for incremental implementation.

---

## ✅ Phase 4.1: Basic Targeting & Combat (COMPLETE)

**Status:** Complete (commit 22eebd4)

**Implemented:**
- Entity attack cooldowns and target tracking
- Targeting logic (find nearest enemy)
- Damage application and attack execution
- Entity death and cleanup
- Target type filtering (ground/air/buildings)

**Testing:**
- Knight vs Archers combat verified
- Entities die and are removed correctly
- Attack cooldowns work properly

---

## Phase 4.2: Basic Movement AI

**Goal:** Make units move toward targets when out of range

**Tasks:**
1. Add velocity calculation toward target
2. Implement "move toward target if out of range" logic
3. Add movement speed from card data to entities
4. Stop moving when in attack range
5. Test Knight walking toward distant Archers

**Implementation Plan:**
- Add `set_velocity_toward()` helper to shared Position
- Update movement system to calculate direction to target
- Add `is_in_attack_range()` check to stop movement
- Units should:
  - Stand still if target in range (attack)
  - Move toward target if out of range
  - Stop when reaching attack range

**Testing Scenario:**
- Spawn Knight at (10, 10)
- Spawn Archers at (20, 10) - 10 tiles away
- Knight should walk toward Archers
- Knight should stop at distance 1.2 (attack range)
- Combat should begin once in range

**Deliverables:**
- Updated movement system with target-seeking
- CLI demo showing unit movement
- Deterministic movement (same seed = same paths)

**Success Criteria:**
- Knight walks toward distant target
- Knight stops when in attack range
- Movement speed matches card data
- No jittering or overshooting

---

## ✅ Phase 4.3: Circular Collision Detection (COMPLETE)

**Status:** Complete (commit 3792d16)

**Goal:** Prevent units from overlapping and walking through each other

**Implemented:**
- Added `radius()` method to Entity (Towers: 1.5, Troops: 0.4, Projectiles: 0.1, Spells: 0.0)
- Implemented circle-circle collision detection in movement system
- Three-pass movement: calculate velocities → check collisions → apply positions
- Simple collision response: don't move if collision would occur
- CLI test with 2 Knights side-by-side targeting same Archer

**Testing Results:**
- Knights started 0.9 tiles apart
- Converged to exactly 0.80 tiles (minimum safe distance)
- Maintained spacing throughout simulation
- No overlapping occurred

**Known Limitations & Deferred Features:**

1. **No pathfinding around obstacles**
   - Units stop if completely blocked by collision
   - Don't find alternate routes around obstacles
   - Works fine for simple scenarios (units side-by-side)
   - Advanced pathfinding deferred to future phase (outside Phase 4 scope)

2. **No pushing/sliding mechanics**
   - Units don't slide along collision surfaces
   - If diagonal movement blocked, unit stops completely
   - Could add tangent-space sliding in future (low priority)

3. **No separation force**
   - Units that somehow spawn overlapping won't push apart
   - Only prevents new overlaps, doesn't fix existing ones
   - Not an issue if spawn positions are validated

4. **Simple "stop moving" collision response**
   - Unit completely halts if collision detected
   - No partial movement or gradual navigation
   - Works well enough for current needs

5. **Performance: O(n²) collision checks**
   - Every moving entity checks against all others
   - Fine for <100 entities (typical match)
   - Will need spatial partitioning (quadtree/grid) for 100+ units
   - Optimization deferred to Phase 4.6 if needed

6. **No collision layers/groups**
   - All entities collide with all others (except radius 0.0)
   - Can't have "pass-through" relationships (e.g., air units over ground)
   - May need in future for air/ground separation

**Success Criteria Met:**
- ✅ Units don't overlap (circle collision works)
- ✅ Collision detection is deterministic
- ⚠️  Units can't navigate around obstacles (no pathfinding - deferred)
- ✅ Performance acceptable for typical match sizes

---

## Phase 4.4: Projectile System

**Goal:** Ranged attacks spawn projectiles that travel to targets

**Tasks:**
1. Add `is_ranged` flag to EntityKind::Troop
2. Create projectile spawning in combat system
3. Implement projectile movement (faster than troops)
4. Add projectile-target collision detection
5. Apply damage on projectile hit
6. Test Archers shooting arrows

**Implementation Plan:**
- Modify combat system:
  - Melee: Apply damage instantly (as now)
  - Ranged: Spawn projectile entity
- Projectile entity:
  - Owns damage value
  - Tracks target entity ID
  - Moves toward target at projectile_speed
  - Dies on impact or if target dies
- Add projectile_speed to card data (Archers: fast, Spear Goblins: slower)

**Testing Scenario:**
- Spawn Archers at (10, 10)
- Spawn Knight at (15, 10) - within archer range
- Archers should shoot arrows (projectile entities)
- Arrows should travel toward Knight
- Knight should take damage when arrow hits

**Deliverables:**
- Projectile entity spawning for ranged attacks
- Projectile movement and collision
- CLI demo showing projectile count and hits

**Success Criteria:**
- Projectiles spawn on ranged attack
- Projectiles travel to target
- Damage applied on impact
- Projectiles removed after hit
- Melee attacks still instant (no projectiles)

---

## Phase 4.5: Advanced Targeting (Tower Priority)

**Goal:** Implement proper targeting priorities (buildings > troops)

**Tasks:**
1. Identify entity types (troop vs building vs tower)
2. Implement priority-based targeting
3. Giants should prefer towers over troops
4. Troops should prefer nearest enemy (any type)
5. Test Giant ignoring troops to attack tower

**Implementation Plan:**
- Add building flag to EntityKind
- Update `find_target()` to consider priorities:
  - If attacker targets buildings: prioritize buildings/towers
  - If target type is "both": prioritize nearest
  - If multiple same-priority: choose nearest
- Tower entities should be marked as buildings

**Testing Scenario:**
- Spawn Giant (targets buildings)
- Spawn enemy Knight between Giant and tower
- Giant should walk past Knight to attack tower
- Knight should attack Giant while Giant ignores it

**Deliverables:**
- Priority-based targeting logic
- Building/tower entity identification
- CLI demo with Giant targeting tower

**Success Criteria:**
- Giants prioritize towers over troops
- Troops attack nearest enemy regardless of type
- Targeting is deterministic
- Units don't retarget unnecessarily

---

## Phase 4.6: Integration Testing

**Goal:** Verify all combat systems work together

**Tasks:**
1. Full battle test: 3v3 units with mixed types
2. Tower destruction test
3. Movement + collision + projectiles + targeting
4. Performance test (100+ entities)
5. Document all known issues

**Testing Scenarios:**
1. **Mixed Combat:**
   - P1: Knight, Archers, Giant
   - P2: Knight, Archers, Giant
   - All spawn near each other
   - Verify correct targeting, movement, projectiles

2. **Tower Siege:**
   - Spawn Giant targeting tower
   - Spawn defending troops
   - Verify Giant reaches and destroys tower

3. **Ranged vs Melee:**
   - Archers vs Knight
   - Knight should walk toward Archers
   - Archers should shoot while retreating (future)
   - For now: just verify projectiles and damage

**Deliverables:**
- Comprehensive combat test suite
- Performance benchmarks
- Known issues documented
- Phase 4 complete summary

**Success Criteria:**
- All sub-phases working together
- No crashes or infinite loops
- Deterministic results
- Ready for Phase 5 (Replay & Serialization)

---

## Timeline Estimate

- **Phase 4.1:** ✅ Complete (2-3 hours)
- **Phase 4.2:** 2-3 hours (movement AI is straightforward)
- **Phase 4.3:** 3-4 hours (collision detection needs careful testing)
- **Phase 4.4:** 3-4 hours (projectile system is moderately complex)
- **Phase 4.5:** 1-2 hours (targeting priority is mostly logic)
- **Phase 4.6:** 2-3 hours (integration testing and documentation)

**Total:** ~11-16 hours for complete Phase 4

---

## Next Step

Currently on: **Phase 4.2 - Basic Movement AI**

Ready to start when you are!
