use std::{
    rc::Rc, cell::RefCell,
};
use quicksilver::{
    Result, lifecycle::{Event, Window},
};

// Import states
pub mod loading;
pub mod play;

use crate::{
    GameAssets, state::{
        loading::LoadingState, play::PlayState,
    }
};

pub enum CurrentState {
    Loading,
    Playing,
}

pub enum StateTransition {
    NoTransition,
    StateLessTransition(CurrentState),
}

pub struct StateManager {
    pub loading: Rc<RefCell<LoadingState>>,
    pub play: Rc<RefCell<GameState>>,
    pub current_state: CurrentState,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            loading : Rc::new(RefCell::new(LoadingState::new())),
            play: Rc::new(RefCell::new(PlayState::new())),
            current_state : CurrentState::Loading,
        }
    }

    pub fn current_state(&mut self) -> Rc<RefCell<GameState>> {
        match self.current_state {
            CurrentState::Loading => self.loading.clone(),
            CurrentState::Playing => self.play.clone(),
        }
    }

    pub fn transition_state(&mut self, transition : StateTransition) {
        match transition {
            StateTransition::NoTransition => (),
            StateTransition::StateLessTransition(state) =>  {
                self.current_state = state;
            },
        }
    }
}

pub trait GameState {
    fn update(&mut self, window: &mut Window, assets: &mut GameAssets) -> StateTransition;

    fn draw(&mut self, window: &mut Window, assets: &mut GameAssets) -> Result<()>;

    fn event(&mut self, event: &Event, _window: &mut Window) -> StateTransition;
}