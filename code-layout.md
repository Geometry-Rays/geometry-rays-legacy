# Spaghetti Code Layout
 So while writing the code for geometry rays I failed to make the code readable.

 This document is hopefully gonna help contributors find their way around this mess.

 Also I'll add more details later I hate explaining this code.

# The Mess
 Line 28 to 251 are for loading textures, buttons, and text boxes.

 Line 253 to 411 are for important variables.

 Line 413 to 454 are for objects.

 Line 456 to 584 are for more random stuff to run before the game loop starts idk.

 Line 587 to 610 are variables that are defined every frame.

 Line 613 to 1898 are the logic for the game.

 Line 1901 to 2724 are for the rendering of the game.

 Everything past all that is for saving your level and stars and stuff when exiting the game.

# Notes
 If you need to check if the player has died then use the "kill_player" variable.

 If you need to update the physics then go to src/MenuLogic/playing.rs

 Be careful updating stuff like physics.