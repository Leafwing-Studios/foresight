use crate::actions::Action;
use bevy::prelude::IntoSystem;
use leafwing_terminal::TerminalCommand;

#[derive(TerminalCommand)]
#[terminal_command(name = "attack")]
pub(super) struct AttackCommand;

impl Action {
    /// Creates a new [`Action`] that corresponds to an [`AttackCommand`]
    pub fn attack() -> Action {
        Action::new(
            "Attack",
            vec![
                Box::new(roll_hit.system()),
                Box::new(roll_damage.system()),
                Box::new(roll_crit.system()),
            ],
        )
    }
}

fn roll_hit() {}

fn roll_damage() {}

fn roll_crit() {}
