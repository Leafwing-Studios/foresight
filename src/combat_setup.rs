//! Transition in and out of combat

use crate::GameState;
use bevy::prelude::*;

/// Adds, removes and cleans up entities when combat starts and ends
pub struct CombatSetupPlugin;

impl Plugin for CombatSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::GameStart).add_system_set(
            SystemSet::on_enter(GameState::GameStart)
                .with_system(spawn_player)
                .with_system(spawn_enemy),
        );
    }
}

fn spawn_player(mut commands: Commands) {
    todo!()
}

fn spawn_enemy(mut commands: Commands) {
    todo!()
}
