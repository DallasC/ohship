# Assets
You are going to learn how Asset loading in Quicksilver works. We are also going to use our loaded asset and use it a texture on our friendly rectangle.

## Loading Assets
First we need to add our new imports. We add `Asset`, `Atlas` to quicksilver.

`src/main.rs`
``` Rust
use quicksilver::{
    Result,
    geom::{Vector, Rectangle, Transform},
    graphics::{Color, Background, Atlas},
    input::{Key},
    lifecycle::{Asset, Settings, State, Window, run},
};
```
Next we are going to add an `Atlas` to our game struct

`src/main.rs`
``` Rust
struct Game {
    player: Player,
    atlas:  Asset<Atlas>
}
```
Here you can see our `Atlas` but it is wrapped in `Asset`. Whats this asset thing. I'm glad that you asked! Asset is pretty much a polling wrapper over `Futures`. Why do we need to use futures?? Basically it's the webs fault. You can't perform a blocking action on the web so everything needs to be async. Anyways it means that anything that you want to load (images, sounds) are going to be wrapped in the `Asset` system. You could also use `include_bytes!` if you want to have the image available directly but this also has some disadvantages such as making your binary larger.

Next we need to load our `Asset`. In the same file go to your `fn new()` and add the following.

`src/main.rs`
``` Rust
fn new() -> Result<Game> {
    Ok(Game {
        player: Player::new(),
        atlas: Asset::new(Atlas::load("spritesheet.txt"))
    })
}
```
Here we added our new atlas as an asset. All static assets must be put in a folder name `static` at the top level directory. Quicksilver uses LibGDX file format for its sprite sheet. I downloaded some free assets from [Kenny.nl](https://kenney.nl/assets). Next since I dont need the entire sprite sheet I downloaded a free [texture packer](https://www.codeandweb.com/texturepacker) and just added the sprites that I wanted to the sprite sheet then exported as LibGDX format. You can copy the assets from the static folder of this directory if you are just following along or use your own sprites if you want.

## Using Assets
Next we update our `fn draw` to use the sprite sheet that we loaded.

`src/main.rs`
``` Rust
fn draw(&mut self, window: &mut Window) -> Result<()> {
window.clear(Color::BLACK)?;
let body = &self.player.body;
let angle = self.player.angle;
let z = self.player.z;
self.atlas.execute(|atlas| {
    let texture = &atlas.get("redShip1").unwrap().unwrap_image();
    window.draw_ex(
        body,
        Background::Img(texture),
        Transform::rotate(angle),
        z
    );
    Ok(())
})
```
Wow this is looking a lot different than before. Lets break it down. First we declare some new variables associated with our player. We have to do this because we can't borrow from self while inside the closure.

Next we call `execute()` on our asset.  Execute takes one closure and runs the function once the loading is complete. Once the atlas is done loading we use `atlas.get()` to get a specific sprite from the sprite sheet. In our case we are getting a sprite called `redShip1`. 

Last is our familiar `draw_ex()`. Here everything is the same but instead of `Background::Col()` we use `Background::Img()` and give it the sprite we got from our atlas. 

Now if you run `cargo web start` and wait you'll see the black screen come up first. If you wait a little longer you'll see our new ship appear. Why is there a delay? Thats because our closure has to wait for the image to load before before it is executed and draws the ship. In the next chapter once we start adding in different game states we can hide this behavior behind a loading state and display a loading screen. For now we just have to deal with a little extra wait.  

## Small Fix
Besides that everything is working.... or not. If you try to move you'll notice that our ship is driving backwards! This is because the image we are loading from our asset is pointing in the wrong direction. There are a couple of ways to fix this. We could remake our spritesheet and flip the image on there. It wouldn't be to hard but I am pretty lazy. 

Instead we are just going to flip the y direction for our movement logic so that it appears that the ship is moving in the right direction. Then we are going to have our ship start rotated 180 degrees so everything looks normal from the user perspective.

First in our new player function we change the starting angle to 180.

`src/main.rs`
``` Rust
impl Player {
    fn new() -> Player {
        let body = Rectangle::new((277, 243), (66, 113));
        let angle = 180.;
        let z = 2;
        Player {body, angle, z}
    }
}
```

Next in our `fn update()` we update our movement logic so that everything matches up with our what a user would expect. It should look like below.

`main.rs`
``` Rust
fn update(&mut self, w: &mut Window) -> Result<()> {
    if w.keyboard()[Key::Up].is_down() ||  w.keyboard()[Key::W].is_down() { 
        // From degrees to radians
        let angle = self.player.angle * std::f32::consts::PI / 180f32 ; 
        let dx = angle.sin() * -4.;
        let dy = angle.cos() * 4.;
        self.player.body.pos.x = self.player.body.pos.x + dx;
        self.player.body.pos.y = self.player.body.pos.y + dy;
    }  
    ...
}
```

Now if we run `cargo web start` everything should be working as expected. The annoying delay on the loading is still there but we won't get to fix that until next chapter when we add game states to our game.

## Summary
In this chapter you learned how the asset loading feature works in Quicksilver. You first load your img, sound, or whatever into an `Asset<T>`. Then when you need to use your asset you can access it using `asset.execute()` which only runs once the asset is finished loading. 

As always [check out the full source code here](https://github.com/DallasC/ohship). Just go to the branch with the same name as the chapter.

Next chapter we are going to add different game states to our game. This gives us the ability to have a loading screen, menus, pauses, winning and losing screens, and a number of other stuff. For now we are just going to have a loading state and game state. Also, since our code is starting to get a little long we are going to do some refactoring and split up our code base into different files.