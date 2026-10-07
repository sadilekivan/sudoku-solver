use glam::USizeVec2;

use crate::{board::BoardOps, solver::Solver};

/// Solve puzzle by solving fields sequentialy one after another without a jump
/// Left to right, top to bottom
///
/// Backtracking resources:
/// https://gist.github.com/syphh/62e6140361feb2d7196f2cb050c987b3
/// https://www.youtube.com/watch?v=G_UYXzGuqvM
pub struct LinearBacktracking;

impl LinearBacktracking {
    /// Use `solve` for solving the puzzle
    /// # Returns
    /// boolean if able to continue in another solve step
    fn solve_step(board: &mut impl BoardOps, pos: USizeVec2) -> bool {
        // let mut it = board.0.iter();
        // if let Some((pos, value)) = it.next() {}
        if pos.y == 9 {
            // Over max rows, we are done
            true
        } else if pos.x == 9 {
            // Over max columns go to next row
            Self::solve_step(board, USizeVec2::new(0, pos.y + 1))
        } else if board.read(pos) != 0 {
            // This is a preset number, continue
            Self::solve_step(board, pos + USizeVec2::X)
        } else {
            // Test all possible numbers
            for n in 1..=9 {
                if board.is_valid_move(pos, n) {
                    // This one fits
                    board.write(pos, n);

                    // Lets continue
                    if Self::solve_step(board, pos + USizeVec2::X) {
                        // We found them all, yay
                        return true;
                    }
                    // This leads nowhere lets try the next possible number
                    board.clear(pos);
                }
            }
            // We tried all the numbers, go back and eventually end
            false
        }
    }
}

impl<T: BoardOps> Solver<T> for LinearBacktracking {
    fn solve(mut board: T) -> Option<T> {
        if Self::solve_step(&mut board, USizeVec2::splat(0)) {
            Some(board)
        } else {
            None
        }
    }
}

#[cfg(test)]
#[test]
fn run_solver() {
    use crate::solver::test::run_tracked_solver;

    run_tracked_solver::<LinearBacktracking>();
}
