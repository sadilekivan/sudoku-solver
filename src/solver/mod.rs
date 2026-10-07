use crate::board::BoardOps;

#[cfg(test)]
mod test;

pub mod linear_backtracking;
pub mod minimal_choice_backtracking;

pub trait Solver<T: BoardOps> {
    fn solve(board: T) -> Option<T>;
}
