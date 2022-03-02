use crate::actions::Action;
use leafwing_terminal::TerminalCommand;

#[derive(TerminalCommand)]
#[terminal_command(name = "attack")]
pub(super) struct AttackCommand;

impl Action {
    /// Creates a new [`Action`] that corresponds to an [`AttackCommand`]
    pub fn attack() -> Action {
        Action::new("Attack", Vec::default())
    }
}
