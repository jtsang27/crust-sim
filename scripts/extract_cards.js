/**
 * Card Extraction Script
 * Extracts card definitions from the legacy JavaScript engine into JSON format.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const LEGACY_CODE_PATH = '/Users/will/Documents/Projects/clash-royale-engine/code.txt';
const OUTPUT_PATH = path.join(__dirname, '../config/patches/v2020_06/cards.json');

/**
 * Troop data structure (from legacy engine analysis):
 * [0]: name
 * [1]: max_hp
 * [2]: current_hp
 * [3]: damage
 * [4]: spawn_offset_x
 * [5]: spawn_offset_y
 * [6]: size (collision radius)
 * [7]: mass
 * [8]: movement_speed (degrees/tick)
 * [9]: attack_range
 * [10]: sight_range
 * [11]: reload_time (ticks before first attack)
 * [12]: attack_speed (ticks between attacks)
 * [13]: deploy_time (ticks)
 * [14]: ???
 * [15]: flying (boolean)
 * [16]: ???
 * [17]: splash_radius
 * [18]: what (target type: "ground", "air", "all", "buildings")
 * [19]: target (what this unit targets: same values)
 * [20]: ???
 * [21]: spawn_delay (ticks)
 * [22]: projectile_speed
 * [23]: ???
 * [24]: effects array
 */

function extractCards() {
    console.log('Reading legacy code.txt...');
    const code = fs.readFileSync(LEGACY_CODE_PATH, 'utf-8');

    // Find the cardsInit function and extract the cards array
    const cardsInitMatch = code.match(/cardsInit = function\(\)\{\s*cards = \[([\s\S]*?)\];\s*\}/);

    if (!cardsInitMatch) {
        throw new Error('Could not find cards array in code.txt');
    }

    const cardsArrayString = '[' + cardsInitMatch[1] + ']';
    console.log(`Found cards array (${cardsArrayString.length} characters)`);

    // Safely evaluate the JavaScript array
    // Note: This uses eval which is normally unsafe, but we control the input
    let cardsArray;
    try {
        cardsArray = eval(cardsArrayString);
    } catch (e) {
        console.error('Error parsing cards array:', e.message);
        throw e;
    }

    console.log(`Extracted ${cardsArray.length} cards`);

    // Convert to JSON format
    const cards = cardsArray.map((card, index) => {
        const [name, description, elixirCost, troops, optionalType] = card;

        // Determine card type
        let cardType = 'troop';
        if (optionalType === 'spell') {
            cardType = 'spell';
        } else if (troops.length > 0 && troops[0][18] === 'building') {
            cardType = 'building';
        }

        // Extract stats from first troop (representative)
        const primaryTroop = troops[0] || [];
        const [
            troopName, maxHp, currentHp, damage, spawnX, spawnY,
            size, mass, movementSpeed, attackRange, sightRange,
            reloadTime, attackSpeed, deployTime, _, flying, __,
            splashRadius, troopType, targetType, ___, spawnDelay,
            projectileSpeed, ____, effects
        ] = primaryTroop;

        // Convert movement speed from degrees/tick to category
        const getMovementCategory = (speed) => {
            if (!speed || speed <= 0) return null;
            if (speed <= 45) return 'slow';
            if (speed <= 60) return 'medium';
            if (speed <= 90) return 'fast';
            return 'very_fast';
        };

        // Extract effects
        const extractedEffects = [];
        if (effects && Array.isArray(effects)) {
            for (const effect of effects) {
                if (!Array.isArray(effect)) continue;

                const [effectType, ...effectParams] = effect;
                const effectObj = { type: effectType };

                // Handle different effect types
                switch (effectType) {
                    case 'slow':
                    case 'stun':
                    case 'snare':
                    case 'freeze':
                        effectObj.duration = effectParams[0] / 30; // Convert ticks to seconds
                        break;
                    case 'lifetime':
                        effectObj.duration = effectParams[0] / 60; // Convert ticks to seconds
                        break;
                    case 'spawner':
                        effectObj.interval = effectParams[0] / 60;
                        break;
                }

                if (Object.keys(effectObj).length > 1) {
                    extractedEffects.push(effectObj);
                }
            }
        }

        const cardData = {
            id: index,
            name,
            type: cardType,
            rarity: inferRarity(name, elixirCost), // Infer from cost for now
            elixir_cost: elixirCost,
            description,
            stats: {
                hp: maxHp || undefined,
                damage: damage || undefined,
                dps: attackSpeed ? damage / (attackSpeed / 60) : undefined,
                attack_speed: attackSpeed ? attackSpeed / 60 : undefined, // Convert to seconds
                movement_speed: getMovementCategory(movementSpeed),
                range: attackRange || undefined,
                deploy_time: deployTime ? deployTime / 60 : undefined, // Convert to seconds
                target: targetType || undefined,
                count: troops.length,
                projectile_speed: projectileSpeed || undefined,
                radius: splashRadius || undefined
            },
            effects: extractedEffects.length > 0 ? extractedEffects : undefined,
            legacy_data: {
                card_index: index,
                original_name: troopName,
                troops: troops.map(t => ({
                    name: t[0],
                    spawn_offset: { x: t[4], y: t[5] }
                }))
            }
        };

        // Clean up undefined values
        Object.keys(cardData.stats).forEach(key => {
            if (cardData.stats[key] === undefined || cardData.stats[key] === null) {
                delete cardData.stats[key];
            }
        });

        return cardData;
    });

    // Save to JSON
    const outputDir = path.dirname(OUTPUT_PATH);
    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
    }

    fs.writeFileSync(OUTPUT_PATH, JSON.stringify(cards, null, 2), 'utf-8');
    console.log(`✅ Saved ${cards.length} cards to ${OUTPUT_PATH}`);

    // Print summary
    const byType = cards.reduce((acc, card) => {
        acc[card.type] = (acc[card.type] || 0) + 1;
        return acc;
    }, {});

    console.log('\nCard Summary:');
    Object.entries(byType).forEach(([type, count]) => {
        console.log(`  ${type}: ${count}`);
    });

    return cards;
}

function inferRarity(name, cost) {
    // Simple heuristic based on elixir cost
    // This can be manually corrected later
    if (cost <= 2) return 'common';
    if (cost <= 4) return 'rare';
    if (cost <= 6) return 'epic';
    return 'legendary';
}

// Run extraction
try {
    extractCards();
} catch (error) {
    console.error('❌ Extraction failed:', error.message);
    console.error(error.stack);
    process.exit(1);
}
