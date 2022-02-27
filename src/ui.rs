//! Displays text in the console and accepts input
use bevy::prelude::*;
use bevy_egui::egui::RichText;
use bevy_egui::egui::{self, Color32, Frame};
use bevy_egui::EguiContext;
use leafwing_terminal::{TerminalConfiguration, TerminalPlugin};

/// Controls the display of text on the console
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TerminalPlugin)
            .add_system_to_stage(CoreStage::PreUpdate, sync_full_screen)
            .add_system(spawn_rng_window);
    }
}

const CONSOLE_FRACTION: f32 = 2. / 3.;

fn sync_full_screen(mut config: ResMut<TerminalConfiguration>, windows: Res<Windows>) {
    if windows.is_changed() {
        let window = windows.get_primary().unwrap();

        config.left_pos = 0.0;
        config.top_pos = 0.0;

        config.width = window.width() * CONSOLE_FRACTION;
        config.height = window.height();
    }
}

fn spawn_rng_window(mut egui_context: ResMut<EguiContext>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let width = (1f32 - CONSOLE_FRACTION) * window.width();
    let height = window.height();

    let left = CONSOLE_FRACTION * window.width();
    let top = 0.0;

    egui::Window::new("RNG Inspector")
        .collapsible(false)
        .fixed_pos([left, top])
        .title_bar(false)
        .frame(Frame {
            fill: Color32::DARK_GREEN,
            ..Default::default()
        })
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical(|ui_inner| {
                ui_inner.set_min_size((width, height).into());
                ui_inner.label(RichText::new("Hello world").monospace());
                ui_inner.label(RichText::new("this is another line").monospace());
            });
        });
}
