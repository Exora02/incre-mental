use crate::components::*;
use crate::math::BigNumber;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct GeneratorTuning {
    name: &'static str,
    base_cost: f64,
    cost_coefficient: f64,
}

const GENERATOR_TUNING: [GeneratorTuning; 10] = [
    GeneratorTuning {
        name: "Fusion Cell",
        base_cost: 10.0,
        cost_coefficient: 1.12,
    },
    GeneratorTuning {
        name: "Quantum Capacitor",
        base_cost: 300.0,
        cost_coefficient: 1.16,
    },
    GeneratorTuning {
        name: "Antimatter Siphon",
        base_cost: 20_000.0,
        cost_coefficient: 1.18,
    },
    GeneratorTuning {
        name: "Void Harvester",
        base_cost: 2_000_000.0,
        cost_coefficient: 1.19,
    },
    GeneratorTuning {
        name: "Chrono Turbine",
        base_cost: 3.0e9,
        cost_coefficient: 1.20,
    },
    GeneratorTuning {
        name: "Stellar Forge",
        base_cost: 1.0e13,
        cost_coefficient: 1.21,
    },
    GeneratorTuning {
        name: "Galactic Bore",
        base_cost: 5.0e16,
        cost_coefficient: 1.22,
    },
    GeneratorTuning {
        name: "Dimension Weaver",
        base_cost: 2.0e21,
        cost_coefficient: 1.23,
    },
    GeneratorTuning {
        name: "Reality Anchor",
        base_cost: 1.0e26,
        cost_coefficient: 1.24,
    },
    GeneratorTuning {
        name: "Entropy Reverser",
        base_cost: 1.0e32,
        cost_coefficient: 1.25,
    },
];

fn generator_id(tier_index: usize) -> String {
    format!("gen_energy_{}", tier_index + 1)
}

fn output_resource_for_tier(tier_index: usize) -> String {
    if tier_index == 0 {
        "Energy".to_string()
    } else {
        generator_id(tier_index - 1)
    }
}

pub fn generator_cost(generator: &Generator) -> BigNumber {
    let cost_mult = generator
        .cost_coefficient
        .powf(generator.purchased_count.to_f64());
    generator.base_cost * cost_mult
}

pub fn can_afford(resource_amount: BigNumber, cost: BigNumber) -> bool {
    resource_amount.gte(&cost)
}

// Startup system to initialize resources
pub fn setup_resources(mut commands: Commands) {
    // Energy Resource
    commands.spawn(GameResource {
        name: "Energy".to_string(),
        amount: BigNumber::zero(),
        unlocked: true,
    });

    // Matter Resource
    commands.spawn(GameResource {
        name: "Matter".to_string(),
        amount: BigNumber::zero(),
        unlocked: false, // Starts locked
    });

    // Generators
    for (i, tuning) in GENERATOR_TUNING.iter().enumerate() {
        let initial_count = if i == 0 { 1.0 } else { 0.0 };

        commands.spawn(Generator {
            id: generator_id(i),
            name: tuning.name.to_string(),
            output_resource: output_resource_for_tier(i),
            base_production: BigNumber::one(),
            count: BigNumber::new(initial_count, 0),
            purchased_count: BigNumber::new(initial_count, 0),
            cost_resource: "Energy".to_string(),
            base_cost: BigNumber::new(tuning.base_cost, 0),
            cost_coefficient: tuning.cost_coefficient,
        });
    }
}

// Production system
pub fn production_tick(
    time: Res<Time>,
    mut generator_queries: ParamSet<(Query<&Generator>, Query<&mut Generator>)>,
    mut query_resources: Query<&mut GameResource>,
) {
    let delta = time.delta_secs_f64();
    let mut production_buffer: HashMap<String, BigNumber> = HashMap::new();

    // 1. Calculate Production
    for generator in generator_queries.p0().iter() {
        if generator.count.mantissa > 0.0 {
            let production = generator.base_production * generator.count.to_f64() * delta;

            if let Some(current) = production_buffer.get_mut(&generator.output_resource) {
                *current = *current + production;
            } else {
                production_buffer.insert(generator.output_resource.clone(), production);
            }
        }
    }

    // 2. Apply to Resources
    for mut resource in query_resources.iter_mut() {
        if let Some(amount) = production_buffer.get(&resource.name) {
            resource.amount = resource.amount + *amount;
        }
    }

    // 3. Apply to Generators
    // Only affects TOTAL count, not purchased count
    for mut generator in generator_queries.p1().iter_mut() {
        if let Some(amount) = production_buffer.get(&generator.id) {
            generator.count = generator.count + *amount;
        }
    }
}

