# Tile Map
This is going to be another long chapter. In this chapter we are finally going to get rid of our boring background and use tiles to build a game map. 

We are going to cover some in depth details of parsing and loading an external tile map file that I think a lot of tutorials would skip over and instead just hardcode the map. Loading an external file gives you the ability to build your maps/levels in map editors and makes it super easy to add multiple levels to your game. It is also much more inline with how games are actually developed.

In this chapter we cover loading external files, parsing the file, loading multiple assets with our state management system. and drawing the tile map.

## Tile Map
You can use any tile map editor and set tiles that you want for this but you might have to change some of the parsing code to account for the differences in file format. If you are following along you can grab the `level.txt` file from the `static` folder. 

This contains a simple level I made using a tile map editor. The textures used for this tilemap are already loaded as part of the spritesheet. You can have your tiles on a seprate spritesheet if you want. For loading multiple sprite sheets the code is exacttly the same as loading the first one. You just have to copy and paste and change the name.

The output map file I made looks like:

```
size: 20, 20
overlap: 4, 4
tile_size: 64, 64
tile: outsideCorner, 270, 3, false
...
```
The format is pretty basic. 
1. size: (The number of tiles in a row), (The number of tiles in a column)
2. overlap: The amount that the tiles overlap eachother in px.   
    Note: I added this myself as a temporary workaround. When rendering it with no overlap you could see a thin line between the tiles when moving around. I couldn't pinpoint what was causing this issue but this worked for now while I opened an issue with quicksilver to pinpoint the problem. I will update this once everything is figured out.
3. tile_size: (width of tile), (height of tile)
4. tile: (name of the texture), (rotation in degrees), (z value), (if you can travel across the tile)    
    There are a bunch of these (20x20) but they all follow the same format. Also, we don't use the last boolean yet. It is used in the next chapter for collision detection.

The format isn't very imporant so you can use whatever tile editor you want. All you have to do is update you `Map` struct to reflect the values of your tile and adjust how you parse the file so that it works with your format. After you finish reading this chapter this should be easy for you any kind of format you want.

## Building Our Tile Map
Quicksilver comes with a built in `TileMap` and `Tile` but we aren't going to use these. `TileMap` does have a couple convenient functions built in but for the most part I found it clunky to work with when building and rendering your map. Since we have to make our own `file_load` function anyways it was just a lot easier to create our own `Map` struct. 

We are going to make a new file in `src` called `map.rs` to hold our new `Map` struct. Lets start with our imports:

`src/map.rs`
``` Rust
use crate::GameAssets;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::Background,
    lifecycle::{Asset, Window},
    load_file, Future, Result,
};
use std::path::Path;
```
Next we are going to to set up our `Tile` struct.

`src/map.rs`
``` Rust
pub struct Tile {
    shape: Rectangle, 
    texture: String,
    rotation: f32,
    z: i8,
    travel: bool,
}

impl Tile {
    fn new(
        position: Vector,
        tile_size: Vector,
        rotation: f32,
        texture: String,
        z: i8,
        travel: bool,
    ) -> Tile {
        let shape = Rectangle::new(position, tile_size);
        Tile {
            shape,
            texture,
            rotation,
            z,
            travel,
        }
    }
}
```
Here we have our basic tile and a helper function to generate new tiles. The tiles themselves are pretty basic and you can add/delete things that you want.
- shape: position and size of tile
- texture: which texture to use for the tile
- rotation: which angle to rotate the tile if needed
- z: the z index of the tile
- travel: `true` if you can move across this tile. `false` if you can't move across this tile.

Next lets make our `Map` struct.

`src/map.rs`
``` Rust
pub struct Map {
    tiles: Vec<Tile>,
    size: Vector, // in number of tiles
    overlap: Vector, // amount of px that tiles overlap
    tile_size: Vector,
}
```
Here we have our map struct. Most of these are self explanitory:
- tiles: `Vec` to hold our tiles
- size: How many tiles are in our map for x and y
- overlap: The amount of px we want our tiles to overlap for x and y
- tile_size: The size of our tiles in px

Next we are going to make our new and load functions

`src/map.rs`
``` Rust
impl Map {
    pub fn new() -> Map {
        Map {
            tiles: Vec::new(),
            size: Vector::new(0, 0),
            overlap: Vector::new(0, 0),
            tile_size: Vector::new(0, 0),
        }
    }
    pub fn load(path: impl AsRef<Path> + 'static) -> Asset<String> {
        Asset::new(
            load_file(path).and_then(|contents| {
                Ok(String::from_utf8(contents).expect("The file must be UTF-8"))
            }),
        )
    }
}
```
Our new function just makes a completely empty map for us. If we try to draw this map nothing is going to fail but it won't draw anything. Our load function takes creates a new `Asset` for us. If you remember Quicksilver uses Assets as futures so that loading is nonblocking on the web. In this case our `level.txt` file is getting loaded as a `String`.

