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

    // Basic Generator: Fusion Cell
    commands.spawn(Generator {
        id: "gen_energy_1".to_string(),
        name: "Fusion Cell".to_string(),
        output_resource: "Energy".to_string(),
        base_production: BigNumber::new(1.0, 0), // 1 per sec
        count: 1,                                // Start with 1 for testing
        cost_resource: "Energy".to_string(),
        base_cost: BigNumber::new(10.0, 0),
        cost_coefficient: 1.15,
    });
}

// Production system
pub fn production_tick(
    time: Res<Time>,
    query_generators: Query<&Generator>,
    mut query_resources: Query<&mut GameResource>,
) {
    let delta = time.delta_secs_f64();

    // Simple O(N*M) approach for now, optimize later with HashMaps if needed
    for generator in query_generators.iter() {
        if generator.count > 0 {
            let production = generator.base_production * (generator.count as f64) * delta;

            for mut resource in query_resources.iter_mut() {
                if resource.name == generator.output_resource {
                    resource.amount = resource.amount + production;
                }
            }
        }
    }
}
