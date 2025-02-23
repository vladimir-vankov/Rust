pub struct Player {
    pub name: String,
    pub symbol: char,
}

impl Player {
    pub fn new(_name:String, _symbol:char)-> Player {
        Player{
            name : _name,
            symbol : _symbol,
        }
    }
}