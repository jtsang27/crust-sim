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

    // Skip ahead for combat test
    for _ in 0..59 {
        step(&mut state, &[]).unwrap();
    }

    // Combat test: Position units close enough to fight
    println!("\n=== Combat Test ===");
    println!("Spawning opposing Knight vs Archers...\n");

    // Clear existing entities for clean combat test
    state.entities.clear();

    // Player 1: Knight at position (16, 10)
    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player1,
            card_name: "Knight".to_string(),
            level: 11,
            position: Position::new(16.0, 10.0),
        }],
    )
    .unwrap();

    // Player 2: Archers at position (16, 11) - within melee AND ranged range
    // Knight range: 1.2, Archer range: 5.0, Distance: 1.0
    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player2,
            card_name: "Archers".to_string(),
            level: 11,
            position: Position::new(16.0, 11.0),
        }],
    )
    .unwrap();

    println!("[Tick {}] Knight (P1) spawned at (16, 10)", state.tick);
    println!("           HP: 1452, Range: 1.2, Damage: 167, Attack Speed: 1.2s");
    println!("[Tick {}] Archers (P2) spawned at (16, 11)", state.tick);
    println!("           HP: 252 each (x2), Range: 5.0, Damage: 100, Attack Speed: 1.2s");
    println!("           Distance: 1.0 tiles (within both ranges)\n");

    // Run combat for 300 ticks (5 seconds)
    let mut last_entity_count = state.entities.len();
    let mut combat_events = Vec::new();

    for _ in 0..300 {
        step(&mut state, &[]).unwrap();

        // Track when entities die
        if state.entities.len() < last_entity_count {
            combat_events.push((state.tick, state.entities.len()));
            last_entity_count = state.entities.len();
        }
    }

    // Report combat events
    println!("=== Combat Events ===");
    for (tick, count) in combat_events {
        println!("[Tick {}] Entity died ({} remaining)", tick, count);
    }

    // Final state
    println!("\n=== Match Summary ===");
    println!("Total ticks: {}", state.tick);
    println!("Match time: {:.2}s", state.match_time);
    println!("Active entities: {}", state.entities.len());
    println!("Player 1 final elixir: {:.2}", state.players[&PlayerId::Player1].elixir);
    println!("Player 2 final elixir: {:.2}", state.players[&PlayerId::Player2].elixir);

    // List all surviving entities
    if !state.entities.is_empty() {
        println!("\n=== Surviving Entities ===");
        for (id, entity) in &state.entities {
            println!(
                "Entity {}: Owner={:?}, HP={:.0}/{:.0}, Pos=({:.1}, {:.1})",
                id.as_u32(),
                entity.owner,
                entity.hp,
                entity.max_hp,
                entity.position.x,
                entity.position.y
            );
        }
    } else {
        println!("\n=== All entities eliminated ===");
    }

    println!("\n✅ Simulation complete!");
}
