use std::ops::{Index, IndexMut};

use glam::USizeVec2;

pub const BOARD_SIZE: usize = 9;
pub const BOARD_CELLS: usize = BOARD_SIZE * BOARD_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SudokuBoard(pub [u8; BOARD_CELLS]);

impl Default for SudokuBoard {
    fn default() -> Self {
        Self([0; BOARD_CELLS])
    }
}

impl Index<USizeVec2> for SudokuBoard {
    type Output = u8;

    fn index(&self, pos: USizeVec2) -> &Self::Output {
        &self.0[cardinal_into_flat_index(pos)]
    }
}

impl IndexMut<USizeVec2> for SudokuBoard {
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

impl SudokuBoard {
    /// Converts a 1D flat array index into 2D grid coordinates (column x, row y).
    pub fn load_game(data: &str) -> Result<Self, String> {
        let mut game_data_iter = data.chars().map(|ch| ch.to_digit(10).unwrap_or(0) as u8);
        let mut board: SudokuBoard = Default::default();

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

pub trait SudokuBoardOps
where
    Self: std::fmt::Debug + PartialEq,
{
    fn write(&mut self, pos: USizeVec2, value: u8) {
        self.get_mut()[cardinal_into_flat_index(pos)] = value;
    }

    fn clear(&mut self, pos: USizeVec2) {
        self.get_mut()[cardinal_into_flat_index(pos)] = 0;
    }

    fn read(&self, pos: USizeVec2) -> u8 {
        self.get()[cardinal_into_flat_index(pos)]
    }

    fn is_clear(&self, pos: USizeVec2) -> bool {
        self.read(pos) == 0
    }

    fn is_valid_move(&self, pos: USizeVec2, n: u8) -> bool {
        // Check field row
        for x in 0..9 {
            let pos = pos.with_x(x);
            if self.read(pos) == n {
                return false;
            }
        }

        // Check field column
        for y in 0..9 {
            let pos = pos.with_y(y);
            if self.read(pos) == n {
                return false;
            }
        }

        let offset = pos / 3 * 3;
        // Check fields in subgrid
        for x in 0..3 {
            for y in 0..3 {
                let pos = USizeVec2::new(x, y) + offset;
                if self.read(pos) == n {
                    return false;
                }
            }
        }

        true
    }

    fn get(&self) -> &[u8; BOARD_CELLS];

    fn get_mut(&mut self) -> &mut [u8; BOARD_CELLS];

    fn iter(&self) -> impl Iterator<Item = (USizeVec2, &u8)> {
        self.get()
            .iter()
            .enumerate()
            .map(|(index, value)| (cardinal_from_flat_index(index), value))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (USizeVec2, &mut u8)> {
        self.get_mut()
            .iter_mut()
            .enumerate()
            .map(|(index, value)| (cardinal_from_flat_index(index), value))
    }
}

impl SudokuBoardOps for SudokuBoard {
    fn get(&self) -> &[u8; BOARD_CELLS] {
        &self.0
    }

    fn get_mut(&mut self) -> &mut [u8; BOARD_CELLS] {
        &mut self.0
    }
}
