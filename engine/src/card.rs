//! Card definitions and behaviors.

use crate::entities::{Entity, EntityKind, TargetType, TroopData};
use crate::state::GameState;
use serde::{Deserialize, Serialize};
use shared::{PlayerId, Position, Result};

/// A card that can be played by a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub elixir_cost: u32,
    pub card_type: CardType,
    pub stats: CardStats,
}

impl Card {
    /// Spawns entities when this card is played.
    pub fn spawn(&self, state: &mut GameState, owner: PlayerId, position: Position) -> Result<()> {
        match self.card_type {
            CardType::Troop { ref troop } => {
                self.spawn_troop(state, owner, position, troop)?;
            }
            CardType::Spell { ref effect } => {
                self.apply_spell(state, owner, position, effect)?;
            }
            CardType::Building { ref building } => {
                self.spawn_building(state, owner, position, building)?;
            }
        }
        Ok(())
    }

    fn spawn_troop(
        &self,
        state: &mut GameState,
        owner: PlayerId,
        position: Position,
        troop: &TroopStats,
    ) -> Result<()> {
        for _ in 0..troop.count {
            let entity = Entity::new(
                owner,
                position,
                EntityKind::Troop(TroopData {
                    base_hp: troop.hp,
                    damage: troop.damage,
                    range: troop.range,
                    attack_speed: troop.attack_speed,
                    movement_speed: troop.movement_speed,
                    target_type: troop.targets,
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
        _building: &BuildingStats,
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
        _effect: &SpellEffect,
    ) -> Result<()> {
        // TODO: Implement spell effects
        let _ = (state, owner, position);
        Ok(())
    }
}

/// Unique card identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardId {
    Knight,
    Archers,
    Giant,
    Fireball,
    Arrows,
}

/// Type of card and its specific properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardType {
    Troop { troop: TroopStats },
    Spell { effect: SpellEffect },
    Building { building: BuildingStats },
}

/// Stats for troop cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroopStats {
    pub hp: f32,
    pub damage: f32,
    pub attack_speed: f32, // Seconds between attacks
    pub movement_speed: f32,
    pub range: f32,
    pub targets: TargetType,
    pub count: u32, // Number of troops spawned
}

/// Stats for building cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingStats {
    pub hp: f32,
    pub lifetime: f32, // Seconds
    pub damage: Option<f32>,
    pub attack_speed: Option<f32>,
    pub range: Option<f32>,
}

/// Spell effect data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellEffect {
    pub damage: f32,
    pub radius: f32,
    pub duration: Option<f32>,
}

/// General card stats (shared across all types).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardStats {
    pub rarity: Rarity,
    pub deploy_time: f32, // Seconds
}

/// Card rarity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

/// Pre-defined test cards for Phase 2.
pub fn get_test_cards() -> Vec<Card> {
    vec![
        // Knight - 3 elixir melee tank
        Card {
            id: CardId::Knight,
            name: "Knight".to_string(),
            elixir_cost: 3,
            card_type: CardType::Troop {
                troop: TroopStats {
                    hp: 1452.0,
                    damage: 167.0,
                    attack_speed: 1.2,
                    movement_speed: 1.0, // Medium
                    range: 1.2,          // Melee
                    targets: TargetType::Ground,
                    count: 1,
                },
            },
            stats: CardStats {
                rarity: Rarity::Common,
                deploy_time: 1.0,
            },
        },
        // Archers - 3 elixir ranged duo
        Card {
            id: CardId::Archers,
            name: "Archers".to_string(),
            elixir_cost: 3,
            card_type: CardType::Troop {
                troop: TroopStats {
                    hp: 252.0,
                    damage: 100.0,
                    attack_speed: 1.2,
                    movement_speed: 1.0,
                    range: 5.0,
                    targets: TargetType::Both,
                    count: 2,
                },
            },
            stats: CardStats {
                rarity: Rarity::Common,
                deploy_time: 1.0,
            },
        },
        // Giant - 5 elixir tank (targets buildings)
        Card {
            id: CardId::Giant,
            name: "Giant".to_string(),
            elixir_cost: 5,
            card_type: CardType::Troop {
                troop: TroopStats {
                    hp: 3275.0,
                    damage: 211.0,
                    attack_speed: 1.5,
                    movement_speed: 0.75, // Slow
                    range: 1.2,
                    targets: TargetType::Buildings,
                    count: 1,
                },
            },
            stats: CardStats {
                rarity: Rarity::Rare,
                deploy_time: 1.0,
            },
        },
        // Fireball - 4 elixir damage spell
        Card {
            id: CardId::Fireball,
            name: "Fireball".to_string(),
            elixir_cost: 4,
            card_type: CardType::Spell {
                effect: SpellEffect {
                    damage: 572.0,
                    radius: 2.5,
                    duration: None,
                },
            },
            stats: CardStats {
                rarity: Rarity::Rare,
                deploy_time: 0.0, // Instant
            },
        },
        // Arrows - 3 elixir area damage spell
        Card {
            id: CardId::Arrows,
            name: "Arrows".to_string(),
            elixir_cost: 3,
            card_type: CardType::Spell {
                effect: SpellEffect {
                    damage: 144.0,
                    radius: 4.0,
                    duration: None,
                },
            },
            stats: CardStats {
                rarity: Rarity::Common,
                deploy_time: 0.0,
            },
        },
    ]
}
