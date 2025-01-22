use crate::{Point, SudokuBoard, SudokuSolver};

/// Left to right, top to bottom
/// 
/// Solve puzzle by solving fields sequentialy one after another without a jump
/// 
/// Backtracking resources:
/// https://gist.github.com/syphh/62e6140361feb2d7196f2cb050c987b3
/// https://www.youtube.com/watch?v=G_UYXzGuqvM
struct Lrtb;

fn is_valid(board: &mut impl SudokuBoard, row: usize, col: usize, n: u32) -> bool {
    // Check fields in row
    for i in 0..9 {
        let p_row = Point::new(row, i);  
        if board.read(p_row) == n {
            return false;
        }
    }


    // Check fields in column
    for i in 0..9 {
        let p_col = Point::new(i, col);
        if board.read(p_col) == n {
            return false;
        }
    }

    let offset = Point::new(row, col) / 3 * 3;
    // Check fields in subgrid
    for row in 0..3 {
        for col in 0..3 {
            let p = Point::new(row, col) + offset;
            if board.read(p) == n {
                return false;
            }
        }
    }

    return true;
}

impl Lrtb {
    /// Use `solve` for solving the puzzle
    /// # Returns
    /// boolean if able to continue in another solve step
    fn solve_step(board: &mut impl SudokuBoard, row: usize, col: usize) -> bool {
        let p = Point::new(row, col);
        if row == 9 {
            // Over max rows, we are done
            return true;
        } else if col == 9 {
            // Over max columns go to next row
            return Self::solve_step(board, row + 1, 0);
        } else if board.read(p) != 0 {
            // This is a preset number, continue
            return Self::solve_step(board, row, col + 1);
        } else {
            // Test all possible numbers
            for n in 1..10 {
                if is_valid(board, row, col, n) {
                    // This one fits
                    board.write(p, n);

                    // Lets continue
                    if Self::solve_step(board, row, col + 1) {
                        // We found them all, yay
                        return true
                    }
                    // This leads nowhere lets try the next possible number
                    board.clear(p);
                }
            }
            // We tried all the numbers, go back and eventually end
            return false;
        }
    }
}

impl<T: SudokuBoard> SudokuSolver<T> for Lrtb {
    fn solve(mut board: T) -> Option<T> {
        if Self::solve_step(&mut board, 0, 0) {
            return Some(board)
        } else {
            return None
        }
    }
}

#[test]
fn able_to_solve() {
    crate::strategy::test_case::<Lrtb>();
}