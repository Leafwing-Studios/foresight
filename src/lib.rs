#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub mod actions;
pub mod combat_flow;
pub mod combat_setup;
pub mod combat_statistics;
pub mod creatures;
pub mod rng;
pub mod ui;

mod keyboard_variants;

/// What's happening in the game?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    GameStart,
    InCombat,
    OutOfCombat,
}
