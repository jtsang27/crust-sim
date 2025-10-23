//! Player actions that can be applied to the game state.

use crate::state::GameState;
use serde::{Deserialize, Serialize};
use shared::{Error, PlayerId, Position, Result};

/// Actions that players can take during the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Play a card at the specified position with a specific level.
    /// This bypasses the hand/deck system (useful for testing).
    PlayCard {
        player: PlayerId,
        card_name: String,
        level: u32, // Card level (1-15 depending on rarity)
        position: Position,
    },

    /// Play a card from the player's hand.
    /// This uses the deck/hand cycling system.
    PlayCardFromHand {
        player: PlayerId,
        hand_index: usize, // Index in hand (0-3)
        level: u32,        // Card level to play at
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
                card_name,
                level,
                position,
            } => {
                // Get the card by name (clone to avoid borrow issues)
                let card = state
                    .get_card_by_name(card_name)
                    .ok_or_else(|| Error::InvalidAction(format!("Card '{}' not found", card_name)))?
                    .clone();

                // Check if player has enough elixir
                let player_state = state
                    .players
                    .get_mut(player)
                    .ok_or_else(|| Error::InvalidAction("Player not found".to_string()))?;

                if !player_state.spend_elixir(card.elixir_cost) {
                    return Err(Error::InvalidAction(format!(
                        "Not enough elixir. Need {}, have {}",
                        card.elixir_cost, player_state.elixir
                    )));
                }

                // Spawn the card's entities at the specified level
                card.spawn(state, *player, *position, *level)?;

                Ok(())
            }
            Action::PlayCardFromHand {
                player,
                hand_index,
                level,
                position,
            } => {
                // Get the player's state
                let player_state = state
                    .players
                    .get_mut(player)
                    .ok_or_else(|| Error::InvalidAction("Player not found".to_string()))?;

                // Get the card name from the hand and cycle it
                let card_name = player_state
                    .play_card_from_hand(*hand_index)
                    .ok_or_else(|| Error::InvalidAction(format!("Invalid hand index: {}", hand_index)))?;

                // Get the card definition
                let card = state
                    .get_card_by_name(&card_name)
                    .ok_or_else(|| Error::InvalidAction(format!("Card '{}' not found", card_name)))?
                    .clone();

                // Check if player has enough elixir
                let player_state = state
                    .players
                    .get_mut(player)
                    .ok_or_else(|| Error::InvalidAction("Player not found".to_string()))?;

                if !player_state.spend_elixir(card.elixir_cost) {
                    return Err(Error::InvalidAction(format!(
                        "Not enough elixir. Need {}, have {}",
                        card.elixir_cost, player_state.elixir
                    )));
                }

                // Spawn the card's entities at the specified level
                card.spawn(state, *player, *position, *level)?;

                Ok(())
            }
            Action::Emote { .. } => {
                // Emotes have no game effect
                Ok(())
            }
        }
    }
}
