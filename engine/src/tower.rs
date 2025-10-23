//! Tower configurations and stats.

use serde::{Deserialize, Serialize};
use shared::Result;

/// A tower troop configuration (e.g., Tower Princess, Cannoneer, etc.).
/// These don't have elixir costs and replace the default tower.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tower {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub rarity: Rarity,

    // Tower properties (constant across levels)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_hit_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>, // ["air", "ground"]

    // Level-based stats
    pub levels: Vec<TowerLevelStats>,
}

/// Stats that vary by tower level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerLevelStats {
    pub level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dps: Option<f32>,
}

/// Tower rarity (reusing from cards).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

/// Load towers from JSON file.
pub fn load_towers_from_json(path: &str) -> Result<Vec<Tower>> {
    let data = std::fs::read_to_string(path)
        .map_err(|e| shared::Error::InvalidAction(format!("Failed to read towers file: {}", e)))?;

    let towers: Vec<Tower> = serde_json::from_str(&data)
        .map_err(|e| shared::Error::InvalidAction(format!("Failed to parse towers JSON: {}", e)))?;

    Ok(towers)
}
