use bevy::prelude::*;
use foresight_lib::*;

fn main() {
    App::new()
        // Configure the game window
        /*
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "Foresight".to_string(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        */
        // Standard Bevy functionality
        .add_plugins(DefaultPlugins)
        // Foresight game plugins
        .add_plugin(ui::UiPlugin)
        .add_plugin(rng::RNGPlugin)
        .add_plugin(combat_flow::CombatFlowPlugin)
        .add_plugin(actions::ActionPlugin)
        .run();
}
