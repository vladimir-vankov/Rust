// use std::cell::Cell;

// use std::cell::RefCell;
pub struct Board {
    //we use RefCell so desk to be mutable
    desk : [[char; 3]; 3],//RefCell<[[char; 3]; 3]>,
    game_end : bool,
}

impl Board {
    pub fn new() -> Board{
        Board {
            // desk : RefCell::new([['*', '*', '*'],
            //                     ['*', '*', '*'],
            //                     ['*', '*', '*']])
            desk : [['*', '*', '*'],
                    ['*', '*', '*'],
                    ['*', '*', '*']],
            game_end : false,
        }
    }

    pub fn print_board(self: &Board){
        for i in 0..4 { 
            for _ in 0..10{
                print!("-");
            }
            println!("");
            for x in 0..12{
                if x % 3 == 0{
                    print!("|");
                }
                else {
                    print!(" ");
                }            
            }
        }
        println!("");
        for row in self.desk {
            for col in row{
                print!("{} ", col);
            }
            println!("");
        }
    }

    pub fn is_game_end(self: &Board) -> &bool{
        &self.game_end
    }

}