//! CLI binary for running simulations.

use engine::{step, Action, CardId, GameState};
use shared::{PlayerId, Position};

fn main() {
    println!("=== Clash Royale Engine - CLI Simulation ===\n");

    // Create game state with deterministic seed
    let seed = 42;
    let mut state = GameState::new(seed);

    println!("Game initialized with seed: {}", seed);
    println!("Player 1 starting elixir: {}", state.players[&PlayerId::Player1].elixir);
    println!("Player 2 starting elixir: {}\n", state.players[&PlayerId::Player2].elixir);

    // Run simulation with scripted actions
    println!("=== Running Scripted Match ===\n");

    // Tick 0: Start
    println!("[Tick {}] Match begins", state.tick);

    // Tick 60: Player 1 plays Knight (3 elixir) at center
    for _ in 0..60 {
        step(&mut state, &[]).unwrap();
    }
    println!("[Tick {}] Player 1 plays Knight at (16, 8)", state.tick);
    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player1,
            card_id: CardId::Knight,
            position: Position::new(16.0, 8.0),
        }],
    )
    .unwrap();
    println!("  Elixir remaining: {}", state.players[&PlayerId::Player1].elixir);
    println!("  Entities spawned: {}", state.entities.len());

    // Tick 120: Player 2 plays Archers (3 elixir)
    for _ in 0..59 {
        step(&mut state, &[]).unwrap();
    }
    println!("\n[Tick {}] Player 2 plays Archers at (16, 10)", state.tick);
    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player2,
            card_id: CardId::Archers,
            position: Position::new(16.0, 10.0),
        }],
    )
    .unwrap();
    println!("  Elixir remaining: {}", state.players[&PlayerId::Player2].elixir);
    println!("  Entities spawned: {}", state.entities.len());

    // Tick 180: Player 1 plays Giant (5 elixir)
    for _ in 0..59 {
        step(&mut state, &[]).unwrap();
    }
    println!("\n[Tick {}] Player 1 plays Giant at (16, 6)", state.tick);
    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player1,
            card_id: CardId::Giant,
            position: Position::new(16.0, 6.0),
        }],
    )
    .unwrap();
    println!("  Elixir remaining: {}", state.players[&PlayerId::Player1].elixir);
    println!("  Entities spawned: {}", state.entities.len());

    // Run for another 180 ticks (3 seconds)
    for _ in 0..180 {
        step(&mut state, &[]).unwrap();
    }

    // Final state
    println!("\n=== Match Summary ===");
    println!("Total ticks: {}", state.tick);
    println!("Match time: {:.2}s", state.match_time);
    println!("Total entities spawned: {}", state.entities.len());
    println!("Player 1 final elixir: {:.2}", state.players[&PlayerId::Player1].elixir);
    println!("Player 2 final elixir: {:.2}", state.players[&PlayerId::Player2].elixir);

    // List all entities
    println!("\n=== Active Entities ===");
    for (id, entity) in &state.entities {
        println!(
            "Entity {:?}: Owner={:?}, HP={:.0}/{:.0}, Pos=({:.1}, {:.1})",
            id.as_u32(),
            entity.owner,
            entity.hp,
            entity.max_hp,
            entity.position.x,
            entity.position.y
        );
    }

    println!("\n✅ Simulation complete!");
}
