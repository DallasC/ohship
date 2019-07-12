use crate::{
    components::Player,
    map::Map,
    state::{GameState, StateTransition},
    GameAssets,
};
use quicksilver::{
    geom::{Rectangle, Shape},
    graphics::{Color, View},
    input::Key,
    lifecycle::{Event, Window},
    Result,
};


pub struct PlayState {
    player: Player,
    view: Rectangle,
    map: Map,
}

impl PlayState {
    pub fn new() -> PlayState {
        PlayState {
            player: Player::new(),
            view: Rectangle::new_sized((600, 600)),
            map: Map::new(),
        }
    }

    pub fn update_map(&mut self, map: Map) {
        self.map = map;
    }

    fn update_position(&mut self) {
        // From degrees to radians
        let angle = self.player.angle * std::f32::consts::PI / 180f32;
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
        if window.keyboard()[Key::Up].is_down() || window.keyboard()[Key::W].is_down() {
            self.update_position();
            window.set_view(View::new(self.view));
        }
        if window.keyboard()[Key::Left].is_down() || window.keyboard()[Key::A].is_down() {
            self.player.angle = (self.player.angle - 2.) % 360.;
        }
        if window.keyboard()[Key::Right].is_down() || window.keyboard()[Key::D].is_down() {
            self.player.angle = (self.player.angle + 2.) % 360.;
        }
        StateTransition::NoTransition
    }

    #[allow(unused_must_use)]
    fn draw(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.map.draw_map(&self.view, window, assets);
        self.player.draw_player(window, assets)
    }

    fn event(&mut self, _event: &Event, _window: &mut Window) -> StateTransition {
        StateTransition::NoTransition
    }
}