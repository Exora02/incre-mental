use bevy::prelude::*;
use bevy::window::WindowResolution;

mod components;
mod game_state;
mod math;
mod systems;
mod ui;

use game_state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Incre-mental".to_string(),
                resolution: WindowResolution::new(360.0, 800.0), // Mobile-like portrait
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, (systems::setup_resources, ui::setup_ui))
        .add_systems(
            Update,
            (
                systems::production_tick.run_if(
                    in_state(GameState::Loading)
                        .or(in_state(GameState::Playing))
                        .or(in_state(GameState::MainMenu)),
                ), // Run always for now
                ui::update_resource_ui,
            ),
        )
        .run();
}
