pub const GRID_X_SIZE: i32 = 40;
pub const GRID_Y_SIZE: i32 = 30;
pub const DOT_SIZE_IN_PXS: i32 = 20;

pub enum GameState { Playing, Paused}
pub enum PlayerDirection { Up, Down, Right, Left}
pub struct Point(pub i32, pub i32);