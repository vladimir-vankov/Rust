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

pub fn validate_name(name: &String) -> bool{
    let mut is_valid:bool = true;
    if name.trim() == "" {
        is_valid = false;
    }
    else if name.starts_with("_") {
        is_valid = false;
    }
    is_valid
}

pub fn validate_symbol(symbol: &String) -> bool{
    let mut is_valid:bool = true;
    if symbol.chars().count() < 1 {
        is_valid = false;
        println!("Symbol must be one letter X/O");
    }
    else if symbol.chars().next().unwrap() != 'X' && symbol.chars().next().unwrap() != 'O' {
        is_valid = false;
    }
    is_valid
}