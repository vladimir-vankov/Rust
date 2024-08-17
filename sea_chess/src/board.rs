use std::cell::RefCell;

pub struct Board {
    desk : RefCell<[[char; 3]; 3]>,
}

impl Board {
    pub fn new() -> Board{
        Board {
            desk : RefCell::new([['*', '*', '*'],
                                ['*', '*', '*'],
                                ['*', '*', '*']]),
        }
    }

    pub fn print_board(self: &Board){
        for row in self.desk.borrow().iter(){
            for col in row{
                print!("{} ", col);
            }
            println!("");
        }
    }
}