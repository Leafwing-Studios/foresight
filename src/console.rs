//! Displays text in the console and accepts input
use bevy::prelude::*;
use bevy_egui::egui::RichText;
use bevy_egui::egui::{self};
use bevy_egui::EguiContext;
use leafwing_terminal::ConsoleConfiguration;

/// Controls the display of text on the console
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(leafwing_terminal::ConsolePlugin)
            .add_startup_system(configure_console)
            .add_system(spawn_rng_window);
    }
}

const COMMAND_LINE_HEIGHT: f32 = 30.;
const CONSOLE_FRACTION: f32 = 2. / 3.;

fn configure_console(mut config: ResMut<ConsoleConfiguration>, windows: Res<Windows>) {
    // Match the size of the window
    let window = windows.get_primary().unwrap();

    config.left_pos = 0.0 - (1.0 - CONSOLE_FRACTION) * window.width();

    config.width = CONSOLE_FRACTION * window.width();
    // Need a bit of space for the command line
    config.height = window.height() - COMMAND_LINE_HEIGHT;
}

fn spawn_rng_window(mut egui_context: ResMut<EguiContext>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let left = CONSOLE_FRACTION * width;
    let top = 0.0;

    let margin_offset = 25f32;

    egui::Window::new("RNG Inspector")
        .collapsible(false)
        .resizable(false)
        .default_pos([left + margin_offset, top])
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical(|ui_inner| {
                let width = (1f32 - CONSOLE_FRACTION) * width - margin_offset;

                ui_inner.set_min_size((width, height).into());
                ui_inner.label(RichText::new("Hello world").monospace());
                ui_inner.label(RichText::new("this is another line").monospace());
            });
        });
}
