use crate::math::BigNumber;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GameResource {
    pub name: String,
    pub amount: BigNumber,
    pub unlocked: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Generator {
    pub id: String,
    pub name: String,
    /// Resource ID that this generator produces
    pub output_resource: String,
    /// How much it produces per second
    pub base_production: BigNumber,
    pub count: u32,
    /// Resource ID required to buy this
    pub cost_resource: String,
    pub base_cost: BigNumber,
    pub cost_coefficient: f64,
}

#[derive(Component)]
pub struct Player {
    pub money: BigNumber,
}
