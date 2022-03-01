use crate::actions::Action;
use crate::combat_flow::{CurrentAction, CurrentTurn};
use bevy::prelude::*;
use leafwing_terminal::*;

#[derive(TerminalCommand)]
#[terminal_command(name = "attack")]
pub(super) struct AttackCommand;

/// Deals damage to the opponent, based on the active player
pub(super) fn send_attack(
    mut terminal_command: TerminalCommand<AttackCommand>,
    current_turn: Res<CurrentTurn>,
    mut current_action: ResMut<CurrentAction>,
) {
    // Break early if the command was not entered or was malformed
    if terminal_command.take().is_none() {
        return;
    }

    // Break early if it is not time for input
    match current_action.action() {
        Some(_) => terminal_command.reply("You cannot use actions when another action is queued."),
        None => current_action.set_action(Attack),
    };
}

#[derive(Debug)]
struct Attack;

impl Action for Attack {
    fn name(&self) -> String {
        "Attack".to_string()
    }

    fn advance(&self, _world: &mut World) {}
}
