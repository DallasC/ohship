extern crate quicksilver;

mod components;

mod map;
mod state;
use map::Map;

use quicksilver::{
    geom::Vector,
    graphics::Atlas,
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Result,
};
use state::StateManager;
pub struct GameAssets {
    pub atlas: Asset<Atlas>,
    pub map: Asset<String>,
}

impl GameAssets {
    pub fn new() -> GameAssets {
        GameAssets {
            atlas: Asset::new(Atlas::load("spritesheet.txt")),
            map: Map::load("level.txt"),
        }
    }
}

struct Game {
    state: StateManager,
    assets: GameAssets,
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            state: StateManager::new(),
            assets: GameAssets::new(),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let current_state_ref = self.state.current_state();
        let transition = {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.update(window, &mut self.assets)
        };
        self.state.transition_state(transition);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let current_state_ref = self.state.current_state();
        {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.draw(window, &mut self.assets)
        }
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        let current_state_ref = self.state.current_state();
        let transition = {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.event(event, window)
        };
        self.state.transition_state(transition);
        Ok(())
    }

}

fn main() {
    run::<Game>("Oh! Ship", Vector::new(600, 600), Settings::default());
}