pub mod square_array;
pub mod board;
pub mod strategy;
pub use square_array::*;
pub use board::*;

pub trait SudokuBoard: IntoIterator<Item = (Point, u32), IntoIter = SquareArrayIter<9>>
where
    Self: std::fmt::Debug + Clone + Copy + PartialEq
{
    fn write(&mut self, p: Point, n: u32);

    fn clear(&mut self, p: Point);

    fn read(&mut self, p: Point) -> u32;
}

pub trait SudokuSolver<T: SudokuBoard> {
    fn solve(board: T) -> Option<T>;
}
