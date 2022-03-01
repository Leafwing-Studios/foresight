# Balancing RPG and RNG mechanics

## Constraints

- RPG elements must have some degree of randomness
- RPG tools to mitigate randomness are weak
- RNG effects must either be capped effectiveness or very hard to pull off
- different RPG abilities need to be viable / effective based on upcoming RNG state
  - creates a dynamically shifting palette of usable abilities

## Lessons

- ongoing RNG effects change look-ahead from "next move" to "what will these effects do"
- enemies should be stronger than the player, but does not need to be crazy high disparity
- more information should be handed to the player
  - whether their moves will succeed or fail

## Plan

- keep
  - health
  - mana
  - leveling up
- eliminate attributes
- move to 3 AP (player only)
- 1 AP for enemies
- abilities have:
  - success or fail
    - opposed rolls for hitting and dodging
      - attacker has a hit bonus based on weapon
      - defender has a dodge bonus on their character
    - spell failure
    - saving throws
    - always succeed, but random how *well* you succeed
  - costs
    - AP
    - mana
    - life
  - effect
  - crit chance, if appropriate
- special defenses
- add persistent buffs and debuffs
  - can be operated on as a whole
    - e.g. cleanse
  - use advantage and disadvantage
    - stacks, rolling more
  - regeneration
  - ensure that scaling strategies can exist
  - stacking: extends existing duration
  - statuses trigger at the start of the turn
- UI
  - intro screen
    - type "start" to begin
  - standard help syntax
    - investigate specific actions to get detailed information
