//! Entities that can take part in combat

use crate::{actions::AvailableActions, combat_statistics::*};
use bevy::prelude::*;

/// A marker component for the player entity
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player;

/// A marker component for the enemies that the player fights
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Monster;

/// The bundle of components used by the [`Player`]
#[derive(Bundle)]
#[allow(missing_docs)]
pub struct PlayerBundle {
    pub player: Player,
    pub life: Life,
    pub mana: Mana,
    pub ap: ActionPoints,
    pub actions: AvailableActions,
    pub damage: Damage,
    pub crit_chance: CritChance,
    pub dodge_chance: DodgeChance,
    pub flee_chance: FleeChance,
}

/// The bundle of components used by the [`Monster`]
#[derive(Bundle)]
#[allow(missing_docs)]
pub struct MonsterBundle {
    pub monster: Monster,
    pub life: Life,
    pub mana: Mana,
    pub ap: ActionPoints,
    pub actions: AvailableActions,
    pub damage: Damage,
    pub crit_chance: CritChance,
    pub dodge_chance: DodgeChance,
    pub flee_chance: FleeChance,
}
