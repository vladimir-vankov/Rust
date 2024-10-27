use crate::const_vars::*;

pub struct GameContext {
    pub state: GameState
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            state: GameState::Menu
        }
    }

    pub fn next_tick(&mut self){
        if self.state == GameState::Paused || self.state == GameState::Menu {
            return;
        }
    }

    pub fn toggle_pause(&mut self){
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused | GameState::Menu => GameState::Playing
        }
    }
    pub fn toggle_menu(&mut self){
        self.state = match self.state {
            GameState::Playing | GameState::Paused=> GameState::Menu,
            GameState::Menu => GameState::Paused
        }
    }
}