Now that we have our text file loaded as a `String` we need to parse it into something usable. Below our `fn load` we are going to add a new function that converts the `String` to our `Map` struct.

`src/map.rs`
``` Rust
    ...
    pub fn from_string(data: &mut String) -> Map {
        let mut lines = data.lines();

        let size = get_vec(lines.next().unwrap());
        let overlap = get_vec(lines.next().unwrap());
        let tile_size = get_vec(lines.next().unwrap());
        let mut tiles = Vec::with_capacity((size.x * size.y) as usize);

        for y in 0..size.y as usize {
            for x in 0..size.x as usize {
                let position = Vector::new(
                    x as f32 * (tile_size.x - overlap.x),
                    y as f32 * (tile_size.y - overlap.y),
                );
                let tile = build_tile(lines.next().unwrap(), position, tile_size);
                tiles.push(tile);
            }
        }

        Map {
            tiles,
            size,
            overlap,
            tile_size,
        }
    }
    ...
```
A lot of stuff is happening here so lets break it down. In the first line we take our `String` and make it into an iterator by calling `.lines()`. This allows us to use `.next().unwrap` to get a single line from our file.

Next we take the first three lines of our file and convert them into `Vectors` to be used to make our new `Map`. The first three lines of our file are:
```
size: 20, 20
overlap: 4, 4
tile_size: 64, 64
```
And we call the same function on each line:
``` Rust
let size = get_vec(lines.next().unwrap());
let overlap = get_vec(lines.next().unwrap());
let tile_size = get_vec(lines.next().unwrap());
```
As you can see we take a single line from our file and cal `get_vec` with it. Lets make `fn get_vec` now. Under `impl Map {}` at the very bottom of the file add the following:

`src/map.rs`
``` Rust
fn get_vec(line: &str) -> Vector {
    let mut xy = line.split(": ").last().unwrap().split(", ");
    let x = xy.next().unwrap().parse::<f32>().unwrap();
    let y = xy.next().unwrap().parse::<f32>().unwrap();
    Vector::new(x, y)
}
```
Here you can see we take a `line` from our file and output a `Vector`. For the first line I am going to use the first line from our text file to demonstrate what is happening. Our first line looks like `size: 20, 20`. Then on the first line fro `let mut xy =` we do the folowing.

First we take our line and `split(": ")`. This gives us an iterator with `["size", "20, 20"]`. Next we take the last part of that iterator (`"20, 20`) and use `.split(", ")` giving us a new iterator `["20", "20"]`.

For the next to line we call `.next()` which gives us the first part of our new iterator (in this case `"20"`) and `.parse::<f32>()` which converts a `&str` to whichever type is in the turbo fish (turbo fish is what we call the `::<>`). At the end of this it means that we have `let x = 20 as f32`. We do the same thing for `y` and then we make a new Vector that we return.

We do this for the first 3 lines of our `levels.txt`. Back up to our `pub fn from_string(data: &mut String) -> Map`. After we get our `size`, `overlap`, `tile_size` Vectors we make a new `Vec` to store our tiles in

``` Rust
let size = get_vec(lines.next().unwrap());
let overlap = get_vec(lines.next().unwrap());
let tile_size = get_vec(lines.next().unwrap());
let mut tiles = Vec::with_capacity((size.x * size.y) as usize);
```
Next we build our `Vec`:
``` Rust
for y in 0..size.y as usize {
    for x in 0..size.x as usize {
        let position = Vector::new(
            x as f32 * (tile_size.x - overlap.x),
            y as f32 * (tile_size.y - overlap.y),
        );
        let tile = build_tile(lines.next().unwrap(), position, tile_size);
        tiles.push(tile);
    }
}
```
This looks confusing but makes sense once you work through it. The `for` loops just means that we are building our `Map` Left to Right, Top to bottom. If we had a 3x3 tile map it would look like:
```
1 2 3
4 5 6
7 8 9
```
For each of these tiles we then calculate the position in the game world. Next we call `build_tile()` to create a `Tile` then store it in our `Vec<Tile>` by calling `tiles.push(tile)`. This is all pretty straight forward except for `build_tile()` that we haven't created yet. So let's do that. At the bottom of the file under `fn get_vec()` add the following

