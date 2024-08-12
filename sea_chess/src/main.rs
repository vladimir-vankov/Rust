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
    for _ in 0..2{
        let mut input = String::new();
        println!("Please enter name :");
        io::stdin().read_line(&mut input).expect("Problem when reading name from console!");
        let name:String = input.to_string().trim().to_string();
        println!("Please enter symbol ( X/O ) :");
        input.clear();
        io::stdin().read_line(&mut input).expect("Problem when reading symbol from console!");
        // println!("Input : {}", input);
        let mut symbol:char = '*';
        println!("Input size : {}", input.trim().chars().count());
        if input.trim().chars().count() == 1 {
            symbol = input.chars().next().unwrap();
            println!("INPUT {}", symbol);
        }
        
        players.push(Player::new(name.clone(), symbol.clone()));
    }
    println!("Print All Players");
    for player in &players{
        player.print_player_info();
    }
}
