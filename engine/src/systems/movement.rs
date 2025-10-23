//! Movement system for entities.

use crate::state::{EntityId, GameState};
use shared::{Position, Velocity};

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

    // Second pass: Apply velocities to positions with collision detection
    let mut position_updates: Vec<(EntityId, Position)> = Vec::new();

    for (id, entity) in &state.entities {
        // Skip if not moving
        if entity.velocity.x == 0.0 && entity.velocity.y == 0.0 {
            continue;
        }

        // Calculate new position
        let new_x = entity.position.x + entity.velocity.x * dt;
        let new_y = entity.position.y + entity.velocity.y * dt;
        let new_position = Position::new(new_x, new_y);

        // Check for collisions with other entities
        let would_collide = check_collision(state, *id, &new_position);

        if !would_collide {
            position_updates.push((*id, new_position));
        }
        // If collision detected, don't move (stay in current position)
    }

    // Apply position updates
    for (id, position) in position_updates {
        if let Some(entity) = state.entities.get_mut(&id) {
            entity.position = position;
        }
    }
}

/// Checks if moving an entity to a new position would cause a collision.
fn check_collision(state: &GameState, moving_entity_id: EntityId, new_position: &Position) -> bool {
    let moving_entity = &state.entities[&moving_entity_id];
    let moving_radius = moving_entity.radius();

    // Check against all other entities
    for (other_id, other_entity) in &state.entities {
        // Skip self
        if *other_id == moving_entity_id {
            continue;
        }

        // Skip entities with no collision radius
        let other_radius = other_entity.radius();
        if other_radius == 0.0 {
            continue;
        }

        // Calculate distance between centers
        let distance = new_position.distance_to(&other_entity.position);
        let min_distance = moving_radius + other_radius;

        // Collision if circles overlap
        if distance < min_distance {
            return true;
        }
    }

    false
}
