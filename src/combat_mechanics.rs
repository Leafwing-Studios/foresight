//! Structs and systems for core combat resolution

pub use attributes::*;
pub use damage::*;
pub use derived_stats::*;
pub use resources::*;

mod resources {
    use bevy::ecs::prelude::Component;
    use std::ops::{Add, AddAssign, Sub, SubAssign};

    use super::{Intelligence, Strength};

    /// A type that stores a resource with a current and max value
    pub trait Resource: Add<u8> + Sub<u8> + AddAssign<u8> + SubAssign<u8> + Sized + Clone {
        /// Creates a new struct with `max` and `current` equal to `max`
        #[must_use]
        fn new(max: u8) -> Self;

        /// The current resource value
        #[must_use]
        fn current(&self) -> u8;

        /// The max resource value
        #[must_use]
        fn max(&self) -> u8;

        /// Set the current resource value
        ///
        /// If a value greater than `max` is supplied, it is set to max instead
        fn set_current(&mut self, current: u8);

        /// Sets the maximum resource value
        ///
        /// If a value greater than `current` is supplied, `current` is reduced to `max`
        fn set_max(&mut self, max: u8);
    }

    /// The life points of a creature
    #[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
    pub struct Life {
        current: u8,
        max: u8,
    }

    impl Life {
        /// Computes the life total of a creature based on their [`Strength`]
        ///
        /// The `base` for the player is 40.
        #[must_use]
        fn compute(base: u8, strength: Strength) -> Self {
            Life::new(base + 4 * strength.0)
        }
    }

    impl Resource for Life {
        fn new(max: u8) -> Self {
            Self { current: max, max }
        }

        fn current(&self) -> u8 {
            self.current
        }

        fn max(&self) -> u8 {
            self.max
        }

        fn set_current(&mut self, current: u8) {
            if self.max <= current {
                self.current = current;
            } else {
                self.current = self.max;
            }
        }

        fn set_max(&mut self, max: u8) {
            self.max = max;

            if max > self.current {
                self.current = self.max;
            }
        }
    }

    /// The amount of mana a creature has to spend on spells
    #[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
    pub struct Mana {
        current: u8,
        max: u8,
    }

    impl Mana {
        /// Computes the mana total of a creature based on their [`Intelligence`](super::Strength)
        ///
        /// The `base` for the player is 50.
        #[must_use]
        fn compute(base: u8, intelligence: Intelligence) -> Self {
            Mana::new(base + intelligence.0)
        }
    }

    impl Resource for Mana {
        fn new(max: u8) -> Self {
            Self { current: max, max }
        }

        fn current(&self) -> u8 {
            self.current
        }

        fn max(&self) -> u8 {
            self.max
        }

        fn set_current(&mut self, current: u8) {
            if self.max <= current {
                self.current = current;
            } else {
                self.current = self.max;
            }
        }

        fn set_max(&mut self, max: u8) {
            self.max = max;

            if max > self.current {
                self.current = self.max;
            }
        }
    }
}

mod resource_impls {
    use super::Resource;
    use super::{Life, Mana};
    use std::ops::{Add, AddAssign, Sub, SubAssign};

    impl Add<u8> for Life {
        type Output = Life;

        fn add(self, rhs: u8) -> Self::Output {
            let mut new = self.clone();

            if let Some(new_value) = self.current().checked_add(rhs) {
                new.set_current(new_value);
            } else {
                new.set_current(u8::MAX);
            }
            new
        }
    }

    impl Sub<u8> for Life {
        type Output = Life;

        fn sub(self, rhs: u8) -> Self::Output {
            let mut new = self.clone();

            if let Some(new_value) = self.current().checked_sub(rhs) {
                new.set_current(new_value);
            } else {
                new.set_current(u8::MIN);
            }
            new
        }
    }

    impl AddAssign<u8> for Life {
        fn add_assign(&mut self, rhs: u8) {
            *self = self.clone() + rhs;
        }
    }

    impl SubAssign<u8> for Life {
        fn sub_assign(&mut self, rhs: u8) {
            *self = self.clone() - rhs;
        }
    }

    impl Add<u8> for Mana {
        type Output = Mana;

        fn add(self, rhs: u8) -> Self::Output {
            let mut new = self.clone();

            if let Some(new_value) = self.current().checked_add(rhs) {
                new.set_current(new_value);
            } else {
                new.set_current(u8::MAX);
            }
            new
        }
    }

    impl Sub<u8> for Mana {
        type Output = Mana;

        fn sub(self, rhs: u8) -> Self::Output {
            let mut new = self.clone();

            if let Some(new_value) = self.current().checked_add(rhs) {
                new.set_current(new_value);
            } else {
                new.set_current(u8::MIN);
            }
            new
        }
    }

    impl AddAssign<u8> for Mana {
        fn add_assign(&mut self, rhs: u8) {
            *self = self.clone() + rhs;
        }
    }

    impl SubAssign<u8> for Mana {
        fn sub_assign(&mut self, rhs: u8) {
            *self = self.clone() - rhs;
        }
    }
}

mod damage {
    use super::Life;
    use bevy::prelude::Component;
    use core::ops::*;

    /// Damage that is or could be dealt by an attack
    #[derive(Component, Clone, Debug, PartialEq)]
    pub struct Damage {
        min: u8,
        max: u8,
        actual: Option<u8>,
    }

    impl Damage {
        /// Creates a new struct that stores potential damage
        pub fn new(min: u8, max: u8) -> Damage {
            assert!(max >= min);

            Damage {
                min,
                max,
                actual: None,
            }
        }

        /// The minimum damage that could be dealt
        pub fn min(&self) -> u8 {
            self.min
        }

        /// The maximum damage that could be dealt
        pub fn max(&self) -> u8 {
            self.max
        }

