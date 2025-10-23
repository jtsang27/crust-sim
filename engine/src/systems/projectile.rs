//! Projectile system (movement and collision).

use crate::entities::{CollisionShape, EntityKind};
use crate::state::{EntityId, GameState};
use shared::Position;

/// Updates projectile movement and handles collisions with targets.
pub fn update(state: &mut GameState, dt: f32) {
    // Collect projectile updates
    let mut position_updates = Vec::new();
    let mut hits = Vec::new();  // (projectile_id, target_id, damage)
    let mut remove_projectiles = Vec::new();

    for (proj_id, projectile) in &state.entities {
        // Only process projectiles
        let proj_data = match &projectile.kind {
            EntityKind::Projectile(data) => data,
            _ => continue,
        };

        // Check if target still exists
        let target_id = match proj_data.target_id {
            Some(id) => EntityId::from_u32(id),
            None => {
                // No target - remove projectile
                remove_projectiles.push(*proj_id);
                continue;
            }
        };

        let target = match state.entities.get(&target_id) {
            Some(t) if t.is_alive() => t,
            _ => {
                // Target dead or missing - remove projectile
                remove_projectiles.push(*proj_id);
                continue;
            }
        };

        // Move toward target
        let (dir_x, dir_y) = projectile.position.direction_to(&target.position);
        let new_x = projectile.position.x + dir_x * proj_data.speed * dt;
        let new_y = projectile.position.y + dir_y * proj_data.speed * dt;
        let new_position = Position::new(new_x, new_y);

        // Check if projectile hit target (supports both circle and rectangle collision)
        let hit = match target.collision_shape() {
            CollisionShape::Circle { radius } => {
                // Circle-to-circle collision
                let distance = new_position.distance_to(&target.position);
                distance <= (projectile.radius() + radius)
            }
            CollisionShape::Rectangle { half_width, half_height } => {
                // Circle-to-rectangle collision (for towers)
                new_position.circle_collides_rect(projectile.radius(), &target.position, half_width, half_height)
            }
            CollisionShape::None => false,
        };

        if hit {
            // Hit! Apply damage and remove projectile
            hits.push((*proj_id, target_id, proj_data.damage));
            remove_projectiles.push(*proj_id);
        } else {
            // No hit yet - update position
            position_updates.push((*proj_id, new_position));
        }
    }

    // Apply position updates
    for (id, position) in position_updates {
        if let Some(entity) = state.entities.get_mut(&id) {
            entity.position = position;
        }
    }

    // Apply hits
    for (_, target_id, damage) in hits {
        if let Some(target) = state.entities.get_mut(&target_id) {
            target.take_damage(damage);
        }
    }

    // Remove projectiles that hit or lost their target
    for id in remove_projectiles {
        state.remove_entity(id);
    }
}
