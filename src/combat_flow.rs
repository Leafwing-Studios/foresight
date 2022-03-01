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
            .init_resource::<CurrentAction>()
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
    use super::{Active, CurrentAction, CurrentTurn, Inactive};
    use crate::combat_statistics::ActionPoints;
    use crate::creatures::{Enemy, Player};
    use crate::keyboard_variants::ANY_KEY;
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
        enemy_query: Query<Entity, With<Enemy>>,
    ) {
        if current_turn.is_changed() {
            let player = player_query.single();
            let enemy = enemy_query.single();

            match *current_turn {
                CurrentTurn::Player => {
                    commands.entity(player).insert(Active).remove::<Inactive>();
                    commands.entity(enemy).insert(Inactive).remove::<Active>();
                }
                CurrentTurn::Enemy => {
                    commands.entity(enemy).insert(Active).remove::<Inactive>();
                    commands.entity(player).insert(Inactive).remove::<Active>();
                }
            }
        }
    }

    /// Runs the next system in the [`Action`] on the [`World`] when any keyboard button is pressed
    pub(super) fn advance_action(world: &mut World) {
        // Is an action active?
        world.resource_scope(|world, current_action: Mut<CurrentAction>| {
            if let Some(action) = current_action.action() {
                let keyboard_input: &Input<KeyCode> = world.get_resource().unwrap();

                if keyboard_input.any_just_pressed(ANY_KEY) {
                    action.advance(world);
                }
            }
        });
    }
}

mod resources {
    use crate::actions::Action;

    /// Whose turn is it?
    #[allow(missing_docs)]
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum CurrentTurn {
        Player,
        Enemy,
    }

    impl CurrentTurn {
        /// Changes the turn to the opposite variant
        pub fn swap(&mut self) {
            use CurrentTurn::*;
            *self = match self {
                Player => Enemy,
                Enemy => Player,
            }
        }
    }

    /// Which is action is currently being taken
    #[derive(Default, Debug)]
    pub struct CurrentAction {
        action: Option<Box<dyn Action>>,
    }

    impl CurrentAction {
        /// Gets the current action
        pub fn action(&self) -> &Option<Box<dyn Action>> {
            &self.action
        }

        /// Sets the current action
        pub fn set_action(&mut self, action: impl Action) {
            self.action = Some(Box::new(action));
        }

        /// Clears the current action because it is complete
        fn clear(&mut self) {
            self.action = None;
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