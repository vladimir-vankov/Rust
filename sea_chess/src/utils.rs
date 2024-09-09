pub struct Point{
    x:u8,
    y:u8
} 

impl Point{
    pub fn new(_x:u8, _y:u8) -> Point{
        Point{x : _x, y: _y}
    }

    pub fn set_x(&mut self, _x: u8){
        self.x = _x;
    }

    pub fn set_y(&mut self, _y: u8){
        self.y = _y;
    }

    pub fn get_x(&self) -> u8{
        self.x 
    }

    pub fn get_y(&self) -> u8{
        self.y 
    }
}