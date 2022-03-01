use crate::actions::Action;
use bevy::prelude::*;
use leafwing_terminal::*;

#[derive(TerminalCommand)]
#[terminal_command(name = "attack")]
pub(super) struct AttackCommand;

#[derive(Debug, Default)]
pub struct AttackAction;

impl Action for AttackAction {
    fn name(&self) -> String {
        "Attack".to_string()
    }

    fn advance(&self, _world: &mut World) {}
}
