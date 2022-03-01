//! Actions that can be used by both players and monsters

use bevy::prelude::*;
use core::fmt::Debug;
use leafwing_terminal::*;

mod attack;

/// Adds [`TerminalCommands`](TerminalCommand) and [`Actions`](Action) for all of the available actions
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_terminal_command::<attack::AttackCommand, _, _>(attack::send_attack);
    }
}

/// An in-game action
pub trait Action: Debug + Send + Sync + 'static {
    /// The formatted name of the action
    fn name(&self) -> String;

    /// Runs the next system on the world
    fn advance(&self, world: &mut World);
}