``` Rust
fn build_tile(line: &str, pos: Vector, tsize: Vector) -> Tile {
    let mut val = line.split(": ").last().unwrap().split(", ");
    let texture = val.next().unwrap().parse::<String>().unwrap();
    let angle = val.next().unwrap().parse::<f32>().unwrap();
    let z = val.next().unwrap().parse::<i8>().unwrap();
    let travel = val.next().unwrap().parse::<bool>().unwrap();
    Tile::new(pos, tsize, angle, texture, z, travel)
}
```
This basically the same as what we did in `get_vec()` but the line we are parsing looks like `tile: outsideCorner, 270, 3, false` instead of `size: 20, 20`.

After we parse the line we use our `Tile::new()` that we created earlier to create our `Tile` that we return and put in our `Vec<Tile>`. 

That looked really complicated but once we broke it down we were just parsing our map file like we would for any file. In fact if you didn't know how to parse log files, csv, or other types of files in `Rust` after understanding how we parsed our `level.txt` you should now be to parse all these files too!

## Drawing our Map
Now that we have our map file converted to our `Map` struct we need to be able draw our map on the screen. Inside our `impl Map` add the following function:

`src/map.rs`
``` Rust
    pub fn draw_map(
        &mut self,
        view: &Rectangle,
        window: &mut Window,
        assets: &mut GameAssets,
    ) -> Result<()> {
        // figure out what tiles to drawing
        let x_start = (view.pos.x / (self.tile_size.x - self.overlap.x)).floor() as usize;
        let y_start = (view.pos.y / (self.tile_size.y - self.overlap.y)).floor() as usize;
        // figure out how many tiles to draw per row/colum
        let mut x_end =
            (view.size.x / (self.tile_size.x - self.overlap.x)).ceil() as usize + x_start + 1;
        let mut y_end =
            (view.size.y / (self.tile_size.y - self.overlap.y)).ceil() as usize + y_start + 1;

        if x_start + x_end > self.size.x as usize {
            // If on the right edge of the screen get rid of the right buffer
            x_end = self.size.x as usize;
        }

        if y_start + y_end > self.size.y as usize {
            // If on the bottom edge of the screen get rid of the bottom buffer
            y_end = self.size.y as usize;
        }

        assets.atlas.execute(|atlas| {
            for y in y_start..y_end {
                for x in x_start..x_end {
                    let tile = &self.tiles[(y as f32 * self.size.y + x as f32) as usize];
                    let texture = &tile.texture;
                    let img = &atlas.get(texture).unwrap().unwrap_image();
                    let rect = img.area().with_center(tile.shape.center());
                    window.draw_ex(
                        &rect,
                        Background::Img(&img),
                        Transform::rotate(tile.rotation),
                        tile.z,
                    );
                }
            }
            Ok(())
        })
    }
```
This is another long one so lets break it down again. The reason it is so long is because we are only drawing the part of the map that is in the view area. We could just draw the entire map at once and be done with it but this is really inefficent the bigger the map gets and wastes a lot of resources on things that the user doesn't even see. 

First we have 
``` Rust
// figure out what tiles to start drawing from
let x_start = (view.pos.x / (self.tile_size.x - self.overlap.x)).floor() as usize;
let y_start = (view.pos.y / (self.tile_size.y - self.overlap.y)).floor() as usize;
// figure out how many tiles to draw per row/colum
let mut x_end =
    (view.size.x / (self.tile_size.x - self.overlap.x)).ceil() as usize + x_start + 1;
let mut y_end =
    (view.size.y / (self.tile_size.y - self.overlap.y)).ceil() as usize + y_start + 1;

if x_start + x_end > self.size.x as usize {
    x_end = self.size.x as usize;
}

if y_start + y_end > self.size.y as usize {
    y_end = self.size.y as usize;
}
```
This gives us the starting point of our drawing and lets us know how many tiles we need to draw in order to cover the screen. You don't have to worry about this sinze it automatically adjusts based on tile size, screen size, overlap, and the camera size. If you don't need an overlap you can just get rid of it.

