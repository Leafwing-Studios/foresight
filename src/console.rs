//! Displays text in the console and accepts input

use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsoleOpen};

/// Controls the display of text on the console
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_console::ConsolePlugin)
            .add_startup_system(configure_console);
    }
}

fn configure_console(
    mut config: ResMut<ConsoleConfiguration>,
    windows: Res<Windows>,
    mut console_open: ResMut<ConsoleOpen>,
) {
    const COMMAND_LINE_HEIGHT: f32 = 30.;

    // Match the size of the window
    let window = windows.get_primary().unwrap();

    config.width = window.width();
    // Need a bit of space for the command line
    config.height = window.height() - COMMAND_LINE_HEIGHT;

    // The console is always is open
    console_open.open = true;
    config.keys = Vec::default();
}
