//! Card definitions and behaviors.

use crate::entities::{Entity, EntityKind, TargetType, TroopData};
use crate::state::GameState;
use serde::{Deserialize, Serialize};
use shared::{PlayerId, Position, Result};

/// A card that can be played by a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub elixir_cost: f32,
    pub rarity: Rarity,
    #[serde(rename = "card_type")]
    pub type_name: String, // "troop", "spell", "building", "tower troop"

    // Card-level properties (constant across levels)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_hit_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_speed: Option<String>, // "slow", "medium", "fast", "very_fast"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_speed_value: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy_time: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>, // ["air", "ground", "buildings"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>, // "ground", "air"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<String>>, // ["freeze", "knockback", "spawn", etc.]

    // Level-based stats
    pub levels: Vec<CardLevelStats>,
}

/// Stats that vary by card level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardLevelStats {
    pub level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dps: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_damage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_damage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shield_hp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healing: Option<f32>,
}

impl Card {
    /// Spawns entities when this card is played at a specific level.
    pub fn spawn(&self, state: &mut GameState, owner: PlayerId, position: Position, level: u32) -> Result<()> {
        // Get stats for the requested level
        let level_stats = self.get_level_stats(level)?;

        match self.type_name.as_str() {
            "troop" | "tower troop" => {
                self.spawn_troop(state, owner, position, level_stats)?;
            }
            "spell" => {
                self.apply_spell(state, owner, position, level_stats)?;
            }
            "building" => {
                self.spawn_building(state, owner, position, level_stats)?;
            }
            _ => {
                return Err(shared::Error::InvalidAction(format!(
                    "Unknown card type: {}",
                    self.type_name
                )));
            }
        }
        Ok(())
    }

    /// Get stats for a specific card level.
    pub fn get_level_stats(&self, level: u32) -> Result<&CardLevelStats> {
        self.levels
            .iter()
            .find(|stats| stats.level == level)
            .ok_or_else(|| {
                shared::Error::InvalidAction(format!("Level {} not found for {}", level, self.name))
            })
    }

    /// Get the target type from the targets list.
    fn get_target_type(&self) -> TargetType {
        match &self.targets {
            Some(targets) => {
                let has_air = targets.iter().any(|t| t == "air");
                let has_ground = targets.iter().any(|t| t == "ground");
                let has_buildings = targets.iter().any(|t| t == "buildings");

                if has_buildings {
                    TargetType::Buildings
                } else if has_air && has_ground {
                    TargetType::Both
                } else if has_air {
                    TargetType::Air
                } else {
                    TargetType::Ground
                }
            }
            None => TargetType::Both, // Default
        }
    }

    fn spawn_troop(
        &self,
        state: &mut GameState,
        owner: PlayerId,
        position: Position,
        level_stats: &CardLevelStats,
    ) -> Result<()> {
        let count = self.count.unwrap_or(1);
        let hp = level_stats.hp.unwrap_or(100.0);
        let damage = level_stats.damage.unwrap_or(10.0);

        for _ in 0..count {
            let entity = Entity::new(
                owner,
                position,
                EntityKind::Troop(TroopData {
                    base_hp: hp,
                    damage,
                    range: self.range.unwrap_or(1.0),
                    attack_speed: self.attack_speed.unwrap_or(1.0),
                    movement_speed: self.movement_speed_value.unwrap_or(60.0),
                    target_type: self.get_target_type(),
                }),
            );
            state.add_entity(entity);
        }
        Ok(())
    }

    fn spawn_building(
        &self,
        state: &mut GameState,
        owner: PlayerId,
        position: Position,
        _level_stats: &CardLevelStats,
    ) -> Result<()> {
        // TODO: Implement building spawning
        let _ = (state, owner, position);
        Ok(())
    }

    fn apply_spell(
        &self,
        state: &mut GameState,
        owner: PlayerId,
        position: Position,
        level_stats: &CardLevelStats,
    ) -> Result<()> {
        // TODO: Implement spell effects using level_stats.area_damage or .damage
        let _ = (state, owner, position, level_stats);
        Ok(())
    }
}

/// Card rarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

/// Load cards from JSON file.
pub fn load_cards_from_json(path: &str) -> Result<Vec<Card>> {
    let data = std::fs::read_to_string(path)
        .map_err(|e| shared::Error::InvalidAction(format!("Failed to read cards file: {}", e)))?;

    let cards: Vec<Card> = serde_json::from_str(&data)
        .map_err(|e| shared::Error::InvalidAction(format!("Failed to parse cards JSON: {}", e)))?;

    Ok(cards)
}