Once we know where to start drawing our tiles next we need to start actually drawing them.
``` Rust
assets.atlas.execute(|atlas| {
    for y in y_start..y_end {
        for x in x_start..x_end {
            let tile = &self.tiles[(y as f32 * self.size.y + x as f32) as usize];
            let texture = &tile.texture;
            let img = &atlas.get(texture).unwrap().unwrap_image();
            let rect = img.area().with_center(tile.shape.center());
            window.draw_ex(
                &rect,
                Background::Img(&img),
                Transform::rotate(tile.rotation),
                tile.z,
            );
        }
    }
    Ok(())
})
```
First we have `atlas.execute()` like when we are drawing our `Player`. This is so that we apply a texture to our tile from our texture atlas. Next we have a couple `for` loops. This is similar to when we were building our `Map` but instead of going over our entire map we just go to the tiles that we want to use.

To get the tile that we want we use `let tile = &self.tiles[(y as f32 * self.size.y + x as f32) as usize];`. This gets the tile we want from our `Vec<tile>`. After we have our tile we get the texture that want use from our `atlas`. Lastly we build our tile as a `Rectangle` with the background being our texture. 

By having rotation availble to the tile can save us some space on our texture atlas. Instead of have 4 copies of the same tile but facing different directions we can just rotate the tile by 90, 180, or 270 degrees instead. 

Thats all we need to draw our tile map on the screen. As a bonus we aren't wasting resources drawing the entire map each cycle instead we just draw what the user can see. Now all we need to do is hook up our new tile map to our state management system and we are good to go.

## Adding Additional GameAssets
Before we set up our game to load our sprite sheet. Now that we have added an additional asset, our `level.txt`. To add another Asset we first have to add it in `main.rs`. Add the following imports at the top of the file:

`src/main.rs`
``` Rust
mod map;
use map::Map;
```
Next we just need to add our new asset in.

`src/main.rs`
``` Rust
pub struct GameAssets {
    pub atlas:  Asset<Atlas>,
    pub map: Asset<String>,
}

impl GameAssets {
    pub fn new() -> GameAssets {
        GameAssets {
            atlas: Asset::new(Atlas::load("spritesheet.txt")),
            map: Map::load("level.txt"),
        }
    }
}
```
All we did was add `pub map: Asset<String>,` and load it with `map: Map::load("level.txt"),`

Now that we have it in our `GameAssets` we can take addvantage of it with our `StateManager`. Go to `src/state.rs` and add our new map import

`src/state.rs`
``` Rust
use crate::{
    GameAssets, state::{
        loading::LoadingState, play::PlayState,
    }, map::Map
};
```
Next we are going to be adding a new transition since we want to be able to pass our map to the the play game state once it is done loading.

`src/state.rs`
``` Rust
pub enum StateTransition {
    NoTransition,
    StateLessTransition(CurrentState),
    StartGameTransition(Map),
}
```
As you can see we added a new transition called `StartGameTransition(Map)`.

Lastly we just have to add our new transtion to `transition_state()`. It should look like this now:

`src/state.rs`
``` Rust
pub fn transition_state(&mut self, transition : StateTransition) {
    match transition {
        StateTransition::NoTransition => (),
        StateTransition::StateLessTransition(state) =>  {
            self.current_state = state;
        },
        StateTransition::StartGameTransition(map) =>  {
            let mut play_state_mut = self.play.borrow_mut();
            play_state_mut.update_map(map);
            self.current_state = CurrentState::Playing;
        }
    }
}
```
Here we added the new transition called `StartGameTransition`. As you can see we take our play_state then update our default empty map to instead use the `Map` that we pas it. Lastly we set our new state to `Playing`.

## Updating States
Now we just need need to update our `loading` and `play` states to take advantage of our new changes and then we are good to go. Let's first update our loading state to load our new `level.txt` asset and use our new transtion. 

`src/state/loading.rs`
``` Rust
use crate::{
    state::{StateTransition, GameState},
    GameAssets, map::Map,
};
```
We import our new `Map` struct so that we can load our map during the loading screen.

Next we update our `update` function to load both our spritesheet and new map before using our new transtion to pass the `Map` and transition to our `PlayState`.

`src/state/loading.rs`
``` Rust
#[allow(unused_must_use)]
impl GameState for LoadingState {
    fn update(&mut self, _window: &mut Window, assets: &mut GameAssets) -> StateTransition {
        let mut transition = StateTransition::NoTransition;
        let mut textures = false;
        assets.atlas.execute(|_| {
            textures = true;
            Ok(())
        });
        assets.map.execute(|data| {
            let new_map = Map::from_string(data);
            if textures {
                transition = StateTransition::StartGameTransition(new_map);
            }
            Ok(())
        });
        transition
    }
    ...
```
First you'll notice we added `#[allow(unused_must_use)]`. This is because we don't use the `result` from `assets.execute()`. There isn't a good way to bubble these errors up through our state management system. Doing this would require integrating our system directly into the `Quicksilver` crate since the quicksilver error is private. Before we just hid the errors behind an unused variable like `let _x =` but using `#[allow(unused_must_use)]` is the idomatic way to do this. 

