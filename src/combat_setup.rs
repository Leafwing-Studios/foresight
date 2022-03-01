//! Transition in and out of combat

use crate::combat_statistics::{Damage, Life, Resource};
use crate::creatures::{Enemy, Player};
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
    commands
        .spawn()
        .insert(Life::new(10))
        .insert(Damage::new(3, 5))
        .insert(Player);
}

fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn()
        .insert(Life::new(10))
        .insert(Damage::new(6, 9))
        .insert(Enemy);
}
