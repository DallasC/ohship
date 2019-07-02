use quicksilver::{
    geom::{Rectangle}
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
        Player {body, angle, z}
    }
}