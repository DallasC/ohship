extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Vector},
    graphics::{Color},
    lifecycle::{Settings, State, Window, run},
};

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

fn main() {
    run::<Game>("Oh! Ship", Vector::new(600, 600), Settings::default());
}