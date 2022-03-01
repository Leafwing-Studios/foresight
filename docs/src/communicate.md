# Things we have to communicate to the player, pretty early on

- Basic hueristics/mechanics for working with the RNG
  - Higher RNG values are better for the person rolling them (spell fail, hit, dodge, damage, etc.)
  - You can see the number of RNG calls next to each ability
  - You can also see whether one of your abilities will succeed or fail based on the RNG roll it's going to end up using
    - This is somewhat more complicated with opposed rolls and spell saves, since you might not know the enemy's stats. In this case, it doesn't show anything. However, if the player scans the enemy and learns the relevant abilities, then the exact success/failure information is shown
- The help system
  - There is baseline help command that shows at the begininng of the game
  - Any other command (action) can use the `help $ACTION` command to get detailed information about it
  - This chains through help commands: the output of a help command may have other bolded words that the user can `help X` about
  - NTH: You can also call help on the current enemy name, which will give you hints on how it works?
- Progressive disclosure/tutorial
  - Have a few very simple tutorial fights that each introduce a new core mechanic
- What effects are generally associated with the three special defenses?
- How is hitting and such determined?
  - Opposed rolls
  - Spell fail
  - Others