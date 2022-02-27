//! Displays text in the console and accepts input

use bevy::prelude::*;
use bevy_console::ConsoleConfiguration;

/// Controls the display of text on the console
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_console::ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                ..Default::default()
            });
    }
}