In reality this doesn't cause much of an issue. The only thing to keep in mind is if you get stuck at the LOADING screen, your issue comes from here. Most likely it had to do with `Map::from_string()` not parsing your map file correctly.

Moving on we added:
``` Rust
let mut textures = false;
assets.atlas.execute(|_| {
    textures = true;
    Ok(())
});
```
What this does is once our spritesheet is done loading textures becomes `true`. Next we added:
``` Rust
assets.map.execute(|data| {
    let new_map = Map::from_string(data);
    if textures {
        transition = StateTransition::StartGameTransition(new_map);
    }
    Ok(())
});
```
This loads our map from our `levels.txt` file. Once it is loaded we create our map and if our spritesheet is done loading change our transition to `StartGameTransition` and pass it the map we just loaded.

Now all we have left to do is update our `play` state to draw our new map and we are good to done!

`src/state/play.rs`
``` Rust
use crate::{
    components::Player,
    map::Map,
    state::{GameState, StateTransition},
    GameAssets,
};
use quicksilver::{
    geom::{Rectangle, Shape},
    graphics::{Color, View},
    input::Key,
    lifecycle::{Event, Window},
    Result,
};
```
We added `map::Map` to our imports. Now lets update our `PlayState` to use our new map.

`src/state/play.rs`
``` Rust
pub struct PlayState {
    player: Player,
    view: Rectangle,
    map: Map,
}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {
            player: Player::new(),
            view: Rectangle::new_sized((600, 600)),
            map: Map::new(),
        }
    }

    pub fn update_map(&mut self, map: Map) {
        self.map = map;
    }
    ...
}
```
We added `map: Map` to our `PlayState` and updated `fn new` to create a new empty map on startup. Lastly we added an additional function that updates the map from the current map to a new `Map`.

Now we just update our `fn draw` in `impl GameState for PlayState`. 

`src/state/play.rs`
``` Rust
#[allow(unused_must_use)]
fn draw(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()> {
    window.clear(Color::BLACK)?;
    self.map.draw_map(&self.view, window, assets);
    self.player.draw_player(window, assets)
}
```
Here we simplified our draw function and cal `draw_map` and `draw_player`. I like to split out my draw functions into seprate functions. I think this makes it easier to reason about. It has the disadvantage of needing to use `#[allow(unused_must_use)]` because each draw function returns a `Result<()>` but we are only able to return 1 of these results. To get around this you could just have 1 big draw function that you call if you want. I think both ways of doing this have there pros and cons and you should pick what you find most comfortable. 

`draw_map` we already created when we made our `Map` struct. `draw_player` we don't have yet but all we are doing is moving what we had here before to it's own function in `components.rs`.

`src/components.rs`
``` Rust
use crate::GameAssets;
use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::Background,
    lifecycle::Window,
    Result,
};
```
Update our imports like usual. Then in our `impl Play` add the following function

`src/components.rs`
``` Rust
pub fn draw_player(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()> {
    assets.atlas.execute(|atlas| {
        let texture = &atlas.get("redShip1").unwrap().unwrap_image();
        window.draw_ex(
            &self.body,
            Background::Img(texture),
            Transform::rotate(self.angle),
            self.z,
        );
        Ok(())
    })
}
```
Like I mentioned before all we did was move what we had in our `draw` function to here. It does the exact same thing, draws our player ship on the screen.

We are finally done!! Now we can run `cargo web start` and after our loading screen we see our new tilemap is loaded along with our ship that we can move around.


## Summary
Another long chapter. We got a lot done though. We leaned how to parse and external file, load an external file, create a tile map, and integrate that map into our state managment system. 

If you notice we didn't really have to add that much actual code to our game logic. Most of our time was spent learning how to parse external files in Rust. These are skills that can transfer over to other projects that you might do in the future! 

Being able to load external files is much more realistic than hard coding the level into the game. Going beyond the simplist games like tetris and snake most levels are going to be designed in some external map editor and then loeaded into the game. It would be way to tedious to hard code each level into the game.

This was a long chapter and if you were a little unsure on parts, don't forget that you can [check out the full source code here](https://github.com/DallasC/ohship). Just go to the branch with the same name as the chapter.

You might notice that when you ran the game that you can go through the "edge" of the map. Thats because we haven't set up our collision detection yet. We are going to set that up next chapter!