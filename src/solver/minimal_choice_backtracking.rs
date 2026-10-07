use glam::USizeVec2;

use crate::{board::BoardOps, solver::Solver};

/// Least Amount Possible
///
/// Solve puzzle targeting fields with least amount of possible numbers
pub struct MinimalChoiceBacktracking;

fn get_valid_numbers(board: &impl BoardOps, pos: USizeVec2) -> Vec<u8> {
    let mut valid_number_v: Vec<u8> = (1..=9).collect();
    for i in 0..9 {
        // Retain keeps elements if true, so lets reverse it with a not and use conditions like a filter
        let p_row = pos.with_y(i);
        let p_col = pos.with_x(i);
        valid_number_v.retain(|n| !(*n == board.read(p_row) || *n == board.read(p_col)));
    }

    let offset = pos / 3 * 3;

    // Check fields in subgrid
    for row in 0..3 {
        for col in 0..3 {
            let pos = USizeVec2::new(row, col) + offset;
            valid_number_v.retain(|n| !(*n == board.read(pos)));
        }
    }
    valid_number_v
}

/// Contains info about the lowest valid move on the board
#[derive(Debug, Clone)]
struct LowestValid {
    pos: USizeVec2,
    valid_moves: Vec<u8>,
}

impl LowestValid {
    fn new(pos: USizeVec2, valid_moves: Vec<u8>) -> Self {
        Self { pos, valid_moves }
    }
}

// Find the first field with the lowest valid numbers to be filled, left to right, top to bottom
fn first_lowest_valid(board: &impl BoardOps) -> Option<LowestValid> {
    board
        .iter()
        .filter(|(_, field_value)| **field_value == 0) // Filter out empty fields
        .map(|(pos, _)| LowestValid::new(pos, get_valid_numbers(board, pos)))
        .min_by_key(|el| el.valid_moves.len())
}

impl MinimalChoiceBacktracking {
    fn solve_step(board: &mut impl BoardOps) -> bool {
        if let Some(lowest_valid) = first_lowest_valid(board) {
            for vm in lowest_valid.valid_moves {
                board.write(lowest_valid.pos, vm);

                if Self::solve_step(board) {
                    return true;
                };

                board.clear(lowest_valid.pos);
            }
            false
        } else {
            true
        }
    }
}

impl<T: BoardOps> Solver<T> for MinimalChoiceBacktracking {
    fn solve(mut board: T) -> Option<T> {
        if Self::solve_step(&mut board) {
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

    run_tracked_solver::<MinimalChoiceBacktracking>();
}
