pub mod square_array;
pub mod board;
pub mod strategy;
pub use square_array::*;
pub use board::*;

pub trait SudokuSolver {
    fn solve(board: Board) -> Option<Board>;
}
