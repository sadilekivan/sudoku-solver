// Raw backtracking fields one by one, left to right, top to bottom
// Resources
// https://gist.github.com/syphh/62e6140361feb2d7196f2cb050c987b3
// https://www.youtube.com/watch?v=G_UYXzGuqvM
use crate::*;

fn is_valid(board: &Board, row: usize, col: usize, n: u32) -> bool {
    // Check fields in row
    for i in 0..9 {
        let p_row = Point::new(row, i);  
        if board[p_row] == n {
            return false;
        }
    }


    // Check fields in column
    for i in 0..9 {
        let p_col = Point::new(i, col);
        if board[p_col] == n {
            return false;
        }
    }

    let offset = Point::new(row, col) / 3 * 3;
    // Check fields in subgrid
    for row in 0..3 {
        for col in 0..3 {
            let p = Point::new(row, col) + offset;
            if board[p] == n {
                return false;
            }
        }
    }

    return true;
}

/// Start solving the puzzle
pub fn solve(mut board: Board, steps: &mut usize) -> Option<Board> {
    if _solve(&mut board, 0, 0, steps) {
        return Some(board)
    } else {
        return None
    }
}

/// Use `solve` for solving the puzzle
/// # Returns
/// boolean if able to continue in another solve step
fn _solve(board: &mut Board, row: usize, col: usize, steps: &mut usize) -> bool {
    let p = Point::new(row, col);
    if row == 9 {
        // Over max rows, we are done
        return true;
    } else if col == 9 {
        // Over max columns go to next row
        return _solve(board, row + 1, 0, steps);
    } else if board[p] != 0 {
        // This is a preset number, continue
        return _solve(board, row, col + 1, steps);
    } else {
        // Test all possible numbers
        for n in 1..10 {
            if is_valid(board, row, col, n) {
                // This one fits
                board[p] = n;
                *steps += 1;
                // Lets continue
                if _solve(board, row, col + 1, steps) {
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
fn game1() {
    let board = Board::load_game(include_str!("games/game1.setup"));

    let mut steps = 0;
    let board_s = solve(board, &mut steps).unwrap();
    println!("solution took {steps} solve steps");

    let solution = Board::load_game(include_str!("games/game1.solution"));

    assert_eq!(board_s, solution);
}