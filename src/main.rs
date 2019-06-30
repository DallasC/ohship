extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Vector, Rectangle, Transform},
    graphics::{Color, Background},
    lifecycle::{Settings, State, Window, run},
};

struct Game;

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game)
    }

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
}

fn main() {
    run::<Game>("Oh! Ship", Vector::new(600, 600), Settings::default());
}