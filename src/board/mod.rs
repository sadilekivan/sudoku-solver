use std::ops::{Index, IndexMut};

use glam::USizeVec2;

pub const BOARD_SIZE: usize = 9;
pub const BOARD_CELLS: usize = BOARD_SIZE * BOARD_SIZE;

mod board_ops;
pub use board_ops::BoardOps;

#[cfg(test)]
pub mod tracked;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Sudoku board wrapper, indexable with cardinal position
pub struct Board(pub [u8; BOARD_CELLS]);

impl Default for Board {
    fn default() -> Self {
        Self([0; BOARD_CELLS])
    }
}

impl Index<USizeVec2> for Board {
    type Output = u8;

    fn index(&self, pos: USizeVec2) -> &Self::Output {
        &self.0[cardinal_into_flat_index(pos)]
    }
}

impl IndexMut<USizeVec2> for Board {
    fn index_mut(&mut self, pos: USizeVec2) -> &mut Self::Output {
        let idx = cardinal_into_flat_index(pos);
        &mut self.0[idx]
    }
}

pub fn cardinal_from_flat_index(index: usize) -> USizeVec2 {
    let x = index % BOARD_SIZE;
    let y = index / BOARD_SIZE;
    USizeVec2::new(x, y)
}

pub fn cardinal_into_flat_index(pos: USizeVec2) -> usize {
    pos.y * BOARD_SIZE + pos.x
}

impl Board {
    /// Converts a 1D flat array index into 2D grid coordinates (column x, row y).
    pub fn load_game(data: &str) -> Result<Self, String> {
        let mut game_data_iter = data.chars().map(|ch| ch.to_digit(10).unwrap_or(0) as u8);
        let mut board: Board = Default::default();

        for field_value in board.0.iter_mut() {
            if let Some(value) = game_data_iter.next() {
                *field_value = value
            }
        }

        Ok(board)
    }

    pub fn load_games() {}

    pub fn load_games_old(data: &str) -> Result<Vec<Self>, String> {
        let data = data.lines().collect::<Vec<_>>();

        let game_v: Vec<Self> = data
            .into_iter()
            .map(|game_string| {
                let mut board: Self = Default::default();
                // Load all elements into board
                for (value, ch) in board.0.iter_mut().zip(game_string.chars()) {
                    *value = ch.to_digit(10).unwrap_or(0) as u8;
                }
                board
            })
            .collect();

        Ok(game_v)
    }

    pub fn draw_board(&self) {
        println!("┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓");
        for (pos, value) in self
            .0
            .iter()
            .enumerate()
            .map(|(i, value)| (cardinal_from_flat_index(i), value))
        {
            print!("┃");
            if *value == 0 {
                print!("   ");
            } else {
                print!(" {value} ");
            }

            if pos.x != 8 {
                if pos.x % 3 == 2 {
                    print!("┃");
                } else {
                    print!("│");
                }
            }
            println!("┃");
            if pos.y != 8 {
                if pos.y % 3 == 2 {
                    println!("┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫");
                } else {
                    println!("┠───┼───┼───╂───┼───┼───╂───┼───┼───┨");
                }
            }
        }
        println!("┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛");
    }

    pub fn print_board(&mut self) {
        for n in self.0.iter() {
            print!("{n}");
        }
        println!();
    }
}

impl BoardOps for Board {
    fn get(&self) -> &[u8; BOARD_CELLS] {
        &self.0
    }

    fn get_mut(&mut self) -> &mut [u8; BOARD_CELLS] {
        &mut self.0
    }
}
