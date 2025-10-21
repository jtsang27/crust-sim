//! Movement system for entities.

use crate::state::GameState;

/// Updates positions based on velocities.
pub fn update(state: &mut GameState, dt: f32) {
    for entity in state.entities.values_mut() {
        entity.position.x += entity.velocity.x * dt;
        entity.position.y += entity.velocity.y * dt;

        // TODO: Add collision detection and pathfinding
    }
}
