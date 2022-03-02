use bevy::prelude::*;
use bevy::utils::HashSet;

/// The set of available [`Actions`](crate::actions::Action) available to a creature
#[derive(Component, Debug, Default)]
pub struct AvailableActions {
    set: HashSet<String>,
}

impl AvailableActions {
    /// Inserts a new [`Action`](crate::actions::Action) by its label
    ///
    /// Fails with a warning if [`u8::MAX`](u8) actions are already registered.
    pub fn insert(&mut self, label: String) {
        if self.len() < u8::MAX {
            self.set.insert(label);
        } else {
            warn!("Too many actions were inserted!");
        }
    }

    /// Lists the available actions in alphabetical order
    pub fn list(&self) -> Vec<String> {
        let mut vec: Vec<String> = self.set.iter().cloned().collect();
        vec.sort();
        vec
    }

    /// Are there any actions registered
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The number of available actions
    pub fn len(&self) -> u8 {
        self.set.len() as u8
    }

    /// Selects a random action from the available set
    pub fn random(&self, rng: u8) -> String {
        // Get a sorted list
        let list = self.list();

        // Use a modulo function to break this up nicely
        let index = rng % self.len();
        list[index as usize].clone()
    }
}
