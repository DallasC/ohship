use quicksilver::{
    lifecycle::{Event, Window},
    Result,
};
use std::{cell::RefCell, rc::Rc};
// Import states
pub mod loading;
pub mod play;

use crate::{
    map::Map,
    state::{loading::LoadingState, play::PlayState},
    GameAssets,
};

pub enum CurrentState {
    Loading,
    Playing,
}

#[allow(dead_code)]
pub enum StateTransition {
    NoTransition,
    StateLessTransition(CurrentState),
    StartGameTransition(Map),
}

pub struct StateManager {
    pub loading: Rc<RefCell<LoadingState>>,
    pub play: Rc<RefCell<PlayState>>,
    pub current_state: CurrentState,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            loading: Rc::new(RefCell::new(LoadingState::new())),
            play: Rc::new(RefCell::new(PlayState::new())),
            current_state: CurrentState::Loading,
        }
    }

    pub fn current_state(&mut self) -> Rc<RefCell<GameState>> {
        match self.current_state {
            CurrentState::Loading => self.loading.clone(),
            CurrentState::Playing => self.play.clone(),
        }
    }

    pub fn transition_state(&mut self, transition: StateTransition) {
        match transition {
            StateTransition::NoTransition => (),
            StateTransition::StateLessTransition(state) => {
                self.current_state = state;
            }
            StateTransition::StartGameTransition(map) => {
                let mut play_state_mut = self.play.borrow_mut();
                play_state_mut.update_map(map);
                self.current_state = CurrentState::Playing;
            }
        }
    }
}

pub trait GameState {
    fn update(&mut self, window: &mut Window, assets: &mut GameAssets) -> StateTransition;

    fn draw(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()>;

    fn event(&mut self, event: &Event, _window: &mut Window) -> StateTransition;
}