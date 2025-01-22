use crate::SquareArray;

pub type Board = SquareArray<9>;

impl Board {
    pub fn draw_board(&self) {
        println!("┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓");
        for (row_i, row) in self.data.iter().enumerate() {
            print!("┃");
            for (col_i, el) in row.iter().enumerate() {
                
                if *el == 0 {
                    print!("   ");
                } else {
                    print!(" {el} ");
                }
    
                if col_i != 8 {
                    if col_i % 3 == 2 {
                        print!("┃");
                    } else {
                        print!("│");
                    }
                }
            }
            println!("┃");
            if row_i != 8 {
                if row_i % 3 == 2 {
                    println!("┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫");
                } else {
                    println!("┠───┼───┼───╂───┼───┼───╂───┼───┼───┨");
                }
            }
        }
        println!("┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛");
    }

    pub fn print_board(&self) {
        for (_, n) in self.into_iter() {
            print!("{n}");
        }
        println!("");
    }
}
