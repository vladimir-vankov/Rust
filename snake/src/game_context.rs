use crate::const_vars::*;

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point(3, 3),
        }
    }
}