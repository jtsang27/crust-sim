//! Entity lifecycle management (spawning, death, cleanup).

use crate::state::GameState;

/// Removes dead entities and manages spawning.
pub fn update(state: &mut GameState, _dt: f32) {
    // Remove dead entities
    state.entities.retain(|_, entity| entity.is_alive());

    // TODO: Handle spawn timers and death effects
}
