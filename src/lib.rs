#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub mod combat_interface;
pub mod combat_mechanics;
pub mod combat_setup;
pub mod creatures;
pub mod rng;
pub mod ui;

/// What's happening in the game?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    GameStart,
    InCombat,
    OutOfCombat,
}
