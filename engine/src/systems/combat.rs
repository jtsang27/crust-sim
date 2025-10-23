//! Combat system (targeting, attacking, damage).

use crate::entities::TargetType;
use crate::state::{EntityId, GameState};
use shared::PlayerId;

/// Updates combat logic (targeting, attacks).
pub fn update(state: &mut GameState, dt: f32) {
    // Update attack cooldowns
    for entity in state.entities.values_mut() {
        if entity.attack_cooldown > 0.0 {
            entity.attack_cooldown = (entity.attack_cooldown - dt).max(0.0);
        }
    }

    // First pass: Assign targets to all entities that can attack
    let mut target_assignments = Vec::new();

    for (attacker_id, attacker) in &state.entities {
        // Skip if entity can't attack
        if !attacker.can_attack() {
            continue;
        }

        // Find or verify target
        let target_id = if let Some(current_target) = attacker.target {
            // Check if current target is still valid
            if is_valid_target(state, *attacker_id, EntityId::from_u32(current_target)) {
                Some(EntityId::from_u32(current_target))
            } else {
                // Find new target
                find_target(state, *attacker_id, attacker.owner, attacker.target_type())
            }
        } else {
            // Find new target
            find_target(state, *attacker_id, attacker.owner, attacker.target_type())
        };

        if let Some(target_id) = target_id {
            target_assignments.push((*attacker_id, target_id));
        }
    }

    // Apply target assignments
    for (attacker_id, target_id) in &target_assignments {
        if let Some(attacker) = state.entities.get_mut(attacker_id) {
            attacker.target = Some(target_id.as_u32());
        }
    }

    // Second pass: Execute attacks for entities that are ready and in range
    let mut attacks = Vec::new();

    for (attacker_id, target_id) in target_assignments {
        let attacker = &state.entities[&attacker_id];

        // Skip if on cooldown
        if attacker.attack_cooldown > 0.0 {
            continue;
        }

        let target = &state.entities[&target_id];
        let distance = attacker.position.distance_to(&target.position);

        // Check if target is in range
        if distance <= attacker.attack_range() {
            attacks.push((attacker_id, target_id, attacker.damage(), attacker.attack_speed()));
        }
    }

    // Apply attacks
    for (attacker_id, target_id, damage, attack_speed) in attacks {
        let attacker = &state.entities[&attacker_id];

        // Check if this is a ranged attack
        if attacker.is_ranged() {
            // Spawn projectile
            spawn_projectile(state, attacker_id, target_id, damage);
        } else {
            // Melee: Apply damage instantly
            if let Some(target) = state.entities.get_mut(&target_id) {
                target.take_damage(damage);
            }
        }

        // Set cooldown
        if let Some(attacker) = state.entities.get_mut(&attacker_id) {
            attacker.attack_cooldown = attack_speed;
        }
    }
}

/// Spawns a projectile from attacker toward target.
fn spawn_projectile(state: &mut GameState, attacker_id: EntityId, target_id: EntityId, damage: f32) {
    use crate::entities::{Entity, EntityKind, ProjectileData};

    let attacker = &state.entities[&attacker_id];

    let projectile = Entity::new(
        attacker.owner,
        attacker.position, // Start at attacker's position
        EntityKind::Projectile(ProjectileData {
            damage,
            speed: 15.0, // Projectiles move at 15 tiles/second (fast)
            target_id: Some(target_id.as_u32()),
        }),
    );

    state.add_entity(projectile);
}

/// Finds the best target for an attacker.
/// Returns the nearest valid enemy, regardless of range (for movement purposes).
fn find_target(
    state: &GameState,
    attacker_id: EntityId,
    attacker_owner: PlayerId,
    target_type: Option<TargetType>,
) -> Option<EntityId> {
    let attacker = &state.entities[&attacker_id];

    let mut best_target: Option<(EntityId, f32)> = None;

    for (id, entity) in &state.entities {
        // Skip self
        if *id == attacker_id {
            continue;
        }

        // Skip allies
        if entity.owner == attacker_owner {
            continue;
        }

        // Skip dead entities
        if !entity.is_alive() {
            continue;
        }

        // Check target type compatibility
        if let Some(target_type) = target_type {
            if !is_valid_target_type(entity, target_type) {
                continue;
            }
        }

        let distance = attacker.position.distance_to(&entity.position);

        // Prioritize targets by distance (closest first)
        match best_target {
            None => {
                best_target = Some((*id, distance));
            }
            Some((_, best_distance)) => {
                // Prefer closer targets
                if distance < best_distance {
                    best_target = Some((*id, distance));
                }
            }
        }
    }

    // Return the nearest target (even if out of range)
    // Movement system will move toward it, combat system will attack when in range
    best_target.map(|(id, _)| id)
}

/// Checks if a target is still valid (alive and enemy).
fn is_valid_target(state: &GameState, attacker_id: EntityId, target_id: EntityId) -> bool {
    let attacker = match state.entities.get(&attacker_id) {
        Some(a) => a,
        None => return false,
    };

    let target = match state.entities.get(&target_id) {
        Some(t) => t,
        None => return false,
    };

    // Target must be alive and enemy
    target.is_alive() && target.owner != attacker.owner
}

/// Checks if an entity matches the target type.
fn is_valid_target_type(entity: &crate::entities::Entity, target_type: TargetType) -> bool {
    use crate::entities::EntityKind;

    match target_type {
        TargetType::Ground => {
            // TODO: Add air/ground transport tracking to entities
            true // For now, treat all troops as ground
        }
        TargetType::Air => {
            // TODO: Add air/ground transport tracking to entities
            false // For now, no air units
        }
        TargetType::Both => true,
        TargetType::Buildings => {
            // Towers are buildings
            matches!(entity.kind, EntityKind::Tower(_))
        }
    }
}