pub fn handle_buy_interactions(
    _commands: Commands,
    interactions: Query<(&Interaction, &BuyGeneratorButton), Changed<Interaction>>,
    mut generators: Query<&mut Generator>,
    mut resource_queries: ParamSet<(Query<&GameResource>, Query<&mut GameResource>)>,
) {
    for (interaction, buy_button) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut generator) = generators.get_mut(buy_button.0) {
                let cost = generator_cost(&generator);

                // Check affordability
                let mut can_buy = false;
                for resource in resource_queries.p0().iter() {
                    if resource.name == generator.cost_resource {
                        can_buy = can_afford(resource.amount, cost);
                        break;
                    }
                }

                if can_buy {
                    // Pay the cost
                    for mut resource in resource_queries.p1().iter_mut() {
                        if resource.name == generator.cost_resource {
                            resource.amount = resource.amount - cost;
                            break;
                        }
                    }

                    // Increase generator count AND purchased count
                    generator.count = generator.count + BigNumber::one();
                    generator.purchased_count = generator.purchased_count + BigNumber::one();
                    println!("Bought generator: {}", generator.name);
                } else {
                    println!("Cannot afford generator: {}", generator.name);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simulate_unlock_times(max_seconds: u64) -> [Option<u64>; 10] {
        let mut counts = [0.0_f64; 10];
        let mut purchased = [0.0_f64; 10];
        let mut unlock_times: [Option<u64>; 10] = [None; 10];
        let mut energy = 0.0_f64;

        counts[0] = 1.0;
        purchased[0] = 1.0;
        unlock_times[0] = Some(0);

        for second in 1..=max_seconds {
            let previous_counts = counts;

            energy += previous_counts[0];
            for tier in 1..10 {
                counts[tier - 1] += previous_counts[tier];
            }

            let mut bought_this_second = true;
            while bought_this_second {
                bought_this_second = false;
                for tier in (0..10).rev() {
                    let tuning = GENERATOR_TUNING[tier];
                    let cost = tuning.base_cost * tuning.cost_coefficient.powf(purchased[tier]);
                    if energy >= cost {
                        energy -= cost;
                        counts[tier] += 1.0;
                        purchased[tier] += 1.0;
                        if unlock_times[tier].is_none() {
                            unlock_times[tier] = Some(second);
                        }
                        bought_this_second = true;
                        break;
                    }
                }
            }
        }

        unlock_times
    }

    #[test]
    fn higher_tiers_feed_lower_tier() {
        for tier in 0..GENERATOR_TUNING.len() {
            let output = output_resource_for_tier(tier);
            if tier == 0 {
                assert_eq!(output, "Energy");
            } else {
                assert_eq!(output, generator_id(tier - 1));
            }
        }
    }

    #[test]
    fn cost_scales_by_purchased_count() {
        let generator = Generator {
            id: "gen_energy_2".to_string(),
            name: "Quantum Capacitor".to_string(),
            output_resource: "gen_energy_1".to_string(),
            base_production: BigNumber::one(),
            count: BigNumber::zero(),
            purchased_count: BigNumber::new(5.0, 0),
            cost_resource: "Energy".to_string(),
            base_cost: BigNumber::new(300.0, 0),
            cost_coefficient: 1.16,
        };

        let expected = BigNumber::new(300.0 * 1.16_f64.powf(5.0), 0);
        assert_eq!(generator_cost(&generator), expected);
    }

    #[test]
    fn affordability_logic_handles_big_numbers() {
        let have = BigNumber::new(1.2, 6);
        let cheap = BigNumber::new(9.9, 5);
        let expensive = BigNumber::new(2.0, 6);

        assert!(can_afford(have, cheap));
        assert!(!can_afford(have, expensive));
    }

    #[test]
    fn deterministic_progression_targets_are_reasonable() {
        let times = simulate_unlock_times(90 * 24 * 3600);

        let t2 = times[1].expect("tier 2 should unlock");
        let t3 = times[2].expect("tier 3 should unlock");
        let t4 = times[3].expect("tier 4 should unlock");
        let t10 = times[9].expect("tier 10 should unlock in long-term simulation");

        assert!((120..=300).contains(&t2));
        assert!((300..=600).contains(&t3));
        assert!((3000..=7200).contains(&t4));
        assert!(t10 > 30 * 24 * 3600);
    }
}
