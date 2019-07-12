# Oh! Ship
A not quite so simple but still simple game written in Rust.

*Note: This is still a work in progress. Check the roadmap section to see how far along we are.*

## Try it out
All the tutorials are in the `chapters` folder. You can go to the branch with the associated chapter name and it will have just the code up to the end of that chapter. For example if you are on chapter `05-game-state` go to the `game-state` branch.

To run this game locally make sure you have [cargo web](https://github.com/koute/cargo-web) installed. Then just clone this and run `cargo web start`. The finished game is going to be availble online once it is finished.

## Motivation
The Rust game development community is still relatively new and still going through rapid developments. As a result documentation and tutorials are lacking. There are some examples out there but most are extreamly basic and as soon as you want to do anything nontrivial you are on your own.

I hope to help fill this gap and provide a solid example that covers things that you would expect to find in a game made in 2019. Some things that I plan to inclue are

- Compiling to WASM   
It is 2019 and you'll find that most people don't want to download and install a small simple game. It is much easier to get people to try your game if all you have to do is open a webpage. 
- Nontrivial game features   
    I'm not going to be making a complex game by any means but I want to go more in depth than a basic tetris, snake, space invader type example. I want to include how to make menus, moving camera that follows your character, basic gameplay components, animations, and so on
- Multiplayer   
    It's a lot more fun to play games with other people. I want to show you how to add basic multiplayer functionality to your game via websockets. Don't expect to have 1000 in the same lobby with this method but you should easily be able have 10 people playing together.

## Roadmap
**Game**
- [x] Set-up
- [x] Drawing
- [x] Input
- [x] Assets
- [x] Game-state
- [x] Camera
- [x] Tilemap
- [ ] Collision-detection
- [ ] Animation
- [ ] Menu
- [ ] \(Optional) Bot-AI
- [ ] \(Optional) Additional-gameplay-features

**Multiplayer**
- [ ] Server-setup
- [ ] Websockets
- [ ] Client-prediction
- [ ] Server-reconciliation
- [ ] Entity-interpolation
- [ ] Lag-compensation
- [ ] Game-lobby
- [ ] \(Optional) Chat

## Technologies
- [Quicksilver](https://github.com/ryanisaacg/quicksilver) 
    This used as the game engine. As of writing this it is the only engine that can compile to WASM so hats off to them.
- [Actix-web](https://github.com/actix/actix-web)
    This is used as our web server for multiplayer support. Actix is a full featured web framework but we are mostly just focusing on it's websocket features.
