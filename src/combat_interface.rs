//! Describes the flow of combat, and the terminal commands that can be issued

use crate::combat_mechanics::*;
use crate::rng::Rng;
use bevy::prelude::*;
use leafwing_terminal::{reply, AddTerminalCommand, TerminalCommand};

/// Controls the flow of combat, and adds combat-specific terminal commands
pub struct CombatInterfacePlugin;

/// What can currently be done?
#[derive(PartialEq, Clone, Debug)]
enum TurnState {
    Input(CurrentTurn),
    Resolution(CurrentTurn),
    Help,
}

/// Whose turn is it?
#[derive(PartialEq, Clone, Debug)]
enum CurrentTurn {
    Player,
    Enemy,
}

#[derive(Component, Debug)]
struct Active;

#[derive(Component, Debug)]
struct Inactive;

impl Plugin for CombatInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentTurn::Player)
            .add_terminal_command::<LogCommand, _, _>(log_command)
            .add_terminal_command::<AttackCommand, _, _>(attack);
    }
}

/// Prints given arguments to the terminal
#[derive(TerminalCommand)]
#[terminal_command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}

fn log_command(mut log: TerminalCommand<LogCommand>) {
    if let Some(LogCommand { msg, num }) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}

#[derive(TerminalCommand)]
#[terminal_command(name = "attack")]
struct AttackCommand;

/// Deals damage to the opponent, based on the active player
fn attack(
    mut terminal_command: TerminalCommand<AttackCommand>,
    mut rng: ResMut<Rng>,
    turn_state: Res<TurnState>,
    mut active_query: Query<&mut Damage, With<Active>>,
    mut inactive_query: Query<&mut Life, With<Inactive>>,
) {
    // Break early if the command was not entered or was malformed
    if terminal_command.take().is_none() {
        return;
    }

    // Break early if it is not time for input
    let current_turn_state = turn_state.clone();
    match current_turn_state {
        TurnState::Input(_) => {
            let mut attacker_damage = active_query.single_mut();
            let mut defender_life = inactive_query.single_mut();

            let damage_dealt = attacker_damage.roll(rng.gen());
            *defender_life -= damage_dealt;

            terminal_command.reply(format!("{damage_dealt} damage was dealt!"));
        }
        _ => terminal_command.reply("You cannot use actions when it is not your turn."),
    };
}
