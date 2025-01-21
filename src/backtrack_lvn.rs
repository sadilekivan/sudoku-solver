// Backtrack the puzzle starting with the fields of lowest valid numbers to be filled in

use crate::*;

/// Start solving the puzzle
pub fn solve(mut board: Board, steps: &mut usize) -> Option<Board> {
    if _solve(&mut board, steps) {
        return Some(board)
    } else {
        return None
    }
}

fn _solve(board: &mut Board, steps: &mut usize) -> bool {
    if let Some(lv) = first_lowest_valid(board) {
        for vm in lv.valid_moves {
            board[lv.p] = vm;
            *steps += 1;
            if _solve(board, steps) {
                return true;
            };

            board[lv.p] = 0;
        }
        return false;
    } else {
        return true;
    }
}

#[test]
fn able_to_solve() {
    let board = Board::load_game(include_str!("games/game.setup"), 0).unwrap();

    let mut steps = 0;
    let board_s = solve(board, &mut steps).unwrap();
    println!("solution took {steps} solve steps");

    let solution = Board::load_game(include_str!("games/game.solution"), 0).unwrap();

    assert_eq!(board_s, solution);
}