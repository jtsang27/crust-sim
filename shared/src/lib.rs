//! Shared data structures and utilities used across the engine.

use serde::{Deserialize, Serialize};

/// Represents a 2D position in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the direction vector (normalized) from this position to another.
    /// Returns (0, 0) if positions are the same.
    pub fn direction_to(&self, other: &Position) -> (f32, f32) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 0.001 {
            (0.0, 0.0)
        } else {
            (dx / distance, dy / distance)
        }
    }
}

/// Represents a 2D velocity vector.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Player identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerId {
    Player1,
    Player2,
}

impl PlayerId {
    pub fn opponent(&self) -> Self {
        match self {
            PlayerId::Player1 => PlayerId::Player2,
            PlayerId::Player2 => PlayerId::Player1,
        }
    }
}

/// Common result type for engine operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the engine.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid action: {0}")]
    InvalidAction(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Configuration(String),
}
