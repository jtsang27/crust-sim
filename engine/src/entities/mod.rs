//! Entity definitions (troops, towers, projectiles, spells).

use serde::{Deserialize, Serialize};
use shared::{PlayerId, Position, Velocity};

/// An entity in the game world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub owner: PlayerId,
    pub position: Position,
    pub velocity: Velocity,
    pub hp: f32,
    pub max_hp: f32,
    pub kind: EntityKind,
}

impl Entity {
    pub fn new(owner: PlayerId, position: Position, kind: EntityKind) -> Self {
        let max_hp = kind.base_hp();
        Self {
            owner,
            position,
            velocity: Velocity::zero(),
            hp: max_hp,
            max_hp,
            kind,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }
}

/// Different types of entities in the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityKind {
    /// Player towers (King, Princess).
    Tower(TowerData),

    /// Ground or air troops.
    Troop(TroopData),

    /// Projectiles (arrows, fireballs, etc.).
    Projectile(ProjectileData),

    /// Spell effects (area damage, etc.).
    Spell(SpellData),
}

impl EntityKind {
    fn base_hp(&self) -> f32 {
        match self {
            EntityKind::Tower(data) => data.base_hp,
            EntityKind::Troop(data) => data.base_hp,
            EntityKind::Projectile(_) => 1.0,
            EntityKind::Spell(_) => 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerData {
    pub base_hp: f32,
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroopData {
    pub base_hp: f32,
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub movement_speed: f32,
    pub target_type: TargetType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileData {
    pub damage: f32,
    pub speed: f32,
    pub target_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellData {
    pub damage: f32,
    pub radius: f32,
    pub duration: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TargetType {
    Ground,
    Air,
    Both,
    Buildings,
}