        /// Determine how much damage is dealt based on a provided `rng`
        pub fn roll(&mut self, rng: u8) -> u8 {
            let fraction: f32 = rng as f32 / 255.;
            let range = self.max - self.min;

            let damage_f32 = self.min as f32 + fraction * range as f32;
            self.actual = Some(damage_f32 as u8);
            damage_f32 as u8
        }

        /// Reset the amount of damage that is dealt
        pub fn reset(&mut self) {
            self.actual = None;
        }

        /// Get the amount of damage that is rolled
        ///
        /// # Panics
        /// Panics if damage was not rolled, or was reset before this method was called.
        pub fn damage_rolled(&self) -> u8 {
            self.actual.unwrap()
        }
    }

    impl Sub<Damage> for Life {
        type Output = Life;

        fn sub(self, damage: Damage) -> Life {
            if let Some(damage_dealt) = damage.actual {
                self - damage_dealt
            } else {
                self
            }
        }
    }

    impl Add<u8> for Damage {
        type Output = Damage;

        fn add(self, int: u8) -> Damage {
            if let Some(damage) = self.actual {
                Damage {
                    actual: Some(damage.checked_add(int).unwrap_or(u8::MAX)),
                    ..self
                }
            } else {
                self
            }
        }
    }

    impl Sub<u8> for Damage {
        type Output = Damage;

        fn sub(self, int: u8) -> Damage {
            if let Some(damage) = self.actual {
                Damage {
                    actual: Some(damage.checked_sub(int).unwrap_or(u8::MIN)),
                    ..self
                }
            } else {
                self
            }
        }
    }

    impl Mul<u8> for Damage {
        type Output = Damage;

        fn mul(self, scaling: u8) -> Damage {
            if let Some(damage) = self.actual {
                Damage {
                    actual: Some(damage.checked_mul(scaling).unwrap_or(u8::MAX)),
                    ..self
                }
            } else {
                self
            }
        }
    }

    impl Div<u8> for Damage {
        type Output = Damage;

        fn div(self, scaling: u8) -> Damage {
            if let Some(damage) = self.actual {
                Damage {
                    actual: Some(damage.checked_div(scaling).unwrap_or(u8::MIN)),
                    ..self
                }
            } else {
                self
            }
        }
    }
}

mod attributes {
    use bevy::ecs::prelude::Component;

    /// The strength of a creature
    ///
    /// Increases damage dealt with attacks, and max life
    #[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct Strength(pub u8);

    /// The agility of a creature
    ///
    /// Increases dodge chance, crit chance and chance to flee
    #[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct Agility(pub u8);

    /// The intelligence of a creature
    ///
    /// Increases max mana and decreases spell failure chance
    #[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct Intelligence(pub u8);
}

mod derived_stats {
    use super::attributes::*;
    use bevy::ecs::prelude::Component;

    /// The chance to land a critical hit with an attack
    #[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
    pub struct CritChance(u8);

    impl CritChance {
        /// The base chance to dodge attacks
        const BASE: f32 = 0.;

        /// The marginal fraction of attacks that will become a hit for each point of agility gained
        const SCALING: f32 = 2.5 / 100.;

        /// Creates a new [`CritChance`] component
        pub fn new(agility: Agility) -> Self {
            let fraction: f32 = (Self::BASE + Self::SCALING * agility.0 as f32).clamp(0., 1.);

            CritChance(255 * fraction as u8)
        }

        /// Given a provided `rng` input, is the attack a crit?
        pub fn roll(&self, rng: u8) -> bool {
            self.0 >= rng
        }
    }

    /// The chance to dodge an attack
    #[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
    pub struct DodgeChance(u8);

    impl DodgeChance {
        /// The base chance to dodge attacks
        const BASE: f32 = 10. / 100.;

        /// The marginal fraction of attacks that will be dodged for each point of agility gained
        const SCALING: f32 = 1. / 100.;

        /// Creates a new [`Dodge`] component
        pub fn new(agility: Agility) -> Self {
            let fraction: f32 = (Self::BASE + Self::SCALING * agility.0 as f32).clamp(0., 1.);

            DodgeChance(255 * fraction as u8)
        }

        /// Given a provided `rng` input, is the attack dodged?
        pub fn roll(&self, rng: u8) -> bool {
            self.0 >= rng
        }
    }

    /// The chance to flee from combat
    #[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
    pub struct FleeChance(u8);

    impl FleeChance {
        /// The base chance to flee from combat
        const BASE: f32 = 10. / 100.;

        /// The marginal fraction of attempts to flee that will succeed for each point of agility gained
        const SCALING: f32 = 1. / 100.;

        /// Creates a new [`Flee`] component
        pub fn new(agility: Agility) -> Self {
            let fraction: f32 = (Self::BASE + Self::SCALING * agility.0 as f32).clamp(0., 1.);

            FleeChance(255 * fraction as u8)
        }

        /// Given a provided `rng` input, is the attack dodged?
        pub fn roll(&self, rng: u8) -> bool {
            self.0 >= rng
        }
    }

    /// The chance that a particular spell succeeds
    pub struct SpellSuccess(u8);

    impl SpellSuccess {
        /// The marginal fraction of  spells that will succeed for each point of intelligence gained
        const SCALING: f32 = 1. / 100.;

        /// Creates a new [`Flee`] component
        pub fn new(base_chance: u8, intelligence: Intelligence) -> Self {
            let fraction: f32 =
                (base_chance as f32 + Self::SCALING * intelligence.0 as f32).clamp(0., 1.);

            SpellSuccess(255 * fraction as u8)
        }

        /// Given a provided `rng` input, does the spell succeed?
        pub fn roll(&self, rng: u8) -> bool {
            self.0 >= rng
        }
    }
}
