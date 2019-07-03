use quicksilver::{
    Result, geom::{Transform, Rectangle, Shape},
    graphics::{Color, Background, View},
    input::{Key}, lifecycle::{Event, Window},
};
use crate::{
    state::{StateTransition, GameState},
    components::Player, GameAssets,
};

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

    fn event(&mut self, _event: &Event, _window: &mut Window) -> StateTransition {
       StateTransition::NoTransition
    }
}