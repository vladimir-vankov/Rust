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

    pub fn is_game_end(self: &Board) -> &bool{
        &self.game_end
    }

}