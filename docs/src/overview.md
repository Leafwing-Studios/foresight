# Overview

## Big Picture

### Elevator Pitch

TAS-style RNG manipulation for RPGs as a game mechanic.

### Target Audience

Indie game lovers who are aware of speedrunning.

### Themes

Unfair advantage.

## Mechanics

### Moment-to-moment Loop

Enter actions in a turn-based battler, which can cause the RNG to advance.

### Game Loop

Fight monster, talk to NPC, repeat.

### Progression Loop

1. Discover RNG manipulation strategies.
2. Compare to notes from GameFAQs.
3. Die.
4. Try again.

### Objects

Overworld:

- monsters
- player
- NPCs

In combat:

- RNG counter
- consumables

### Actions

Over world:

- talk to NPC
  - dialog tree
- enter combat

In combat:

- actions
  - attack
  - defend
- use consumable
- flee

The verbs that define what players can do.

### Resources

- life
- RNG state

### Design Invariants

What patterns and restrictions are we imposing on ourselves to make reasoning about the design easier?

### Design Constraints

What problems will be deal-breakers for players? What restrictions are we imposing on ourselves?

- small-scope
- GameFAQs page

### Design Tolerances

What unusual flaws or unconventional choices will our players accept?

- very hard
- unfair
- fairly opaque mechanics

### Systems and Hooks

- RNG state
  - advanced by X for each player or monster action taken

## Technical

### Technical Strategy

- console-based
- web release
