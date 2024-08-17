mod player;
use player::Player as Player;
mod board;
use board::Board as Board;
use std::io;

fn main() {
    let mut players : Vec<Player> = Vec::new();
    
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
    
    let game_board = Board::new();
    game_board.print_board();
    println!("Print All Players");
    for player in &players{
        player.print_player_info();
    }
}
