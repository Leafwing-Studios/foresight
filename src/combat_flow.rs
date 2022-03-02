//! Describes the flow of combat, and the terminal commands that can be issued

use bevy::prelude::*;
use bevy_system_graph::SystemGraph;

pub use components::*;
pub use resources::*;
use systems::*;

/// Controls the flow of combat
pub struct CombatFlowPlugin;

impl Plugin for CombatFlowPlugin {
    fn build(&self, app: &mut App) {
        // The player always goes first
        app.insert_resource(CurrentTurn::Player)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemGraph::new()
                    .root(end_turn_when_no_ap)
                    .then(update_active_creature)
                    .graph()
                    .into(),
            )
            // Runs at the end of PreUpdate
            .add_system_to_stage(CoreStage::PreUpdate, advance_action.exclusive_system());
    }
}

mod systems {
    use super::{Active, CurrentTurn, Inactive};
    use crate::actions::Actions;
    use crate::combat_statistics::ActionPoints;
    use crate::creatures::{Monster, Player};
    use bevy::prelude::*;

    pub(super) fn end_turn_when_no_ap(
        query: Query<&ActionPoints, With<Active>>,
        mut current_turn: ResMut<CurrentTurn>,
    ) {
        let action_points = query.single();

        if *action_points == 0 {
            current_turn.swap();
        }
    }

    pub(super) fn update_active_creature(
        mut commands: Commands,
        current_turn: Res<CurrentTurn>,
        player_query: Query<Entity, With<Player>>,
        monster_query: Query<Entity, With<Monster>>,
    ) {
        if current_turn.is_changed() {
            let player = player_query.single();
            let monster = monster_query.single();

            match *current_turn {
                CurrentTurn::Player => {
                    commands.entity(player).insert(Active).remove::<Inactive>();
                    commands.entity(monster).insert(Inactive).remove::<Active>();
                }
                CurrentTurn::Monster => {
                    commands.entity(monster).insert(Active).remove::<Inactive>();
                    commands.entity(player).insert(Inactive).remove::<Active>();
                }
            }
        }
    }

    /// Runs the next system in the [`Action`] on the [`World`] when any keyboard button is pressed
    pub(super) fn advance_action(world: &mut World) {
        // Is an action active?
        world.resource_scope(|world, mut actions: Mut<Actions>| {
            if let Some(action_name) = actions.current() {
                let keyboard_input: &Input<KeyCode> = world.get_resource().unwrap();

                if keyboard_input.just_pressed(KeyCode::Return) {
                    // Run the next system in the action on the world
                    let action = actions.get_mut(action_name);
                    action.advance(world);

                    // Reset the state of the `Action` if it's complete
                    if action.is_finished() {
                        // Reset the current system back to the beginning
                        action.reset();

                        // Clear the current action
                        actions.clear();
                    }
                }
            }
        });
    }
}

mod resources {
    /// Whose turn is it?
    #[allow(missing_docs)]
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum CurrentTurn {
        Player,
        Monster,
    }

    impl CurrentTurn {
        /// Changes the turn to the opposite variant
        pub fn swap(&mut self) {
            use CurrentTurn::*;
            *self = match self {
                Player => Monster,
                Monster => Player,
            }
        }
    }
}

mod components {
    use bevy::prelude::*;

    #[derive(Component, Debug)]
    /// An entity whose turn it is
    ///
    /// The counterpart to [`Inactive`], controlled by [`CurrentTurn`].
    pub struct Active;

    #[derive(Component, Debug)]
    /// An entity whose turn it is
    ///
    /// The counterpart to [`Inactive`], controlled by [`CurrentTurn`].
    pub struct Inactive;
}
