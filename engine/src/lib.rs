//! Core game engine for Clash Royale simulation.
//!
//! This crate provides a deterministic, tick-based simulation engine with:
//! - Fixed timestep updates
//! - Seeded random number generation
//! - Serializable game state
//! - Configuration-driven mechanics

pub mod action;
pub mod arena;
pub mod card;
pub mod entities;
pub mod rng;
pub mod state;
pub mod systems;

pub use action::Action;
pub use arena::Arena;
pub use card::{Card, CardId};
pub use rng::Rng;
pub use state::GameState;

use shared::Result;

/// Fixed timestep for simulation (60 FPS = ~16.67ms per tick).
pub const DELTA_TIME: f32 = 1.0 / 60.0;

/// Main entry point for advancing the simulation by one tick.
///
/// This function:
/// 1. Processes player actions
/// 2. Updates all systems (movement, combat, elixir, etc.)
/// 3. Returns the updated game state
///
/// # Determinism
/// Given the same initial state, actions, and RNG seed, this function
/// will always produce identical results.
pub fn step(state: &mut GameState, actions: &[Action]) -> Result<()> {
    // Process actions
    for action in actions {
        state.apply_action(action)?;
    }

    // Update systems
    systems::elixir::update(state, DELTA_TIME);
    systems::movement::update(state, DELTA_TIME);
    systems::combat::update(state, DELTA_TIME);
    systems::lifecycle::update(state, DELTA_TIME);

    // Increment tick counter and match time
    state.tick += 1;
    state.advance_time(DELTA_TIME);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_simulation() {
        let mut state1 = GameState::new(12345);
        let mut state2 = GameState::new(12345);

        // Run 100 ticks with no actions
        for _ in 0..100 {
            step(&mut state1, &[]).unwrap();
            step(&mut state2, &[]).unwrap();
        }

        // States should be identical
        assert_eq!(state1.tick, state2.tick);
        assert_eq!(state1.tick, 100);
    }
}
