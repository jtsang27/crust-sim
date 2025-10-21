//! Game state management and serialization.

use crate::action::Action;
use crate::entities::Entity;
use crate::rng::Rng;
use serde::{Deserialize, Serialize};
use shared::{PlayerId, Result};
use std::collections::HashMap;

/// The complete state of a game simulation.
///
/// This struct contains everything needed to:
/// - Run the simulation forward
/// - Serialize/deserialize for replays
/// - Restore to a previous state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Current simulation tick (increments each step).
    pub tick: u64,

    /// Deterministic RNG for all randomness.
    pub rng: Rng,

    /// All entities currently in the game (troops, towers, projectiles).
    pub entities: HashMap<EntityId, Entity>,

    /// Player-specific state (elixir, deck, etc.).
    pub players: HashMap<PlayerId, PlayerState>,

    /// Next entity ID to assign.
    next_entity_id: u32,

    /// Game match time in seconds.
    pub match_time: f32,

    /// Maximum match duration (in seconds).
    pub max_match_time: f32,
}

impl GameState {
    /// Creates a new game state with the given RNG seed.
    pub fn new(seed: u64) -> Self {
        let mut players = HashMap::new();
        players.insert(PlayerId::Player1, PlayerState::new(PlayerId::Player1));
        players.insert(PlayerId::Player2, PlayerState::new(PlayerId::Player2));

        Self {
            tick: 0,
            rng: Rng::new(seed),
            entities: HashMap::new(),
            players,
            next_entity_id: 1,
            match_time: 0.0,
            max_match_time: 180.0, // 3 minutes (will be configurable)
        }
    }

    /// Applies a player action to the game state.
    pub fn apply_action(&mut self, action: &Action) -> Result<()> {
        action.apply(self)
    }

    /// Allocates a new entity ID.
    pub fn allocate_entity_id(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        EntityId(id)
    }

    /// Adds an entity to the game.
    pub fn add_entity(&mut self, entity: Entity) -> EntityId {
        let id = self.allocate_entity_id();
        self.entities.insert(id, entity);
        id
    }

    /// Removes an entity from the game.
    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(&id)
    }

    /// Checks if the match has ended.
    pub fn is_match_over(&self) -> bool {
        self.match_time >= self.max_match_time
            || self.players.values().any(|p| p.is_defeated())
    }

    /// Advances match time by delta.
    pub fn advance_time(&mut self, delta: f32) {
        self.match_time += delta;
    }
}

/// Unique identifier for an entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(u32);

impl EntityId {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Player-specific state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: PlayerId,
    pub elixir: f32,
    pub max_elixir: f32,
    pub elixir_regen_rate: f32,
    pub tower_hp: HashMap<TowerType, f32>,
}

impl PlayerState {
    pub fn new(id: PlayerId) -> Self {
        let mut tower_hp = HashMap::new();
        tower_hp.insert(TowerType::King, 2400.0);
        tower_hp.insert(TowerType::LeftPrincess, 1400.0);
        tower_hp.insert(TowerType::RightPrincess, 1400.0);

        Self {
            id,
            elixir: 5.0,
            max_elixir: 10.0,
            elixir_regen_rate: 1.0, // 1 elixir per second (will be configurable)
            tower_hp,
        }
    }

    /// Checks if this player has been defeated (King tower destroyed).
    pub fn is_defeated(&self) -> bool {
        self.tower_hp.get(&TowerType::King).copied().unwrap_or(0.0) <= 0.0
    }

    /// Adds elixir, capped at max.
    pub fn add_elixir(&mut self, amount: f32) {
        self.elixir = (self.elixir + amount).min(self.max_elixir);
    }

    /// Attempts to spend elixir. Returns true if successful.
    pub fn spend_elixir(&mut self, cost: f32) -> bool {
        if self.elixir >= cost {
            self.elixir -= cost;
            true
        } else {
            false
        }
    }
}

/// Tower types in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TowerType {
    King,
    LeftPrincess,
    RightPrincess,
}
