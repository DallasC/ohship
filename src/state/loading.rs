use quicksilver::{
    Result, geom::{Circle, Rectangle},
    graphics::{Color, Background},
    lifecycle::{Event, Window},
};
use crate::{
    state::{StateTransition, GameState},
    GameAssets, map::Map,
};

pub struct LoadingState;

impl LoadingState{
    pub fn new() -> LoadingState {
        LoadingState
    }
}

#[allow(unused_must_use)]
impl GameState for LoadingState {
    fn update(&mut self, _window: &mut Window, assets: &mut GameAssets) -> StateTransition {
        let mut transition = StateTransition::NoTransition;
        let mut textures = false;
        assets.atlas.execute(|_| {
            textures = true;
            Ok(())
        });
        assets.map.execute(|data| {
            let new_map = Map::from_string(data);
            if textures {
                transition = StateTransition::StartGameTransition(new_map);
            }
            Ok(())
        });
        transition
    }

    fn draw(&mut self, window: &mut Window, _assets: &mut GameAssets) -> Result<()> {
        window.clear(Color::BLACK)?;
        // L
        window.draw(&Rectangle::new((100,250), (10,100)), Background::Col(Color::WHITE));
        window.draw(&Rectangle::new((110,340), (50,10)), Background::Col(Color::WHITE));
        // o
        window.draw(&Circle::new((190,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((190,325), 17.0), Background::Col(Color::BLACK));
        // a
        window.draw(&Circle::new((245,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((245,325), 17.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((262,300), (10,50)), Background::Col(Color::WHITE));
        // d
        window.draw(&Circle::new((302,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((302,325), 17.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((319,250), (10,100)), Background::Col(Color::WHITE));
        // i
        window.draw(&Rectangle::new((334,305), (10,45)), Background::Col(Color::WHITE));
        window.draw(&Circle::new((339,280), 6.0), Background::Col(Color::WHITE));
        // n
        window.draw(&Rectangle::new((349,325), (10,25)), Background::Col(Color::WHITE));
        window.draw(&Rectangle::new((379,325), (10,25)), Background::Col(Color::WHITE));
        window.draw(&Circle::new((369,325), 20.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((369,325), 10.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((359,325), (20,25)), Background::Col(Color::BLACK));
        // g
        window.draw(&Circle::new((419,325), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((419,325), 17.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((436,300), (10,75)), Background::Col(Color::WHITE));
        window.draw(&Circle::new((421,376), 25.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((421,376), 15.0), Background::Col(Color::BLACK));
        window.draw(&Rectangle::new((396,350), (40,25)), Background::Col(Color::BLACK));
        // ...
        window.draw(&Circle::new((456,345), 5.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((471,345), 5.0), Background::Col(Color::WHITE));
        window.draw(&Circle::new((486,345), 5.0), Background::Col(Color::WHITE));
        Ok(())
    }

    fn event(&mut self, _event: &Event, _window: &mut Window) -> StateTransition {
       StateTransition::NoTransition
    }
}