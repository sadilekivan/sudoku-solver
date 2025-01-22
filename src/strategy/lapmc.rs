use crate::{Point, SudokuBoard, SudokuSolver};

// TODO implement checking for constraining, for now same funcionality as Lap

/// Least Amount Possible, Most Constraining
/// 
/// Solve puzzle targeting fields with least amount of possible numbers and if filled constraining most amount of empty cells
struct LapMc;

fn get_valid_numbers(board: &mut impl SudokuBoard, p: Point) -> Vec<u32> {
    let mut valid_number_v: Vec<u32> = (1..=9).collect();
    for i in 0..9 {
        // Retain keeps elements if true, so lets reverse it with a not and use conditions like a filter
        let p_row = p.with_y(i);
        let p_col = p.with_x(i);
        valid_number_v.retain(|n| !(*n == board.read(p_row) || *n == board.read(p_col)));
    }

    let offset = p / 3 * 3;

    // Check fields in subgrid
    for row in 0..3 {
        for col in 0..3 {
            let p = Point::new(row, col) + offset;
            valid_number_v.retain(|n| !(*n == board.read(p)));
        }
    }
    valid_number_v
}

/// Contains info about the lowest valid move on the board
#[derive(Debug, Clone)]
struct LowestValid {
    p: Point,
    valid_moves: Vec<u32>
}

impl LowestValid {
    fn new(p: Point, valid_moves: Vec<u32>) -> Self {
        Self {p, valid_moves}
    }
}

// Find the first field with the lowest valid numbers to be filled, left to right, top to bottom
fn first_lowest_valid(board: &mut impl SudokuBoard) -> Option<LowestValid> {
    let v: Vec<LowestValid> = board.into_iter().filter_map(|f| if f.1 == 0 {Some(f)} else {None} ).map(|(p, _)| 
        LowestValid::new(p, get_valid_numbers(board, p))
    ).collect();

    if v.is_empty() {
        return None;
    }
    
    let mut lowest_valid = v[0].to_owned();
    
    // Find lowest valid field
    for el in &v {
        if el.valid_moves.len() < lowest_valid.valid_moves.len() {
            lowest_valid = el.clone();
        }
    }

    Some(lowest_valid)
}

impl LapMc {
    fn solve_step(board: &mut impl SudokuBoard) -> bool {
        if let Some(lv) = first_lowest_valid(board) {

            for vm in lv.valid_moves {
                board.write(lv.p, vm);

                if Self::solve_step(board) {
                    return true;
                };

                board.clear(lv.p);
            }
            return false;
        } else {
            return true;
        }
    }
}

impl<T: SudokuBoard> SudokuSolver<T> for LapMc {
    fn solve(mut board: T) -> Option<T> {
        if Self::solve_step(&mut board) {
            return Some(board)
        } else {
            return None
        }
    }
}

#[test]
fn able_to_solve() {
    crate::strategy::test_case::<LapMc>();
}