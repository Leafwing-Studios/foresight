//! Tools for constructing linear sequences of [`Systems`](System)

use bevy::ecs::system::{IntoSystem, System};
use bevy::ecs::world::World;

/// A linear sequence of unchained [`Systems`](System)
///
/// Each system runs exclusively on the world.
#[derive(Default)]
pub struct SystemSeq {
    systems: Vec<Box<dyn System<In = (), Out = ()>>>,
    initialized: Vec<bool>,
    index: usize,
}

impl SystemSeq {
    /// Creates a new, empty [`SystemSeq`]
    #[must_use]
    pub fn new() -> Self {
        SystemSeq {
            systems: Vec::new(),
            initialized: Vec::new(),
            index: 0,
        }
    }

    /// Appends a system to the end of the current list of systems
    #[must_use]
    pub fn then<Params, S: IntoSystem<(), (), Params>>(mut self, system: S) -> Self {
        let i = self.len() + 1;
        let boxed_system: Box<dyn System<In = (), Out = ()>> = Box::new(system.system());

        self.systems[i] = boxed_system;
        self.initialized[i] = false;
        self
    }

    /// Initializes the system at the provided `index` on the [`World`]
    fn initialize_one(&mut self, index: usize, world: &mut World) {
        assert!(index <= self.systems.len());
        assert!(index <= self.initialized.len());

        if !self.initialized[index] {
            self.systems[index].initialize(world);
            self.initialized[index] = true;
        }
    }

    /// Runs the system at the provided `index` on the [`World`]
    ///
    /// [`Commands`](bevy::ecs::system::Commands) are flushed after the system runs
    pub fn run_one(&mut self, index: usize, world: &mut World) {
        assert!(index <= self.systems.len());

        self.systems[index].run((), world);
        self.systems[index].apply_buffers(world);
    }

    /// Runs the next system at the provided `index` on the [`World`]
    ///
    /// [`Commands`](bevy::ecs::system::Commands) are flushed after the system runs
    pub fn run_next(&mut self, world: &mut World) {
        if self.index < self.len() {
            self.run_one(self.index, world);
            self.index += 1;
        }
    }

    /// Runs every system in the set on the provided [`World`]
    ///
    /// [`Commands`](bevy::ecs::system::Commands) are flushed after each system
    pub fn run_all(&mut self, world: &mut World) {
        self.reset();

        while !self.is_finished() {
            self.run_next(world);
        }

        self.reset();
    }

    /// How many systems are stored?
    #[must_use]
    pub fn len(&self) -> usize {
        debug_assert!(self.systems.len() == self.initialized.len());
        self.systems.len()
    }

    /// Are any systems stored
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.systems.is_empty()
    }

    /// The current internal iteration `index`
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Have all the systems been iterated over?
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.index >= self.systems.len()
    }

    /// Resets the internal iteration `index` to the start
    pub fn reset(&mut self) {
        self.index = 0;
    }
}
