pub struct Player {
    name : String,
    symbol : char
}

impl Player {
    // Define the `new` function
    pub fn new(name: String, symbol: char) -> Player {
        Player { name, symbol }
    }

    // self must be not mutable reference to Player. So it won't take the ownership.
    pub fn print_player_info(self: &Player){
        println!("Player Name : {} \nPlayer Symbol : {}", self.name, self.symbol);
    }
}