use std::ops::Add;

pub const GRID_X_SIZE: i32 = 40;
pub const GRID_Y_SIZE: i32 = 30;
pub const DOT_SIZE_IN_PXS: i32 = 20;

#[derive(PartialEq)]
pub enum GameState { Playing, Paused, Menu}
#[derive(PartialEq)]
pub enum PlayerDirection { Up, Down, Right, Left}

//Point is tuple-like struct
#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);


impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}