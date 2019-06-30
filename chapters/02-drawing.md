# Draw
By the end of this chapter you are going to be able to draw basic shapes on the canvas.

## Our first shape
Go over to your `main.rs` and at the top add `Rectangle` to `geom` and `Background` to graphics. It should look like below:

Next go to your `fn draw` in your `impl State for Game`. We are going to draw a square in the middle of the screen. 

`src/main.rs`
``` Rust
fn draw(&mut self, window: &mut Window) -> Result<()> {
    window.clear(Color::BLACK)?;
    window.draw(
            &Rectangle::new((275, 275), (50, 50)),
            Background::Col(Color::RED),
    );
    Ok(())
}
```

Here we just added a `.draw()` function to the window. It takes two parameters:
- Drawable
    These are shapes that you can draw on the screen. Quicksilver comes with several built in drawables including [Rectangle](https://docs.rs/quicksilver/0.3.15/quicksilver/geom/struct.Line.html), [Circle](https://docs.rs/quicksilver/0.3.15/quicksilver/geom/struct.Circle.html), [Triangle](https://docs.rs/quicksilver/0.3.15/quicksilver/geom/struct.Circle.html), or [Line](https://docs.rs/quicksilver/0.3.15/quicksilver/geom/struct.Line.html). It also has support for custom [meshes](https://docs.rs/quicksilver/0.3.15/quicksilver/tutorials/_11_mesh/index.html) so you can make your own custom drawables if you want. 
- Background
    This is what is displayed on the drawable. A background can either be a [solid color](https://docs.rs/quicksilver/0.3.15/quicksilver/graphics/struct.Color.html) via `Background::Col()`, an [image](https://docs.rs/quicksilver/0.3.15/quicksilver/graphics/struct.Image.html) via `Background::Img()`, or a color and image blended multiplicatively via `Background::Blended()`.
 
 Now if we run `cargo web start` and go to our browser we see a nice red square in the middle of the canvas.

 ## Transforms
 Now that was pretty easy so we are going to add some transforms to the shape. We need to add the `Transform` to import up top so it looks like:
 
 `src/main.rs`
``` Rust
use quicksilver::{
    Result,
    geom::{Vector, Rectangle, Transform},
    graphics::{Color, Background},
    lifecycle::{Settings, State, Window, run},
};
```

Go back to your shape and change it to the following:

`src/main.rs`
``` Rust
fn draw(&mut self, window: &mut Window) -> Result<()> {
    window.clear(Color::BLACK)?;
    window.draw_ex(
        &Rectangle::new((275, 275), (50, 50)),
        Background::Col(Color::RED),
        Transform::rotate(45) * Transform::scale((2, 2)),
        0
    );
    Ok(())
}
```

You'll notice we changed `draw()` to `draw_ex()`. This takes 4 args in total instead of 2. They are
- Drawable
    Same as `draw()`
- Background
    Same as `draw()`
- Transform
    Transforms the shape. There are 4 transfroms `IDENTITY` does nothing returns the same shape, `rotate(angle)`rotates counter-clockwise by a given amount of **degrees** (not raidians), `translate(vector)` moves an object by a given vector, and `scale(vector)` resizes with a given x and y axis scale factor.

    These transforms can also be chained together with the last transform in a chain being applied first. In our example we first double the size then rotate it 45 degrees. 
- z value
    This is the order in which things are drawn on the screen. A higher z value is drawn on top of things with a lower z value. We only have one thing so far so this doesn't matter for us right now.

Now if we run `cargo web start` you'll see that our square is now a diamond.

## Summary
Here was just a quick lesson on how to get things to appear on the screen. You also learned how to apply different transforms to the things that you want to draw.

 As always [check out the full source code here](https://github.com/DallasC/ohship). Just go to the branch with the same name as the chapter.

 Next chapter we are going to learn how to process input events to move our shape around!



