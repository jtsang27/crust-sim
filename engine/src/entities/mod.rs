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

    /// Time until next attack (in seconds). 0 = ready to attack.
    pub attack_cooldown: f32,

    /// Current target entity ID (if any).
    pub target: Option<u32>,
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
            attack_cooldown: 0.0,
            target: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }

    /// Returns the attack range for this entity.
    pub fn attack_range(&self) -> f32 {
        match &self.kind {
            EntityKind::Tower(data) => data.range,
            EntityKind::Troop(data) => data.range,
            _ => 0.0,
        }
    }

    /// Returns the damage this entity deals.
    pub fn damage(&self) -> f32 {
        match &self.kind {
            EntityKind::Tower(data) => data.damage,
            EntityKind::Troop(data) => data.damage,
            EntityKind::Projectile(data) => data.damage,
            EntityKind::Spell(data) => data.damage,
        }
    }

    /// Returns the attack speed (seconds between attacks).
    pub fn attack_speed(&self) -> f32 {
        match &self.kind {
            EntityKind::Tower(data) => data.attack_speed,
            EntityKind::Troop(data) => data.attack_speed,
            _ => 1.0,
        }
    }

    /// Returns true if this entity can attack (troops and towers).
    pub fn can_attack(&self) -> bool {
        matches!(self.kind, EntityKind::Tower(_) | EntityKind::Troop(_))
    }

    /// Returns the target type for this entity.
    pub fn target_type(&self) -> Option<TargetType> {
        match &self.kind {
            EntityKind::Troop(data) => Some(data.target_type),
            _ => None,
        }
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
