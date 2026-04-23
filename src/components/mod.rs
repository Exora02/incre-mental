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
    pub count: BigNumber,
    /// Number of generators manually purchased (affects cost)
    pub purchased_count: BigNumber,
    /// Resource ID required to buy this
    pub cost_resource: String,
    pub base_cost: BigNumber,
    pub cost_coefficient: f64,
}

#[derive(Component)]
pub struct Player {
    pub money: BigNumber,
}

#[derive(Component)]
pub struct BuyGeneratorButton(pub Entity);

#[derive(Clone, Debug, PartialEq)]
pub enum UpgradeEffect {
    ProductionMultiplier(f64),
}

#[derive(Component, Debug, Clone)]
pub struct Upgrade {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost_resource: String,
    pub cost_amount: BigNumber,
    pub is_purchased: bool,
    pub effect: UpgradeEffect,
    pub target_generator: Option<String>, // None means global
}

#[derive(Component)]
pub struct BuyUpgradeButton(pub Entity);

#[derive(Component)]
pub struct PrestigeButton;
