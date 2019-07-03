# Camera
In this chapter we are going to set up a camera to follow our our ship around and keep it in the center of the screen. In Quicksilver the camera is called a view and it handles translating things from world space to the screen space

Screen space is where things are located on you screen with (0,0) being the top left conner of what you see. World space is where things are located in the game world/level. (0,0) in world space is the top left of your level. If your character was in the middle of the level *world space* and *screen space* the top left is (0,0) for both but they would have different values

Anyways Quicksilver handles all of this for us so it's super easy and we don't really have to think about things to much. In fact it only takes 4 lines of code to add a camera to our game! We are going to move some things to a help function just to clean things up a little but we aren't really adding that much new code.

## Creating A View
First go to `src/state/play.rs` and add a couple new imports to quicksilver. It should look like below.

`src/state/play.rs`
``` Rust
use quicksilver::{
    Result, geom::{Transform, Rectangle, Shape},
    graphics::{Color, Background, View},
    input::{Key}, lifecycle::{Event, Window},
};
```
Now we are going to add a view to our `PlayState` struct.

`src/state/play.rs`
``` Rust
pub struct PlayState {
    player: Player,
    view: Rectangle,
}

impl PlayState {
    pub fn new()  -> PlayState {
        PlayState {
            player: Player::new(),
            view: Rectangle::new_sized((600, 600)),
        }
    }
}
```
You can see here that our view struct is really just Rectangle that is the size of our canvas. 

Next we are going to add a helper functin to our `impl PlayState` in order to clean up our `update` function.

`src/state/play.rs`
``` Rust
impl PlayState {
    pub fn new()  -> PlayState {
        PlayState {
            player: Player::new(),
            view: Rectangle::new_sized((600, 600)),
        }
    }

    fn update_position(&mut self) {
        // From degrees to radians
        let angle = self.player.angle * std::f32::consts::PI / 180f32 ; 
        let dx = angle.sin() * -4.;
        let dy = angle.cos() * 4.;
        // Update player position
        self.player.body.pos.x = self.player.body.pos.x + dx;
        self.player.body.pos.y = self.player.body.pos.y + dy;
        // Update view position
        self.view = self.view.translate((dx, dy));

    }
}
```
As you can see we just add the `update_position()` function. If you noticed we just copy and pasted our movement function that was in the if statement of the `update` function down below. The only line that we add is the last one where we `translate` the view and set it as our new view. This just means that we are moving the camera the same amount as we are moving our character. This makes it look like our character is always in the center of the screen.

Now all we have to do is update our `update` function and we are good to go.

`src/state/play.rs`
``` Rust
impl GameState for PlayState {

    fn update(&mut self, window: &mut Window, _assets: &mut GameAssets) -> StateTransition {
        if window.keyboard()[Key::Up].is_down() ||  window.keyboard()[Key::W].is_down() { 
            self.update_position();
            window.set_view(View::new(self.view));
        }  
        if window.keyboard()[Key::Left].is_down() ||  window.keyboard()[Key::A].is_down() {
            self.player.angle = (self.player.angle - 2.) % 360.;
        } 
        if window.keyboard()[Key::Right].is_down() ||  window.keyboard()[Key::D].is_down() {
            self.player.angle = (self.player.angle + 2.) % 360.;
        }
        StateTransition::NoTransition
    }
```
Here we just set `update` to use our new `update_position` function and we added a new line that sets the view for us. This a function provided by quicksilver that handles all the translating from world cords to screen cords for us. We just give it a rectangle to use as a view and it does all the hard work.

Now we can run `cargo web start` and see it in action!.. well that wasn't very exciting was it. It looks like our boat is not moving at all. We can turn but not move? Actually everything is working fine, our ship is moving but so is our camera. We just don't have any point of reference since the background is all the same.

## Adding A Reference Point
To fix this we just need to add a reference point to make it easier to tell that our ship is moving. Normally we would have a tilemap, background, or some sort of objects that stays still so that we can tell things are actually moving. Like in mario the level is mostly fixed in place except for enimies/moving objects. 

Next chapter we are going to add a tilemap and create a level but for now lets just add a big square to the canvas so we have a refernce point. We are just going to add the following line to `draw()`.

``` Rust
window.draw(&Rectangle::new((250,250), (300,300)), Background::Col(Color::BLUE));
```
The full draw function now looks like this.

`src/state/play.rs`
``` Rust
fn draw(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()> {
    window.clear(Color::BLACK)?;
    let body = &self.player.body;
    let angle = self.player.angle;
    let z = self.player.z;
    window.draw(&Rectangle::new((250,250), (300,300)), Background::Col(Color::BLUE));
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
```
That's it. Now if we run `cargo web start` we can see our ship is moving around but the camera is following and keeping the boat in the center of the screen. 

## Summary
Well that was easy. To set up a basic camera to keep our character in the middle of the screen only took 4 lines of code! Told you this going to be a quick chapter.

As a side note, isn't our state system nice? To add a camera to our game we only had to touch 1 file the entire time. Also, the file isn't cluttered with a bunch of random unrelated functions. It only has stuff related to our play state.

Also, don't forget that you can [check out the full source code here](https://github.com/DallasC/ohship). Just go to the branch with the same name as the chapter.

Next chapter we'll be creating a basic tile map for our boat to sail around on!