//! Deterministic random number generator for reproducible simulations.

use serde::{Deserialize, Serialize};

/// Wrapper around oorandom for deterministic RNG.
///
/// This ensures all randomness in the simulation is reproducible
/// given the same seed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rng {
    #[serde(skip)]
    rng: Option<oorandom::Rand32>,
    seed: u64,
    state: u64,
}

impl Rng {
    /// Creates a new RNG with the given seed.
    pub fn new(seed: u64) -> Self {
        let rng = oorandom::Rand32::new(seed);
        Self {
            rng: Some(rng),
            seed,
            state: 0,
        }
    }

    /// Generates a random u32.
    pub fn rand_u32(&mut self) -> u32 {
        let value = self.rng.as_mut().unwrap().rand_u32();
        self.state = self.state.wrapping_add(value as u64);
        value
    }

    /// Generates a random f32 in the range [0.0, 1.0).
    pub fn rand_float(&mut self) -> f32 {
        self.rand_u32() as f32 / u32::MAX as f32
    }

    /// Generates a random f32 in the range [min, max).
    pub fn rand_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.rand_float() * (max - min)
    }

    /// Generates a random integer in the range [min, max).
    pub fn rand_int_range(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        let range = (max - min) as u32;
        min + (self.rand_u32() % range) as i32
    }

    /// Returns the seed used to initialize this RNG.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns the current internal state (for debugging/validation).
    pub fn state(&self) -> u64 {
        self.state
    }

    /// Restores the RNG after deserialization.
    pub(crate) fn restore(&mut self) {
        if self.rng.is_none() {
            self.rng = Some(oorandom::Rand32::new(self.seed));
            // Advance RNG to match the saved state
            // This is a simplified approach; in production you'd want to
            // serialize the full RNG state or track call count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let mut rng1 = Rng::new(42);
        let mut rng2 = Rng::new(42);

        for _ in 0..100 {
            assert_eq!(rng1.rand_u32(), rng2.rand_u32());
        }
    }

    #[test]
    fn test_float_range() {
        let mut rng = Rng::new(123);

        for _ in 0..1000 {
            let value = rng.rand_range(5.0, 10.0);
            assert!(value >= 5.0 && value < 10.0);
        }
    }

    #[test]
    fn test_int_range() {
        let mut rng = Rng::new(456);

        for _ in 0..1000 {
            let value = rng.rand_int_range(1, 7); // Dice roll
            assert!(value >= 1 && value < 7);
        }
    }
}
