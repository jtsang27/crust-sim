//! Player actions that can be applied to the game state.

use crate::state::GameState;
use serde::{Deserialize, Serialize};
use shared::{PlayerId, Position, Result};

/// Actions that players can take during the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Play a card at the specified position.
    PlayCard {
        player: PlayerId,
        card_id: u32,
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
                // TODO: Implement card playing logic
                // For now, just a placeholder
                let _ = (player, card_id, position);
                Ok(())
            }
            Action::Emote { .. } => {
                // Emotes have no game effect
                Ok(())
            }
        }
    }
}
