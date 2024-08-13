mod player;
use player::Player as Player;
use std::io;

fn main() {
    let mut players : Vec<Player> = Vec::new();
    // let player_one = Player::new("Vladimir".to_string(), 'X');
    // player_one.print_player_info();
    // players.push(player_one);
    // let player_two = Player::new("Ivan".to_string(), 'Y');
    // player_two.print_player_info();
    // players.push(player_two);
    
    println!("Please enter users");
    let mut input_ready:bool = false;
    while !input_ready {
        let mut input = String::new();
        println!("Please enter name :");
        io::stdin().read_line(&mut input).expect("Problem when reading name from console!");
        let name:String = input.to_string().trim().to_string();
        if !player::validate_name(&name){
            println!("Please Enter valid name!");
            continue;
        }
        
        let mut symbol:char = '*';
        while symbol == '*'{
            println!("Please enter symbol ( X/O ) :");
            input.clear();
            io::stdin().read_line(&mut input).expect("Problem when reading symbol from console!");
            if player::validate_symbol(&input.trim().to_string()) {
                symbol = input.chars().next().unwrap();
            }
        }
        
        players.push(Player::new(name.clone(), symbol.clone()));
        if players.len() == 2 {
            input_ready = true;
        }
    }
    
    println!("Print All Players");
    for player in &players{
        player.print_player_info();
    }
}
