# Input
By the end of this chapter you are going to know how to process different input events and also be able to move stuff around on the screen.

## Processing keyboard events
First lets add all of our new imports like usual at the top of our `main.rs`.

`src/main.rs`
``` Rust
use quicksilver::{
    Result,
    geom::{Vector, Rectangle, Transform},
    graphics::{Color, Background},
    input::{Key, ButtonState},
    lifecycle::{Settings, State, Window, Event, run},
};
```
Here you'll notice we added a new line with `input::{input::{Key},}`.

First lets create a new struct to hold our player character. For now I just put this above our `Game` struct. Once the main file starts getting bigger this is something I would split out into another file. 

`src/main.rs`
``` Rust
struct Player {
    body: Rectangle,
}

impl Player {
    fn new() -> Player {
        let body = Rectangle::new((277, 243), (66, 113));
        Player {body}
    }
}
```
As you can see our struct is just a rectangle for now. As you might guess we are going to draw this rectangle instead of the static one.

Next we have a helper function for generating a new player. We make a new rectangle from static values, later in the future you can change this so that we have the new player appear anywhere we want.

Ok next lets add our player to our game struct. It should look like

`src/main.rs`
``` Rust
struct Game {
    player: Player
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            player: Player::new(),
        })
    }

```
Here we just added a player type to our game type and in our `fn new()` we initialize our new player.
Next right below `fn new()` we are going to add our new update function.

`src/main.rs`
``` Rust
fn update(&mut self, w: &mut Window) -> Result<()> {
    if w.keyboard()[Key::Up].is_down() ||  w.keyboard()[Key::W].is_down() { 
        // minus because the top of the screen is 0
        self.player.body.pos.y -= 3.;
    }  
    if w.keyboard()[Key::Left].is_down() ||  w.keyboard()[Key::A].is_down() {
        self.player.body.pos.x -= 3.;
    } 
    if w.keyboard()[Key::Down].is_down() ||  w.keyboard()[Key::S].is_down() {
        self.player.body.pos.y += 3.;
    } 
    if w.keyboard()[Key::Right].is_down() ||  w.keyboard()[Key::D].is_down() {
        self.player.body.pos.x += 3.;
    }
    Ok(())
}
```
Here we have our new update function. We are just looking if a key is pressed or held down and if it is we update the associatied position. 

Why are we doing this in `update` instead of `event`?? Well I couldn't get it working in `event`. If I did it in the `event` function it would have a weird stutter when transitions from `ButtonState::Pressed` to `ButtonState::Held`. Also, it just generally didn't feel as smooth. If you know why this is let me know!

Now all we have to do is update our `fn draw` and we are good to go.

`src/main.rs`
``` Rust
fn draw(&mut self, window: &mut Window) -> Result<()> {
    window.clear(Color::BLACK)?;
    window.draw(
        &self.player.body,
        Background::Col(Color::RED),
    );
    Ok(())
}
```
As you can see we just updated our draw function to use our new `Player` information. This way when we update the players position on key press the new position gets drawn on the update. 

If we run `cargo web start` we should see our rectangle in the middle of the screen. Now you can press WASD or use the Arrow Keys and the character moves!

## Using Rotation
This movement system might work for some games (think top down games like old pokemon), If you couldn't tell from the name of our game we are making a ship based game. Ships don't have instantaneous changes in direction like this, instead they have to turn and point where they are going. 

We are going to implement this by having our right/left keys rotate the ship and our up keys move us forward in the direction that we are facing.

First lets update our player struct

`src/main.rs`
``` Rust
struct Player {
    body: Rectangle,
    angle: f32,
    z: i8,
}

impl Player {
    fn new() -> Player {
        let body = Rectangle::new((277, 243), (66, 113));
        let angle = 0.;
        let z = 2;
        Player {body, angle, z}
    }
}
```
Here we just added two new fields angle and z. Then we updated our `fn new` to make use of these. You'll see how we use these later.

Next lets update our `fn update` to our new movement system.

`src/main.rs`
``` Rust
fn update(&mut self, w: &mut Window) -> Result<()> {
    if w.keyboard()[Key::Up].is_down() ||  w.keyboard()[Key::W].is_down() { 
        // From degrees to radians
        let angle = self.player.angle * std::f32::consts::PI / 180f32 ; 
        let dx = angle.sin() * 4.;
        let dy = angle.cos() * -4.;
        self.player.body.pos.x = self.player.body.pos.x + dx;
        self.player.body.pos.y = self.player.body.pos.y + dy;
    }  
    if w.keyboard()[Key::Left].is_down() ||  w.keyboard()[Key::A].is_down() {
        self.player.angle = (self.player.angle - 2.) % 360.;
    } 
    if w.keyboard()[Key::Right].is_down() ||  w.keyboard()[Key::D].is_down() {
        self.player.angle = (self.player.angle + 2.) % 360.;
    }
    Ok(())
}
```
First you'll notice that we updated our `Up` statement. First we convert from degrees to radians. Then using this we find the change in x and y give a length of 5. Next we update our new position by adding our old position to the change in position.

Next we update our `Right` and `Left` functions. Instead of moving left or right we instead change the angle that our player is facing. 

Lastly we got rid of `Down` since thats not needed anymore.

All thats left is to update our `fn draw` to make use of our new information.

`src/main.rs`
``` Rust
fn draw(&mut self, window: &mut Window) -> Result<()> {
    window.clear(Color::BLACK)?;
    window.draw_ex(
        &self.player.body,
        Background::Col(Color::RED),
        Transform::rotate(self.player.angle),
        self.player.z
    );
    Ok(())
}
```
We went back to using `draw_ex` instead of `draw`. We also added a `rotate()` transform using the angle that our player is facing.

Now run `cargo web start` and you can see it in action.

## Summary 
We made a new character struct and gave the rectangle that we have been drawing the ability to move around. We also learned how to process keyboard events. We used the update loop for this but in later chapters we are going to cover processing other types of events using quicksilvers `fn event()`.

As always [check out the full source code here](https://github.com/DallasC/ohship). Just go to the branch with the same name as the chapter.

Next chapter we are going to learn about asset loading and load a sprite atlas. We are also going to use the sprite atlas to update finally give our rectangle an image so it isn't just a boring red rectangle any more.