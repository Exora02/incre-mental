use bevy::prelude::*;
use bevy::window::WindowResolution;

mod components;
mod game_state;
mod math;
mod systems;
mod ui;

use game_state::GameState;
use ui::UiTab;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Incre-mental".to_string(),
                resolution: WindowResolution::new(1280, 720),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_state::<UiTab>()
        .add_systems(Startup, (systems::setup_resources, ui::setup_ui))
        .add_systems(
            Update,
            (
                systems::production_tick.run_if(
                    in_state(GameState::Loading)
                        .or(in_state(GameState::Playing))
                        .or(in_state(GameState::MainMenu)),
                ), // Run always for now
                systems::handle_buy_interactions,
                systems::handle_buy_upgrades,
                ui::update_resource_ui,
                ui::spawn_generator_ui,
                ui::spawn_upgrade_ui,
                ui::handle_tab_switches,
                ui::update_tab_ui,
                ui::update_generator_ui,
                ui::update_generator_text,
            ),
        )
        .run();
}
