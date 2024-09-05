// use std::cell::Cell;

use crate::player::Player;

// use std::cell::RefCell;
pub struct Board<'a> {
    //we use RefCell so desk to be mutable
    desk : [[char; 3]; 3],//RefCell<[[char; 3]; 3]>,
    game_end : bool,
    current_player: &'a Player
}

impl<'a> Board<'a> {
    pub fn new(current_player: &'a Player) -> Board{
        Board {
            // desk : RefCell::new([['*', '*', '*'],
            //                     ['*', '*', '*'],
            //                     ['*', '*', '*']])
            desk : [['*', '*', '*'],
                    ['*', '*', '*'],
                    ['*', '*', '*']],
            game_end : false,
            current_player : current_player,
        }
    }

    pub fn print_board(& self){
        for i in 0..4 { 
            for _ in 0..13{
                print!("-");
            }
            println!("");
            if i < 3{
                let mut col = 0;
                for x in 0..16{
                    if x % 4 == 0{
                        print!("|");
                    }
                    else {
                        if x % 2 == 0 && x < 14{
                            print!("{}",self.desk[i][col]);
                            col+=1;
                        }
                        else {
                            print!(" ");
                        }
                    }
                }
            }
            println!("");
        }
    }

    pub fn is_game_end(& self) -> &bool{
        &self.game_end
    }

    pub fn set_player(&mut self, new_player: &'a Player){
        self.current_player = new_player;
    }

    pub fn get_current_player(& self) -> &Player{
        &self.current_player
    }
}