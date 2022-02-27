//! Displays text in the console and accepts input
use bevy::prelude::*;
use bevy_egui::egui::text::LayoutJob;
use bevy_egui::egui::{self, Color32, Frame, RichText, TextFormat, TextStyle};
use bevy_egui::EguiContext;
use leafwing_terminal::{TerminalConfiguration, TerminalPlugin};

use crate::rng::{RNGOutputs, CURRENT_RNG_VALUE_INDEX};

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

fn spawn_rng_window(
    mut egui_context: ResMut<EguiContext>, 
    windows: Res<Windows>,
    rng_values: Res<RNGOutputs>,
) {
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
            fill: Color32::BLACK,
            ..Default::default()
        })
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical(|ui_inner| {
                ui_inner.set_min_size((width, height).into());

                ui_inner.label(RichText::new("\nRNG STATE").monospace());
                ui_inner.label(RichText::new("=========\n").monospace());

                for (i, val) in rng_values.0.iter().enumerate() {

                    let fraction = (val.result as f32) / 255f32;

                    let low_color = Color32::from_rgb(39, 134, 217); // Blue
                    let high_color = Color32::from_rgb(237, 36, 36); // Red
                    let r = (high_color.r() as f32 - low_color.r() as f32) * fraction + low_color.r() as f32;
                    let g = (high_color.g() as f32 - low_color.g() as f32) * fraction + low_color.g() as f32;
                    let b = (high_color.b() as f32 - low_color.b() as f32) * fraction + low_color.b() as f32;
                    let color = Color32::from_rgb(r as u8, g as u8, b as u8);

                    let mut job = LayoutJob::default();
                    job.append(
                        format!("[{:03}, {:03}: ", val.s, val.t).as_str(),
                        0.0,
                        TextFormat {
                            style: TextStyle::Monospace,
                            color: color.linear_multiply(0.5),
                            ..Default::default()
                        }
                    );
                    job.append(
                        format!("{:03}", val.result).as_str(),
                        0.0,
                        TextFormat {
                            style: TextStyle::Monospace,
                            color,
                            ..Default::default()
                        }
                    );
                    job.append(
                        format!(" = {:.2}]", fraction).as_str(),
                        0.0,
                        TextFormat {
                            style: TextStyle::Monospace,
                            color: color.linear_multiply(0.8),
                            ..Default::default()
                        }
                    );
                    if i == CURRENT_RNG_VALUE_INDEX {
                        job.append(
                            " < UP NEXT",
                            0.0,
                            TextFormat {
                                style: TextStyle::Monospace,
                                color,
                                ..Default::default()
                            }
                        );
                    }

                    ui_inner.label(job);
                }
                ui_inner.label(RichText::new("...\n").monospace());
            });
        });
}
