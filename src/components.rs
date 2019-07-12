use crate::GameAssets;
use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::Background,
    lifecycle::Window,
    Result,
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
        Player { body, angle, z }
    }

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
}