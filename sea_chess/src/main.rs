mod utils;
use utils::Point as Point;
mod player;
use player::Player as Player;
mod board;
use board::Board as Board;
use std::io;
// use std::{io, borrow::BorrowMut};

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
    
    let mut player_counter: usize = 0;
    let mut game_board: Board = Board::new(&players[player_counter]);
    while !game_board.is_game_end(){
        game_board.print_board();
        
        let mut current_turn = Point::new(0, 0);

        get_coordinate(game_board.get_current_player().get_name(), "x".to_string(), &mut current_turn);
        get_coordinate(game_board.get_current_player().get_name(), "y".to_string(), &mut current_turn);
        
        if !game_board.play_turn(current_turn){
            println!("####################################################################");
            println!("# Please select free position.(Free positions are marked with '*') #");
            println!("####################################################################");
            continue;
        }
        if game_board.check_for_win(){
            game_board.print_board();
            println!("Congratulation {}, you win!!!", game_board.get_current_player().get_name());
            break;
        }
        player_counter += 1;
        if player_counter == players.len(){
            player_counter = 0;
        }
        game_board.set_player(&players[player_counter])
    }
}

fn get_coordinate(player_name :&String, axis: String, current_turn: &mut Point ){
    let mut current_input = String::new();
    println!("{} please enter position '{}' :", player_name, axis.to_lowercase());
    io::stdin().read_line(&mut current_input).expect("Problem when reading symbol from console!");
    let parsed_input: Result<u8, _> = current_input.trim().parse::<u8>();
    match  parsed_input {
        Ok(value) => if axis == "y" {
            current_turn.set_y(value);
        }
        else if axis == "x"{
            current_turn.set_x(value);
        }
        Err(e) => println!("Failed to parse: {}", e),
    }
    current_input.clear();
}
