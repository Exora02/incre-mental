use crate::components::*;
use crate::math::BigNumber;
use bevy::prelude::*;

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
    let generator_configs = vec![
        ("Fusion Cell", 10.0, 1.0),
        ("Quantum Capacitor", 100.0, 8.0),
        ("Antimatter Siphon", 1_000.0, 50.0),
        ("Void Harvester", 10_000.0, 300.0),
        ("Chrono Turbine", 100_000.0, 2_000.0),
        ("Stellar Forge", 1_000_000.0, 15_000.0),
        ("Galactic Bore", 10_000_000.0, 100_000.0),
        ("Dimension Weaver", 100_000_000.0, 800_000.0),
        ("Reality Anchor", 1_000_000_000.0, 6_000_000.0),
        ("Entropy Reverser", 10_000_000_000.0, 50_000_000.0),
    ];

    for (i, (name, cost, prod)) in generator_configs.iter().enumerate() {
        let initial_count = if i == 0 { 1.0 } else { 0.0 };
        let output = if i == 0 {
            "Energy".to_string()
        } else {
            format!("gen_energy_{}", i) // Produces previous generator
        };

        commands.spawn(Generator {
            id: format!("gen_energy_{}", i + 1),
            name: name.to_string(),
            output_resource: output,
            base_production: BigNumber::new(*prod, 0),
            count: BigNumber::new(initial_count, 0),
            purchased_count: BigNumber::new(initial_count, 0),
            cost_resource: "Energy".to_string(),
            base_cost: BigNumber::new(*cost, 0),
            cost_coefficient: 1.15,
        });
    }
}

// Production system
pub fn production_tick(
    time: Res<Time>,
    mut query_generators: Query<&mut Generator>,
    mut query_resources: Query<&mut GameResource>,
) {
    let delta = time.delta_secs_f64();
    let mut production_buffer: std::collections::HashMap<String, BigNumber> =
        std::collections::HashMap::new();

    // 1. Calculate Production
    for generator in query_generators.iter() {
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
    for mut generator in query_generators.iter_mut() {
        if let Some(amount) = production_buffer.get(&generator.id) {
            generator.count = generator.count + *amount;
        }
    }
}

pub fn handle_buy_interactions(
    _commands: Commands,
    mut interactions: Query<(&Interaction, &BuyGeneratorButton), Changed<Interaction>>,
    mut generators: Query<&mut Generator>,
    mut resources: Query<&mut GameResource>,
) {
    for (interaction, buy_button) in interactions.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut generator) = generators.get_mut(buy_button.0) {
                // Cost scales with PURCHASED count, not total count
                let cost_mult = generator
                    .cost_coefficient
                    .powf(generator.purchased_count.to_f64());
                let cost = generator.base_cost * cost_mult;

                // Check affordability
                let mut can_afford = false;
                for resource in resources.iter() {
                    if resource.name == generator.cost_resource {
                        // Check if we have enough
                        let diff = resource.amount.exponent - cost.exponent;
                        if diff > 0 {
                            can_afford = true;
                        } else if diff == 0 && resource.amount.mantissa >= cost.mantissa {
                            can_afford = true;
                        }
                    }
                }

                if can_afford {
                    // Pay the cost
                    for mut resource in resources.iter_mut() {
                        if resource.name == generator.cost_resource {
                            resource.amount = resource.amount - cost;
                        }
                    }

                    // Increase generator count AND purchased count
                    generator.count = generator.count + BigNumber::new(1.0, 0);
                    generator.purchased_count = generator.purchased_count + BigNumber::new(1.0, 0);
                    println!("Bought generator: {}", generator.name);
                } else {
                    println!("Cannot afford generator: {}", generator.name);
                }
            }
        }
    }
}
