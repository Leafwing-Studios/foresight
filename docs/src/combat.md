# Overview

A classical turn-based fantasy combat system.

## Core stats
- Level: 1-10
- Health: 30+5*level
- Mana: 50 (always and forever)
- AP/turn: 3 (always and forever)
- Special defenses
  - Consitution: 1-40%
  - Dexterity: 1-40%
  - Mind: 1-40%
- Dodge bonus: 10 + level + levelup bonuses (max 20)% ??

## Core actions

- Scan (1AP)
  - 1 RNG
  - Reveal one enemy stat at random
  - Stats revealed are *not* reset on death
  - If all stats have been revealed, then isn't allowed
- Longsword swing (2AP)
  - 4 RNG (hit, dodge, damage, crit)
  - Hit bonus: ??
  - Damage: 10-13
  - Crit chance: 3%

## Core spells
- We should have one

## RNG

- range of 0-255
- current value, previous 10-ish, and next 10-ish values are shown to players
- Additonally, there is a log of what values were used for what actions, which updates as the RNG ticks
  - Ex: "Using ${number} to determine ${actor}'s ${action}"
- higher numbers are generally good for actor
  - attacker rolls for spells/attacks
  - defender rolls for dodge chance
  - then, damage rolled by attacker
- NTH: A value of 255 is always a failure, regardless of actual chance
- order of RNG usages must be very clear
  - "Press enter to advance" after every RNG usage/
- enemies select their next action based on combination of internal logic + RNG
  - most enemies have "blocks", where the rng range is chopped up into sections, and each section gets an action
  - but at least one enemy does (rng % n) to determine which action to take
    - the provided guide doesn't know what to make of this, but gives a few known values to help the player get started
