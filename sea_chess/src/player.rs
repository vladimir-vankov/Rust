
pub struct Player {
    name : String,
    symbol : char,
}

impl Player {
    // Define the `new` function
    pub fn new(name: String, symbol: char) -> Player {
        Player { 
            name : name, 
            symbol : symbol,
        }
    }

    // self must be not mutable reference to Player. So it won't take the ownership.
    pub fn print_player_info(self: &Player){
        println!("Player Name : {} \nPlayer Symbol : {}", self.name, self.symbol);
    }


    pub fn get_name(self: &Player) -> &String {
        &self.name
    }

    pub fn get_symbol(self: &Player) -> char {
        self.symbol.clone()
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

#[test]
fn test_symbol(){
    assert_eq!(validate_symbol(&"X".to_string()), true);
    assert_eq!(validate_symbol(&"x".to_string()), false);
    assert_eq!(validate_symbol(&"O".to_string()), true);
    assert_eq!(validate_symbol(&"o".to_string()), false);
    assert_eq!(validate_symbol(&"".to_string()), false);
    for letter in String::from("abcdefghijklmopqrstuvwxyz").chars(){
        assert_eq!(validate_symbol(&letter.to_string()), false);
    }
}
#[test]
fn test_name(){
    assert_eq!(validate_name(&"Vladimir".to_string()), true);
    assert_eq!(validate_name(&"x".to_string()), true);
    assert_eq!(validate_name(&"".to_string()), false);
}