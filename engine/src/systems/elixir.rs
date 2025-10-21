//! Elixir regeneration system.

use crate::state::GameState;

/// Updates elixir for all players.
pub fn update(state: &mut GameState, dt: f32) {
    for player in state.players.values_mut() {
        let regen = player.elixir_regen_rate * dt;
        player.add_elixir(regen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::PlayerId;

    #[test]
    fn test_elixir_regeneration() {
        let mut state = GameState::new(42);
        let initial_elixir = state.players[&PlayerId::Player1].elixir;

        // Run for 1 second (60 ticks at 1/60s each)
        for _ in 0..60 {
            update(&mut state, 1.0 / 60.0);
        }

        let final_elixir = state.players[&PlayerId::Player1].elixir;
        assert!((final_elixir - initial_elixir - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_elixir_cap() {
        let mut state = GameState::new(42);
        state.players.get_mut(&PlayerId::Player1).unwrap().elixir = 9.5;

        // Run for 10 seconds
        for _ in 0..600 {
            update(&mut state, 1.0 / 60.0);
        }

        let final_elixir = state.players[&PlayerId::Player1].elixir;
        assert_eq!(final_elixir, 10.0); // Capped at max
    }
}
