// Backtrack the puzzle starting with the fields of lowest valid numbers to be filled in

use crate::*;

/// Start solving the puzzle
pub fn solve(mut board: Board) -> Option<Board> {
    if _solve(&mut board) {
        return Some(board)
    } else {
        return None
    }
}

fn _solve(board: &mut Board) -> bool {
    if let Some(lv) = first_lowest_valid(board) {
        for vm in lv.valid_moves {
            board.set(lv.p, vm);

            if _solve(board) {
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
    use std::time::Instant;
    
    let board = Board::load_game(include_str!("games/game.setup"), 0).unwrap();

    let i = Instant::now();
    let board_s = solve(board).unwrap();
    let s = i.elapsed().as_secs_f32();
    println!("solution took {s:.6} seconds and {} solve steps", board_s.get_steps());

    let solution = Board::load_game(include_str!("games/game.solution"), 0).unwrap();

    assert_eq!(board_s, solution);
}