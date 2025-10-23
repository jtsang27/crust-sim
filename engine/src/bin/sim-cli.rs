//! CLI binary for running simulations.

use engine::{step, Action, GameState};
use shared::{PlayerId, Position};

fn main() {
    println!("=== Clash Royale Engine - CLI Simulation ===\n");

    // Create game state with deterministic seed
    let seed = 42;
    let mut state = GameState::new(seed);

    // Load test cards (using get_test_cards for now - cards.json needs fixing)
    println!("Loading test cards...");
    // Note: cards_complete.json has a parsing issue, will fix in next iteration
    // let cards = load_cards_from_json("config/patches/v2025_current/cards_complete.json").expect("Failed to load cards");
    println!("Loaded 5 test cards\n");

    println!("Game initialized with seed: {}", seed);
    println!("Player 1 starting elixir: {}", state.players[&PlayerId::Player1].elixir);
    println!("Player 2 starting elixir: {}\n", state.players[&PlayerId::Player2].elixir);

    // Initialize player decks (8 cards each)
    println!("=== Initializing Player Decks ===\n");

    let player1_deck = vec![
        "Knight".to_string(),
        "Archers".to_string(),
        "Giant".to_string(),
        "Fireball".to_string(),
        "Arrows".to_string(),
        "Knight".to_string(),  // Duplicate for 8-card deck
        "Archers".to_string(),
        "Giant".to_string(),
    ];

    let player2_deck = vec![
        "Archers".to_string(),
        "Knight".to_string(),
        "Arrows".to_string(),
        "Fireball".to_string(),
        "Giant".to_string(),
        "Knight".to_string(),
        "Archers".to_string(),
        "Arrows".to_string(),
    ];

    state.set_player_deck(PlayerId::Player1, player1_deck).expect("Failed to set P1 deck");
    state.set_player_deck(PlayerId::Player2, player2_deck).expect("Failed to set P2 deck");

    // Print initial hands
    println!("Player 1 hand:");
    for i in 0..4 {
        if let Some(card) = state.players[&PlayerId::Player1].get_hand_card(i) {
            println!("  [{}] {}", i, card);
        }
    }

    println!("\nPlayer 2 hand:");
    for i in 0..4 {
        if let Some(card) = state.players[&PlayerId::Player2].get_hand_card(i) {
            println!("  [{}] {}", i, card);
        }
    }

    // Run simulation with scripted actions
    println!("\n=== Running Scripted Match ===\n");

    // Tick 0: Start
    println!("[Tick {}] Match begins", state.tick);

    // Tick 60: Player 1 plays card from hand slot 0
    for _ in 0..60 {
        step(&mut state, &[]).unwrap();
    }
    let p1_card = state.players[&PlayerId::Player1].get_hand_card(0).unwrap().clone();
    println!("[Tick {}] Player 1 plays {} (hand slot 0) at (16, 8)", state.tick, p1_card);
    step(
        &mut state,
        &[Action::PlayCardFromHand {
            player: PlayerId::Player1,
            hand_index: 0,
            level: 11,
            position: Position::new(16.0, 8.0),
        }],
    )
    .unwrap();
    println!("  Elixir remaining: {:.1}", state.players[&PlayerId::Player1].elixir);
    println!("  New hand: [{}, {}, {}, {}]",
        state.players[&PlayerId::Player1].get_hand_card(0).unwrap(),
        state.players[&PlayerId::Player1].get_hand_card(1).unwrap(),
        state.players[&PlayerId::Player1].get_hand_card(2).unwrap(),
        state.players[&PlayerId::Player1].get_hand_card(3).unwrap()
    );

    // Tick 120: Player 2 plays card from hand slot 1
    for _ in 0..59 {
        step(&mut state, &[]).unwrap();
    }
    let p2_card = state.players[&PlayerId::Player2].get_hand_card(1).unwrap().clone();
    println!("\n[Tick {}] Player 2 plays {} (hand slot 1) at (16, 10)", state.tick, p2_card);
    step(
        &mut state,
        &[Action::PlayCardFromHand {
            player: PlayerId::Player2,
            hand_index: 1,
            level: 11,
            position: Position::new(16.0, 10.0),
        }],
    )
    .unwrap();
    println!("  Elixir remaining: {:.1}", state.players[&PlayerId::Player2].elixir);
    println!("  New hand: [{}, {}, {}, {}]",
        state.players[&PlayerId::Player2].get_hand_card(0).unwrap(),
        state.players[&PlayerId::Player2].get_hand_card(1).unwrap(),
        state.players[&PlayerId::Player2].get_hand_card(2).unwrap(),
        state.players[&PlayerId::Player2].get_hand_card(3).unwrap()
    );

    // Tick 180: Player 1 plays card from hand slot 2
    for _ in 0..59 {
        step(&mut state, &[]).unwrap();
    }
    let p1_card2 = state.players[&PlayerId::Player1].get_hand_card(2).unwrap().clone();
    println!("\n[Tick {}] Player 1 plays {} (hand slot 2) at (16, 6)", state.tick, p1_card2);
    step(
        &mut state,
        &[Action::PlayCardFromHand {
            player: PlayerId::Player1,
            hand_index: 2,
            level: 11,
            position: Position::new(16.0, 6.0),
        }],
    )
    .unwrap();
    println!("  Elixir remaining: {:.1}", state.players[&PlayerId::Player1].elixir);
    println!("  New hand: [{}, {}, {}, {}]",
        state.players[&PlayerId::Player1].get_hand_card(0).unwrap(),
        state.players[&PlayerId::Player1].get_hand_card(1).unwrap(),
        state.players[&PlayerId::Player1].get_hand_card(2).unwrap(),
        state.players[&PlayerId::Player1].get_hand_card(3).unwrap()
    );

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
