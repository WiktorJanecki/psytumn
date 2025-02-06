# Psytumn Game
Simple 2D game written in rust without a game engine <br/>  
<img src="preview.gif" width="auto" height="400px" />
## How to play
To open game run `cargo run` having in mind that SDL2 must be installed on your machine  
Move with w/s/a/d and shoot using mouse with LMB

## What I have learned
* How to write programs in more data-driven approach utilizing CPU cache.  <br/><br/>
I've done that using entity component system (ecs) pattern, separating data (components) and game logic (systems) 
 combined with component holders (entities) to iterate on game objects by functionality rather
than by type<br/><br/>
* How to procedurally generate random map using perlin noise and simple rules-based random placing algorithm
* How to profile application, measure how much time each method has used and how to benchmark hot functions
* I become more proficient in rust and its specific patterns (code is from 2022/2023 )
## Technologies used
* HECS - ECS library
* SDL2 - rendering 
* [SDL2_particles](https://github.com/wiktorjanecki/sdl2_particles) - my own particles library
* bracket-noise - perlin noise 
* puffin - profiler 
* glam - linear algebra library
* time, rand, easey - utilities
## License
Copyright (c) 2023 Wiktor Janecki. All rights reserved.

This work is licensed under the terms of the MIT license.  
For a copy, see <https://opensource.org/licenses/MIT>.
