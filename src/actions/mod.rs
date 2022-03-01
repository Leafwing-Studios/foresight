//! Actions that can be used by both players and monsters

use crate::combat_flow::{CurrentAction, CurrentTurn};
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use core::fmt::Debug;
use leafwing_terminal::*;

mod attack;
use attack::*;

/// Adds [`TerminalCommands`](TerminalCommand) and [`Actions`](Action) for all of the available actions
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_action::<AttackCommand, AttackAction>();
    }
}

/// An in-game action
pub trait Action: Debug + Send + Sync + 'static {
    /// The formatted name of the action
    fn name(&self) -> String;

    /// Runs the next system on the world
    fn advance(&self, world: &mut World);
}

trait Commandlike: Resource + CommandName + CommandArgs + CommandHelp {}

impl<T: Resource + CommandName + CommandArgs + CommandHelp> Commandlike for T {}

fn start_action<TC: Commandlike, A: Action + Default>(
    mut terminal_command: TerminalCommand<TC>,
    current_turn: Res<CurrentTurn>,
    mut current_action: ResMut<CurrentAction>,
) {
    // Break early if the command was not entered or was malformed
    if terminal_command.take().is_none() {
        return;
    }

    match current_action.action() {
        // Break early if it is not time for input
        Some(_) => terminal_command.reply("You cannot use actions when another action is queued."),
        // If everything is okay, set this action as the current action, from the beginning of its resolution
        None => current_action.set_action(A::default()),
    };
}

trait ActionExt {
    fn add_action<TC: Commandlike, A: Action + Default>(&mut self);
}

impl ActionExt for App {
    fn add_action<TC: Commandlike, A: Action + Default>(&mut self) {
        self.add_terminal_command::<TC, _, _>(start_action::<TC, A>);
    }
}
