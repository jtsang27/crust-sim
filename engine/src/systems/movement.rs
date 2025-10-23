//! Movement system for entities.

use crate::state::{EntityId, GameState};
use shared::Velocity;

/// Updates entity movement - sets velocity toward targets and applies movement.
pub fn update(state: &mut GameState, dt: f32) {
    // First pass: Update velocities based on targets
    let mut velocity_updates: Vec<(EntityId, Velocity)> = Vec::new();

    for (id, entity) in &state.entities {
        // Only move troops (not towers)
        if !entity.can_move() {
            continue;
        }

        // Check if entity has a target
        if let Some(target_id) = entity.target {
            let target_entity_id = EntityId::from_u32(target_id);

            // Get target position (if target still exists)
            if let Some(target) = state.entities.get(&target_entity_id) {
                let distance = entity.position.distance_to(&target.position);
                let attack_range = entity.attack_range();

                // If target is out of range, move toward it
                if distance > attack_range {
                    let (dir_x, dir_y) = entity.position.direction_to(&target.position);
                    let move_speed = entity.movement_speed();

                    velocity_updates.push((
                        *id,
                        Velocity::new(dir_x * move_speed, dir_y * move_speed),
                    ));
                } else {
                    // Target in range - stop moving
                    velocity_updates.push((*id, Velocity::zero()));
                }
            } else {
                // Target doesn't exist anymore - stop
                velocity_updates.push((*id, Velocity::zero()));
            }
        } else {
            // No target - stop moving
            velocity_updates.push((*id, Velocity::zero()));
        }
    }

    // Apply velocity updates
    for (id, velocity) in velocity_updates {
        if let Some(entity) = state.entities.get_mut(&id) {
            entity.velocity = velocity;
        }
    }

    // Second pass: Apply velocities to positions
    for entity in state.entities.values_mut() {
        entity.position.x += entity.velocity.x * dt;
        entity.position.y += entity.velocity.y * dt;

        // TODO: Add collision detection
    }
}
