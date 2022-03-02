//! Actions that can be used by both players and monsters

use crate::system_sequence::SystemSeq;
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy::utils::HashMap;
use leafwing_terminal::*;

mod available_actions;
pub use available_actions::AvailableActions;

mod attack;
use attack::*;

/// Adds [`TerminalCommands`](TerminalCommand) and [`Actions`](Action) for all of the available actions
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_action::<AttackCommand>(Action::attack());
    }
}

/// An action that can be applied to the [`World`] in a step-by-step fashion
pub struct Action {
    name: String,
    systems: SystemSeq,
}

impl Action {
    /// Creates a new [`Action`], whose `systems` will be applied to the [`World`] one step at a time
    pub fn new(name: impl Into<String>, systems: SystemSeq) -> Self {
        Action {
            name: name.into(),
            systems: systems,
        }
    }

    /// The name of the action
    ///
    /// This is immutable after creation.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Applies the next step of the action to the [`World`], according to the provided vector of `systems`
    pub fn advance(&mut self, world: &mut World) {
        self.systems.run_next(world);
    }

    /// Is the action out of systems?
    pub fn is_finished(&self) -> bool {
        self.systems.is_finished()
    }

    /// Resets the pattern of applied `systems` to the beginning of the supplied list
    pub fn reset(&mut self) {
        self.systems.reset();
    }
}

/// The total list of available [`Action`], stored as a resource
#[derive(Default)]
pub struct Actions {
    current: Option<String>,
    map: HashMap<String, Action>,
}

impl Actions {
    /// Gets the current action
    pub fn current(&self) -> Option<String> {
        self.current.clone()
    }

    /// Sets the current action
    pub fn set_current(&mut self, action_name: String) {
        assert!(self.map.contains_key(&action_name));

        self.current = Some(action_name);
    }

    /// Clears the current action
    pub fn clear(&mut self) {
        self.current = None;
    }

    /// Inserts an [`Action`] into this collection
    pub fn insert(&mut self, action: Action) {
        self.map.insert(action.name(), action);
    }

    /// Gets an immutable mutable reference to the underlying [`Action`] with the `action_name`
    pub fn get(&self, action_name: String) -> &Action {
        self.map
            .get(&action_name)
            .unwrap_or_else(|| panic!("No action named {action_name} was found in `Actions`."))
    }

    /// Gets a mutable reference to the underlying [`Action`] with the `action_name`
    pub fn get_mut(&mut self, action_name: String) -> &mut Action {
        self.map
            .get_mut(&action_name)
            .unwrap_or_else(|| panic!("No action named {action_name} was found in `Actions`."))
    }
}

trait Commandlike: Resource + CommandName + CommandArgs + CommandHelp {}

impl<T: Resource + CommandName + CommandArgs + CommandHelp> Commandlike for T {}

trait ActionExt {
    fn add_action<TC: Commandlike>(&mut self, action: Action);
}

impl ActionExt for App {
    fn add_action<TC: Commandlike>(&mut self, action: Action) {
        // Register a system to listen for the TC terminal command
        self.add_terminal_command::<TC, _, _>(create_start_action_system::<TC>(action.name()));
        // Add the action to the Actions collection
        let mut actions = self.world.get_resource_mut::<Actions>().unwrap();
        actions.insert(action);
    }
}

fn create_start_action_system<TC: Commandlike>(
    action_name: String,
) -> impl FnMut(TerminalCommand<TC>, ResMut<Actions>) {
    move |mut terminal_command: TerminalCommand<TC>, mut actions: ResMut<Actions>| {
        // Break early if the command was not entered or was malformed
        if terminal_command.take().is_none() {
            return;
        }

        if let Some(_action_name) = actions.current() {
            terminal_command.reply("You cannot use actions when another action is queued.");
        } else {
            actions.set_current(action_name.clone());
        }
    }
}
