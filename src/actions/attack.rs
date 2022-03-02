use crate::actions::Action;
use crate::system_sequence::SystemSeq;
use leafwing_terminal::TerminalCommand;

#[derive(TerminalCommand)]
#[terminal_command(name = "attack")]
pub(super) struct AttackCommand;

impl Action {
    /// Creates a new [`Action`] that corresponds to an [`AttackCommand`]
    pub fn attack() -> Action {
        Action::new(
            "Attack",
            SystemSeq::new()
                .then(roll_hit)
                .then(roll_damage)
                .then(roll_crit),
        )
    }
}

fn roll_hit() {}

fn roll_damage() {}

fn roll_crit() {}
