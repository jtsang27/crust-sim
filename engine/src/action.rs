//! Player actions that can be applied to the game state.

use crate::card::CardId;
use crate::state::GameState;
use serde::{Deserialize, Serialize};
use shared::{Error, PlayerId, Position, Result};

/// Actions that players can take during the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Play a card at the specified position.
    PlayCard {
        player: PlayerId,
        card_id: CardId,
        position: Position,
    },

    /// Emote (for replay purposes, no game effect).
    Emote { player: PlayerId, emote_id: u32 },
}

impl Action {
    /// Applies this action to the game state.
    pub(crate) fn apply(&self, state: &mut GameState) -> Result<()> {
        match self {
            Action::PlayCard {
                player,
                card_id,
                position,
            } => {
                // Get the card from the player's available cards (clone to avoid borrow issues)
                let card = state
                    .get_card(*card_id)
                    .ok_or_else(|| Error::InvalidAction(format!("Card {:?} not found", card_id)))?
                    .clone();

                // Check if player has enough elixir
                let player_state = state
                    .players
                    .get_mut(player)
                    .ok_or_else(|| Error::InvalidAction("Player not found".to_string()))?;

                if !player_state.spend_elixir(card.elixir_cost as f32) {
                    return Err(Error::InvalidAction(format!(
                        "Not enough elixir. Need {}, have {}",
                        card.elixir_cost, player_state.elixir
                    )));
                }

                // Spawn the card's entities
                card.spawn(state, *player, *position)?;

                Ok(())
            }
            Action::Emote { .. } => {
                // Emotes have no game effect
                Ok(())
            }
        }
    }
}
