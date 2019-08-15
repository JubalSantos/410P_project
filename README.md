# Mazemania

Copyright (c) Jubal Gonzalez-Santos, Ryan Sanford

Welcome to Mazemania!

# Description
Our project is a simple graphical game where the player navigates some type of maze to the finish line! The game will use graphical output to display the data on screen in a (hopefully) colorful and fun way! The game features four different mazes which will be randomly selected for the user at runtime. Navigate to the exit using either the arrow keys or WASD. if you reach the goal, you win!

# How to Play!
Simply clone this repository to your local machine.
to build and run the program you can use the two commands below

* `cargo build`
* `cargo run` 

when the program is up and running you can use the following keys to move the player in the game to win.

* `WASD or arrow keys`


![alt text](https://github.com/JubalSantos/mazemania/blob/master/src/Screen%20Shot%202019-08-15%20at%2012.40.32%20PM.png "a picture of the game when running")
Here is a rough prototype for a maze:
```
| ----------------------------- |
|  start  |                     |
|   |     |    -----------|     |
|   |        |    |             |
|   ----------    |    |------- |   
|         |            |    end |
| ----|  |------------|    |    |
|                     |    -    |
|     |---------   |       |    |
| ----------------------------- |
```

## License 
This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.
