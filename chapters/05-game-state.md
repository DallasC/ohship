# Game State
You are going to learn how to have different game states for your game. We are going to use these game states in order to have a loading screen while we wait for our assets to load. Also, since our code is starting to get a bit long for 1 file we are going to start splitting it up into separate files.

This is the longest chapter so far. About 4x as long as the other ones. It goes by pretty fast though and by the end of the chapter our code is organized way better, which is going to make things a lot easier going forward.

## Refactoring 
Let's start by splitting our code up now before we add our game states. There are a number of ways to split up our code. For now we just going to make a new file called `components.rs` in the `src` folder.

`src/components`
``` Rust
use quicksilver::{
    geom::{Rectangle}
};

pub struct Player {
    pub body: Rectangle,
    pub angle: f32,
    pub z: i8,
}

impl Player {
    pub fn new() -> Player {
        let body = Rectangle::new((277, 243), (66, 113));
        let angle = 180.;
        let z = 2;
        Player {body, angle, z}
    }
}
```
As you can see we just moved our player struct over to our new file and made it public so we can access it from other files. 

Next in our `main.rs` file we just need to add our new file to our imports and get rid of our old player struct.

`src/main.rs`
``` Rust
extern crate quicksilver;

mod components;
use components::Player;

use quicksilver::{
    Result,
    geom::{Vector, Transform},
    graphics::{Color, Background, Atlas},
    input::{Key},
    lifecycle::{Asset, Settings, State, Window, run},
};

struct Game {
    player: Player,
    atlas:  Asset<Atlas>
}
...
```
Now your folder structure should look something like this

```
ohship
|
|-src
|  |-main.rs
|  |-components.rs
|-static
|  |-spritesheet.txt
|  |-spritesheet.txt
|-Cargo.toml
```
You might have a couple other folders/files that rust automatically generates when you are compiling like a `target` folder and a `Cargo.lock`. Now if you run `cargo web start` everything should compile like we expect it to.

## Game States
Now that we got that out of the way lets move on to setting up our game state. First make a new file in `src` called `state.rs` and we are gonig to start with all of our imports

`src/state.rs`
``` Rust
use std::{
    rc::Rc, cell::RefCell,
};
use quicksilver::{
    Result, lifecycle::{Event, Window},
};

// Import states
pub mod loading;
pub mod play;

use crate::{
    GameAssets, state::{
        loading::LoadingState, play::PlayState,
    }
};
```
We haven't made `GameAssets`, `LoadingState`, or `PlayState` yet but are going to get to that later. We are just going to import them so that we don't have to come back here later.Now that we got that out of the way lets start by defining a couple enums.

`src/state.rs`
``` Rust
pub enum CurrentState {
    Loading,
    Running,
}

pub enum StateTransition {
    NoTransition,
    StateLessTransition(CurrentState),
}
```
Here we have `CurrentState` which is going to list all of our states. Right now we only have two states `Loading` and `Running`. Later any new states that we might want like pause, menu, winner/loser screen, etc go here. 

There is also `StateTransition`. This is how we switch between different states. Right now we have `NoTransition` which means to keep the same state. We also have `StateLessTransition(CurrentState)`. This transition takes a state from our CurrentState and switches to it. It is called `stateless` because we just switch states but don't pass along any information from the previous state. Later we are going to add `StateTransitions` that allow us to share data between states.

Next we are going to set up our `StateManager`

`src/state.rs`
``` Rust
pub struct StateManager {
    pub loading: Rc<RefCell<LoadingState>>,
    pub play: Rc<RefCell<GameState>>,
    pub current_state: CurrentState,
}
```
First we define our two states `loading` and `play`. Later when we add new states to our game we also have to add them here as well as in the `CurrentState` enum. Lastly we have our `CurrentState` enum.

