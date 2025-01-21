pub mod square_array;
pub mod board;
pub mod backtrack_raw;
pub mod backtrack_lvn;
pub mod backtrack_lvnach;

pub use square_array::*;
pub use board::*;

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

fn get_valid_numbers(board: &Board, p: Point) -> Vec<u32> {
    let mut valid_number_v: Vec<u32> = (1..=9).collect();
    for i in 0..9 {
        // Retain keeps elements if true, so lets reverse it with a not and use conditions like a filter
        let p_row = p.with_y(i);
        let p_col = p.with_x(i);
        valid_number_v.retain(|n| !(*n == board[p_row] || *n == board[p_col]));
    }

    let offset = p / 3 * 3;

    // Check fields in subgrid
    for row in 0..3 {
        for col in 0..3 {
            let p = Point::new(row, col) + offset;
            valid_number_v.retain(|n| !(*n == board[p]));
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
fn first_lowest_valid(board: &Board) -> Option<LowestValid> {
    let v: Vec<LowestValid> = board.into_iter().skip_filled().map(|(p, _)| 
        LowestValid::new(p, get_valid_numbers(board, p))
    ).collect();

    if v.is_empty() {
        return None;
    }
    
    let mut lowest_valid = v[0].to_owned();

    // Find lowest valid field
    for el in v {
        if el.valid_moves.len() < lowest_valid.valid_moves.len() {
            lowest_valid = el;
        }
    }

    Some(lowest_valid)
}

// Should convert to a benchmark since I dont have the solutions to compare against
#[test]
#[ignore = "time consuming"]
fn game_many() {
    for game_id in 0..1000 {
        let board = Board::load_game(include_str!("games/puzzles1_unbiased"), game_id).unwrap();
        backtrack_raw::solve(board).ok_or("puzzle has no solutions").unwrap();
        backtrack_lvn::solve(board).ok_or("puzzle has no solutions").unwrap();
    }
}