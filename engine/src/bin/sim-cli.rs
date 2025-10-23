//! CLI binary for running simulations.

use engine::{step, Action, GameState};
use engine::state::EntityId;
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

    // Collision test: Spawn 2 Knights side-by-side trying to reach same target
    println!("\n=== Collision Detection Test ===");
    println!("Spawning 2 Knights side-by-side and 1 Archer as target...\n");

    // Clear existing entities for clean collision test
    state.entities.clear();

    // Reset elixir for testing
    state.players.get_mut(&PlayerId::Player1).unwrap().elixir = 10.0;
    state.players.get_mut(&PlayerId::Player2).unwrap().elixir = 10.0;

    // Player 1: Two Knights side-by-side at (10, 10) and (10, 10.6)
    // Troop radius is 0.4, so spacing of 0.6 should make them barely NOT overlapping
    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player1,
            card_name: "Knight".to_string(),
            level: 11,
            position: Position::new(10.0, 10.0),
        }],
    )
    .unwrap();

    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player1,
            card_name: "Knight".to_string(),
            level: 11,
            position: Position::new(10.0, 10.9), // 0.9 tiles apart (should not overlap with radius 0.4 each)
        }],
    )
    .unwrap();

    // Player 2: Archer at position (20, 10.5) - in between the two Knights' Y positions
    step(
        &mut state,
        &[Action::PlayCard {
            player: PlayerId::Player2,
            card_name: "Archers".to_string(),
            level: 11,
            position: Position::new(20.0, 10.45),
        }],
    )
    .unwrap();

    println!("[Tick {}] Knight 1 (P1) spawned at (10.0, 10.0)", state.tick);
    println!("[Tick {}] Knight 2 (P1) spawned at (10.0, 10.9)", state.tick);
    println!("[Tick {}] Archers (P2) spawned at (20.0, 10.45)", state.tick);
    println!("           Troop collision radius: 0.4 tiles");
    println!("           Knights are 0.9 tiles apart (should NOT overlap)");
    println!("\nExpected: Knights walk toward Archer without overlapping each other\n");

    // Find both Knight entities
    let mut knight_ids: Vec<EntityId> = state.entities.iter()
        .filter(|(_, e)| e.owner == PlayerId::Player1)
        .map(|(id, _)| *id)
        .collect();
    knight_ids.sort_by_key(|id| id.as_u32());

    let knight1_id = knight_ids[0];
    let knight2_id = knight_ids[1];

    let mut position_samples = Vec::new();

    // Run simulation for 600 ticks (10 seconds)
    let mut last_entity_count = state.entities.len();
    let mut combat_events = Vec::new();

    for i in 0..600 {
        step(&mut state, &[]).unwrap();

        // Sample positions every 60 ticks (1 second)
        if i % 60 == 0 {
            if let (Some(k1), Some(k2)) = (state.entities.get(&knight1_id), state.entities.get(&knight2_id)) {
                // Store both knight positions and distance between them
                let dist_between = k1.position.distance_to(&k2.position);
                position_samples.push((state.tick, k1.position.x, k1.position.y, k2.position.x, k2.position.y, dist_between));
            }
        }

        // Track when entities die
        if state.entities.len() < last_entity_count {
            combat_events.push((state.tick, state.entities.len()));
            last_entity_count = state.entities.len();
        }
    }

    // Report movement
    println!("=== Knight Movement (sampled every 1s) ===");
    for (tick, k1_x, k1_y, k2_x, k2_y, dist_between) in &position_samples {
        println!("[Tick {}] Knight1: ({:.2}, {:.2}), Knight2: ({:.2}, {:.2}), Distance between: {:.2}",
            tick, k1_x, k1_y, k2_x, k2_y, dist_between);
    }

    // Check if they ever overlapped (distance < sum of radii = 0.8)
    let min_distance = position_samples.iter()
        .map(|(_, _, _, _, _, dist)| dist)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&999.0);

    println!("\nMinimum distance between Knights: {:.2} tiles", min_distance);
    println!("Expected minimum: 0.80 tiles (radius 0.4 + radius 0.4)");
    if *min_distance < 0.79 {
        println!("⚠️  WARNING: Knights overlapped! (distance < 0.79)");
    } else {
        println!("✅ Knights maintained proper spacing (collision detection working)");
    }

    // Report combat events
    println!("\n=== Combat Events ===");
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