You might be wonder what `Rc<RefCell<LoadingState>>` is all about especially  if you are new to rust. `LoadingState` and `GameState` we haven't definded yet so don't worry about those yet. I recommend reading chapter 15 in the [rust book](https://doc.rust-lang.org/beta/book/ch15-00-smart-pointers.html) as it does a great job explaining `Rc`, `RefCell`, and other smart pointers.  Basically we are using these as `Rc` allows us to have multiple owners of the same data. This way we don't delete a state that is still in use. `RefCell` allows us to mutate our state because the borrows are checked at runtime instead of compile time.

Next we are going to give our StateManager a couple basic functions.

`src/state.rs`
``` Rust
impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            loading : Rc::new(RefCell::new(LoadingState::new())),
            play: Rc::new(RefCell::new(PlayState::new())),
            current_state : CurrentState::Loading,
        }
    }

    pub fn current_state(&mut self) -> Rc<RefCell<GameState>> {
        match self.current_state {
            CurrentState::Loading => self.loading.clone(),
            CurrentState::Playing => self.play.clone(),
        }
    }

    pub fn transition_state(&mut self, transition : StateTransition) {
        match transition {
            StateTransition::NoTransition => (),
            StateTransition::StateLessTransition(state) =>  {
                self.current_state = state;
            },
        }
    }
}
```
We give `StateManager` 3 functions. First is `new()` so that we can start up our state manager. `current_state()` gives us a copy of our current state. `transition_state(StateTransition)` allows us to switch states using te supplied `StateTransition`

Lastly we just have one trait to implement.

`src/state.rs` 
``` Rust
pub trait GameState {
    fn update(&mut self, window: &mut Window, assets: &mut GameAssets) -> StateTransition;

    fn draw(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()>;

    fn event(&mut self, event: &Event, _window: &mut Window) -> StateTransition;
}
```
For each of our states we are going to be implementing this trait. We have `update`, `draw`, `event`. You might be thinking this sounds familiar. Thats because it mimics Quicksilvers `update`, `draw`, `event` function. What this means is that each of our states is going to have it's own set of these functions.

## Updating Our Quicksilver Loop
Now lets go back over to our `main.rs` and set it up to use our new `StateManager`. Delete **everything** and copy the following imports for now

`src/main.rs`
``` Rust
extern crate quicksilver;

mod components;
mod state;

use state::StateManager;

use quicksilver::{
    Result, geom::Vector, graphics::Atlas,
    lifecycle::{Asset, Settings, State, Window, Event, run},
};
```
Next we are going make `GameAssets` that we mentioned earlier.

`src/main.rs`
``` Rust
pub struct GameAssets {
    pub atlas:  Asset<Atlas>,
}

impl GameAssets {
    pub fn new() -> GameAssets {
        GameAssets {
            atlas: Asset::new(Atlas::load("spritesheet.txt")),
        }
    }
}
```
Here we just have a new struct containing the atlas that we had before. We also have a helper function `new()` in order to create the `GameAssets` struct. Right now we only have one asset but in the future you might have many more assets like sounds, and some more pictures. This provides a convient place to put them.

Next we are going to create our old `Game` struct.

`src/main.rs`
``` Rust
struct Game {
    state: StateManager,
    assets:  GameAssets,
}
```
This didn't change much we basically traded `player` for our new `state` and our old asset for our new `GameAssets` struct. This might not look like a big change but it simplifies our `Game` a lot. After we finish `impl State for Game` we'll never have to touch our `Game` struct again! In fact we won't have to touch our `main.rs` again unless we want to add additional assets to the `GameAssets` struct.

Lets do our `impl State` now:

`src/main.rs`
``` Rust
impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            state: StateManager::new(),
            assets: GameAssets::new(),
        })
    }
    
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let current_state_ref = self.state.current_state();
        let transition = {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.update(window, &mut self.atlas)
        };
        self.state.transition_state(transition);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let current_state_ref = self.state.current_state();
        {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.draw(window, &mut self.atlas)
        }
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        let current_state_ref = self.state.current_state();
        let transition = {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.event(event, window)
        };
        self.state.transition_state(transition);
        Ok(())
    }

}
```
Here we have our `new`, `update`, `draw`, and `event` functions from Quicksilver. It looks a little complicated but what each one is doing is basically the same except for new which just initializes our game. The others each get the current state, perform the associated function for that state, and then transtion states if needed.

This handles all of the game state management for us so now we just need to write the `update`, `draw`, and `event` functions for each state and we never have to worry about this stuff again!

Lastly we just add our `main()` function that is the same as before

`src/main.rs`
``` Rust
fn main() {
    run::<Game>("Oh! Ship", Vector::new(600, 600), Settings::default());
}
```
Now we are all done with our `main.rs` and we pretty much never have to touch this file again. The only exceptions are if we want to add more assets or need to change the Quicksilver `Settings` mentioned in chapter 1.

## Play State
Next lets set up our `PlayState` or the state that we use when we are playing the game. This State does exactly what we had before, renders a ship on the screen that we can move around.

First we are going to make a new folder in `src` called `state`. Then in that new folder we make a file called `play.rs`. Lets start by adding our imports like usual.

`src/state/play.rs`
``` Rust
use quicksilver::{
    Result, geom::{Transform},
    graphics::{Color, Background},
    input::{Key}, lifecycle::{Event, Window},
};
use crate::{
    state::{StateTransition, GameState},
    components::Player, GameAssets,
};
```
Next lets make our `PlayState`

`src/state/play.rs`
``` Rust
pub struct PlayState {
    player: Player
}

impl PlayState {
    pub fn new()  -> PlayState {
        PlayState {
            player: Player::new(),
        }
    }
}
```
Each of our states files is going to have us define anything that we want to use in our state and create a `fn new()` in order to start our initial state in the `StateManager`.

Next lets move on to implementing our `update`, `draw`, and `event` functions.

`src/state/play.rs`
``` Rust
impl GameState for PlayState {

    fn update(&mut self, window: &mut Window, _assets: &mut GameAssets) -> StateTransition {
        if window.keyboard()[Key::Up].is_down() ||  window.keyboard()[Key::W].is_down() { 
            // From degrees to radians
            let angle = self.player.angle * std::f32::consts::PI / 180f32 ; 
            let dx = angle.sin() * -4.;
            let dy = angle.cos() * 4.;
            self.player.body.pos.x = self.player.body.pos.x + dx;
            self.player.body.pos.y = self.player.body.pos.y + dy;
        }  
        if window.keyboard()[Key::Left].is_down() ||  window.keyboard()[Key::A].is_down() {
            self.player.angle = (self.player.angle - 2.) % 360.;
        } 
        if window.keyboard()[Key::Right].is_down() ||  window.keyboard()[Key::D].is_down() {
            self.player.angle = (self.player.angle + 2.) % 360.;
        }
        StateTransition::NoTransition
    }

    fn draw(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()> {
        window.clear(Color::BLACK)?;
        let body = &self.player.body;
        let angle = self.player.angle;
        let z = self.player.z;
        assets.atlas.execute(|atlas| {
            let texture = &atlas.get("redShip1").unwrap().unwrap_image();
            window.draw_ex(
                body,
                Background::Img(texture),
                Transform::rotate(angle),
                z
            );
            Ok(())
        })
    }

    fn event(&mut self, _event: &Event, _window: &mut Window) -> StateTransition {
       StateTransition::NoTransition
    }
}
```
This looks like we did a lot! If you take a closer look though you'll notice this is almost exactly what we had before except that we return a `StateTransition` for `update` and `event`. The reason that it is the same is because this state does the exact same thing that we were doing before.

## Loading State
Next lets set up our `LoadingState`. Remember that annoying wait period where we just had a black screen while we waited for assets to render? Now instead of that we are going to display this loading state while we wait for the assets to render like you would expect from a game.

Make a new file called `loading.rs` in the `state` folder and lets start by adding our imports like usual.

`src/state/loading.rs`
``` Rust
use quicksilver::{
    Result, geom::{Circle, Rectangle},
    graphics::{Color, Background},
    lifecycle::{Event, Window},
};
use crate::{
    state::{StateTransition, CurrentState, GameState},
    GameAssets
};
```
Next like with any of our states we start by defining the state and it's `fn new()`.

`src/state/loading.rs`
``` Rust
pub struct LoadingState;

impl LoadingState{
    pub fn new() -> LoadingState {
        LoadingState
    }
}
```
Our loading state doesn't need to have any data. We are just going keep it simple and draw stuff on the screen.

Now for the meat of our State. The `update`, `draw`, and `event` functions.

`src/state/loading.rs`
``` Rust
impl GameState for LoadingState {
    fn update(&mut self, _window: &mut Window, assets: &mut GameAssets) -> StateTransition {
        let mut transition = StateTransition::NoTransition;
        let _x = assets.atlas.execute(|_| {
            transition = StateTransition::StateLessTransition(CurrentState::Playing);
            Ok(())
        });
        transition
    }
    ...
```
Here we have the `update` function. It checks to see if the asset is loaded using `asset.execute()`. If the assets are loaded then we set the `StateTranstion` to transtion to the `Playing` state. If it is not loaded we stay in our current state.

Next the draw function

`src/state/loading.rs`
``` Rust
    ...
    fn draw(&mut self, window: &mut Window, _assets: &mut GameAssets) -> Result<()> {
        window.clear(Color::BLACK)?;
        // L
        window.draw(&Rectangle::new((100,250), (10,100)), Background::Col(Color::WHITE));
        window.draw(&Rectangle::new((110,340), (50,10)), Background::Col(Color::WHITE));
        // o
        window.draw(&Circle::new((190,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((190,325), 17.0), Background::Col(Color::BLACK));
        // a
        window.draw(&Circle::new((245,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((245,325), 17.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((262,300), (10,50)), Background::Col(Color::WHITE));
        // d
        window.draw(&Circle::new((302,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((302,325), 17.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((319,250), (10,100)), Background::Col(Color::WHITE));
        // i
        window.draw(&Rectangle::new((334,305), (10,45)), Background::Col(Color::WHITE));
        window.draw(&Circle::new((339,280), 6.0), Background::Col(Color::WHITE));
        // n
        window.draw(&Rectangle::new((349,325), (10,25)), Background::Col(Color::WHITE));
        window.draw(&Rectangle::new((379,325), (10,25)), Background::Col(Color::WHITE));
        window.draw(&Circle::new((369,325), 20.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((369,325), 10.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((359,325), (20,25)), Background::Col(Color::BLACK));
        // g
        window.draw(&Circle::new((419,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((419,325), 17.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((436,300), (10,75)), Background::Col(Color::WHITE));
        window.draw(&Circle::new((421,376), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((421,376), 15.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((396,350), (40,25)), Background::Col(Color::BLACK));
        // ...
        window.draw(&Circle::new((456,345), 5.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((471,345), 5.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((486,345), 5.0), Background::Col(Color::WHITE));
        Ok(())
    }
    ...
```
Ok this is kind of long. Whats going on here? Well I used circles and rectangles to write "Loading..." in the middle of the screen. Is this the best use of your time? How bored were you? Do you regret it? These are all excellent questions that I'm not going to answer. 

Is this the best way to do a loading screen? No probably not. You would probably be better off converting a samll image or font to bytes and having your binary be a bit bigger but be able to use the image for the loading screen. You could also do something like a loading bar if you wanted. Does this work though? Yes, this works fine too if you have some extra time to mess around with drawing things on the screen. It actually didn't take very long to put together.

Lastly we have our `update()`. There is not much going on here since we aren't listening for any events yet.

`src/state/loading.rs`
``` Rust
    ...
    fn event(&mut self, _event: &Event, _window: &mut Window) -> StateTransition {
       StateTransition::NoTransition
    }
}
```
You can see the changes we made by running `cargo web start` again.
## Summary
Wow! We made a ton of changes this time. We seperated our code base and added an entire state management system. This was by far our longest chapter yet so good job at making it all the way here!! This was also the first non-trivial piece of code we went over in this guide. The past chapters you could have figured out by looking at the docs/examples and piecing it together yourself. One of the big goals in writing this is to go over non-trivial stuff when it comes to making games. The Rust game dev scene is still rather small and young even though it has some big goals. One thing this means is that there is a lack of material on how to do things beyond the basics of a single screen games like pong, snake, or astroids. I hope to fill this gap so we can start seeing some more creative games that are made using Rust!

It might seem like we added some boilerplate with the state management system but it is going to save us a lot of time in the future! It gives us a clear division in our codebase of where to put things and how things interact with eachother. This means it is way easier for us organize and reason about our code. Also, now that we have it out of the way we can pretty much forget about it and just focus on the specific states that we need.

A lot changed this chapter but you can [check out the full source code here](https://github.com/DallasC/ohship) if you missed anything. Just go to the branch with the same name as the chapter.

Next chapter we are going to be adding a camera that follows our ship around. You know how currently if you go over past the edge of the screen you can't see your ship anymore even though it's still there? We are going to fix that by have a camera that always keeps the ship in the center of the screen. This is pretty easy so it is going to be a quick short chapter.