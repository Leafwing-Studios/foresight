# Overview

A classical turn-based fantasy combat system.

## Resources

- life: 40+4*str for the player, monster health is just set as a value
- mana: 50+int

Both restored to full after each fight.

## Actions

\* indicates that they are learned from monsters

### Major

- attack
  - crit chance: floor(2.5*agl)
  - base damage is around 8 + str (but check the scaling on this, and double check the math)
    - monsters are 2-3 shotting the player
    - player is taking about 10 hits to kill the monster
  - heavy
    - 8-10 damage (+ 2*str)
    - 3 RNG (dodge, damage, crit)
  - light
    - two attacks: 3-6 (+str) damage each
    - 5 RNG (dodge, damage, damage, crit, crit)
- cast a spell
  - spell fail chance: base_chance - int (compared directly to the u8 rng, not a percentage)
- pocket sand!*
  - 1 RNG (dodge)
  - Reduce enemy damage by 50% for three turns

### Minor

- utility spells
- dodge
  - O RNG
  - Increases dodge chance by 20%
  - Base dodge chance: 10+agl
- flee
  - 1 RNG
  - on a success, hp and such are restored, but you have to start the fight again
  - flee chance: 15+4*agl?
- scan
  - 1 RNG
  - gives you one new stat from the enemy at random (chosen uniformly from stats you haven't yet seen this fight)
- focus*
  - 0 RNG
  - Reduce your spell fail chance by 20
- trip*
  - 2 RNG (dodge)
  - Reduce enemy dodge by 20-25% for one turn

### Free

- status
  - show known stats side-by-side
- about
  - shows percentages and damage values for your actions

## Hooks

- damage
- dodge
- spell failure chance
- crits
- health
- mana
- number of times the RNG advances
- scan
- flee

## Stats

- ranges from 0-20 inclusive
- player starts at 0, 0, 0
- strength
  - increases basic attack damage
  - increases max life
- agility
  - increases dodge chance
  - increases crit chance
- intelligence
  - increase max mana
  - reduces spell failure chance

## RNG

- range of 0-255
- current value and next few values are shown to players
- higher numbers are generally good for actor
  - attacker rolls for spells
  - defender rolls for dodge chance
  - then, damage rolled by attacker
- A value of 255 is always a failure, regardless of actual chance
- order of RNG usages must be very clear
- enemies select their next action based on combination of internal logic + RNG
  - most enemies have "blocks", where the rng range is chopped up into sections, and each section gets an action
  - but at least one enemy does (rng % n) to determine which action to take
    - the provided guide doesn't know what to make of this, but gives a few known values to help the player get started

## Enemies

- About the same
  - Mana
  - Crit chance
  - Options available
    - They can flee, but the chance of selecting is really low (the chance of success is quite high though)
- A little better
  - Dodge
  - Spell fail
- Way higher
  - Damage numbers
  - health
