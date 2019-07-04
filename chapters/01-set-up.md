# Set-up
By the end of this chapter you are going to have an empty canvas in the browser. That might not sound very exciting but this is just a short chapter to make sure we get everything set up and running. It gets much more exciting after this!

## Make sure you have everything installed
You need the following things installed.
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Cargo web](https://github.com/koute/cargo-web)

## Create your project
run the following command where `ohship` can be your projects name.
```
cargo new ohship
```
Open the newly created folder and it should look like this. 
```
ohship
|
|-src
|  |-main.rs
|
|-Cargo.toml
```
In the `Cargo.toml` add the following under the dependencies section

`Cargo.toml`
```
[dependencies]
quicksilver = "0.3.15"
```
Here we just added the two dependencies we are going to be using for our game. `specs` & `specs-derive` for an ECS and `quicksilver` is our game framework.

Next run cargo build from the command line and it should download and compile the crates for you
```
cargo build
```

## Creating an empty screen
Now to finally start writing some code! Go to your `src/main.rs` and delete everything and we start by adding everything we are going to need.

`src/main.rs`
``` Rust
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Vector},
    graphics::{Color},
    lifecycle::{Settings, State, Window, run},
};
```
Next we are going to set up our game struct. In the same file add this below what we just put

`src/main.rs`
``` Rust
struct Game;

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        Ok(())
    }
}
```
This is basically Quicksilver's game loop. 
- `new`   
    This what we use to intialize our game state. It is required even though it is empty for now. In the future we are going to be adding stuff to it.

    From the quicksilver docs: `new` is the only mandatory function of `State`, which every Quicksilver application must implement. Start all asset loading here, as well as initializing physics worlds or other persistent state.

    Do not attempt to use any Quicksilver features before `new` runs! For example, do not call `Image::load` in your main before you invoke run. Platform-specific setup occurs behind-the-scenes, so just use new for all your initialization.
- `update`   
    We don't have this yet but we are going to add this later. 

    From the quicksilver docs: Update is useful for any fixed-rate calculations or ticks. By default, it is called 60 times per second, and will attempt to make up for any lost time. See this [Gaffer on Games](https://gafferongames.com/post/fix_your_timestep/) blog post for a description of the algorithm. You can change the tick rate with the update_rate setting, which determines how many milliseconds take place between ticks.
- `draw`   
    This is our render event. It is what we use to draw the scene that the user sees. Right now we are just clearing the color and setting it to black. You can also use pick a color via `RGBA`, `Hex`, or from some built in colors like `BLACK`. [Look here for all the options](https://docs.rs/quicksilver/0.3.15/quicksilver/graphics/struct.Color.html)

    From the quicksilver docs: By default, it will run as fast as vsync will allow. You can choose to run it less often, by providing higher values to draw_rate in Settings. After each call to draw, the buffers are flipped (meaning your changes become visible to the user).
- `event`   
    We also don't have this yet either. Here we can set up basic event listeners such as waiting for a key to be pressed and performing some action in response to that.

    From the quicksilver docs: event is called when the events are triggered, either immediately or buffered before the next update. Events can form their own custom lifecycle: for example, listening for an Event::Closed means you can run code to save the game state before the application terminates.

Last we need to start up our game. Add the following at the bottom of the file.

`src/main.rs`
``` Rust
fn main() {
    run::<Game>("Oh! Ship", Vector::new(600, 600), Settings::default());
}
```
This is our main function and we just run the Quicksilver Game here. This is pretty much all we ever have in our main section so we just set it and forget it.

In `run` 
- `<Game>`   
    This our game struct. If you call your struct something else you have to change this
- `"Oh! Ship"`   
    The first string is the title of the game. It is going to appear as the tab title in the browser.
- `Vector::new(600, 600)`   
    This is the starting size of our window/canvas. You can resize while the game is running.
- `Settings::default()`   
    These are Quicksilver settings where you can do stuff like adjust the framerate, hide/show the mouse, and a [number of different things](https://docs.rs/quicksilver/0.3.15/quicksilver/lifecycle/struct.Settings.html).

## Running our App

Once you save your `src/main.rs` you can go to the command line and run
```
cargo web start
```
*Note: It can take a little bit if it is your first time compiling the project. Once you compile it once you just have to recompile the changes so it is much faster. In fact if you save changes while the server is running it well auto recompile for you*

Once it is done compiling it tells you to check out port 8000. If you go to your browser at `localhost:8000` you should see a black canvas!

## Summary
Here we just went through the basics of setting up a new `Quicksilver` project and getting it to compile to WASM so that we could see it in our browser.

If you ever get a little lost or something is not compiling you can [check out the full source code here](https://github.com/DallasC/ohship). Just go to the branch with the same name as the chapter and you'll see the working version.

In the next chapter we start drawing stuff on our new canvas!