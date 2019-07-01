extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Vector, Rectangle, Transform},
    graphics::{Color, Background, Atlas},
    input::{Key},
    lifecycle::{Asset, Settings, State, Window, run},
};

struct Player {
    body: Rectangle,
    angle: f32,
    z: i8,
}

impl Player {
    fn new() -> Player {
        let body = Rectangle::new((277, 243), (66, 113));
        let angle = 180.;
        let z = 2;
        Player {body, angle, z}
    }
}

struct Game {
    player: Player,
    atlas:  Asset<Atlas>
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            player: Player::new(),
            atlas: Asset::new(Atlas::load("spritesheet.txt"))
        })
    }
    
    fn update(&mut self, w: &mut Window) -> Result<()> {
        if w.keyboard()[Key::Up].is_down() ||  w.keyboard()[Key::W].is_down() { 
            // From degrees to radians
            let angle = self.player.angle * std::f32::consts::PI / 180f32 ; 
            let dx = angle.sin() * -4.;
            let dy = angle.cos() * 4.;
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
    }
}

fn main() {
    run::<Game>("Oh! Ship", Vector::new(600, 600), Settings::default());
}