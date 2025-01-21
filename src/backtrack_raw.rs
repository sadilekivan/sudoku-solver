use std::time::Instant;

// Raw backtracking fields one by one, left to right, top to bottom
// Resources
// https://gist.github.com/syphh/62e6140361feb2d7196f2cb050c987b3
// https://www.youtube.com/watch?v=G_UYXzGuqvM
use crate::*;

/// Start solving the puzzle
pub fn solve(mut board: Board) -> Option<Board> {
    if _solve(&mut board, 0, 0) {
        return Some(board)
    } else {
        return None
    }
}

/// Use `solve` for solving the puzzle
/// # Returns
/// boolean if able to continue in another solve step
fn _solve(board: &mut Board, row: usize, col: usize) -> bool {
    let p = Point::new(row, col);
    if row == 9 {
        // Over max rows, we are done
        return true;
    } else if col == 9 {
        // Over max columns go to next row
        return _solve(board, row + 1, 0);
    } else if board[p] != 0 {
        // This is a preset number, continue
        return _solve(board, row, col + 1);
    } else {
        // Test all possible numbers
        for n in 1..10 {
            if is_valid(board, row, col, n) {
                // This one fits
                board.set(p, n);

                // Lets continue
                if _solve(board, row, col + 1) {
                    // We found them all, yay
                    return true
                }
                // This leads nowhere lets try the next possible number
                board[p] = 0;
            }
        }
        // We tried all the numbers, go back and eventually end
        return false;
    }
}

#[test]
fn able_to_solve() {
    let board = Board::load_game(include_str!("games/game.setup"), 0).unwrap();

    let i = Instant::now();
    let board_s = solve(board).unwrap();
    let s = i.elapsed().as_secs_f32();
    println!("solution took {s:.6} seconds and {} solve steps", board_s.get_steps());

    let solution = Board::load_game(include_str!("games/game.solution"), 0).unwrap();

    assert_eq!(board_s, solution);
}