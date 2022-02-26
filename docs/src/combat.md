# Overview

A classical turn-based fantasy combat system.

## Resources

- life
- mana

Both restored to full after each fight.

## Actions

### Major

- attack
  - heavy
    - single high damage
    - 3 RNG (damage, crit, dodge)
  - light
    - two attacks
    - 5 RNG (damage, damage, crit, crit, dodge)
- cast a spell

### Flee

- utility spells
- dodge
  - O RNG
- flee
  - can be attempted once per turn for free

## Hooks

- damage
- dodge
- spell failure chance
- crits
- health
- mana
- number of times the RNG advances

## Stats

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
- good numbers are generally good for actor
  - attacker rolls for spells
  - defender rolls for dodge chance
  - then, damage rolled by attacker
- order of RNG usages must be very clear
- enemies select their next action based on combination of internal logic + RNG
