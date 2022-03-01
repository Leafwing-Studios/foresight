//! Entities that can take part in combat

use crate::combat_mechanics::*;
use bevy::prelude::*;

/// A marker component for the player entity
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player;

/// A marker component for the enemy entity
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enemy;

/// A bundle containing everything needed to let a unit fight
#[derive(Bundle)]
#[allow(missing_docs)]
pub struct CreatureBundle {
    pub life: Life,
    pub mana: Mana,
    pub damage: Damage,
    pub crit_chance: CritChance,
    pub dodge_chance: DodgeChance,
    pub flee_chance: FleeChance,
}
