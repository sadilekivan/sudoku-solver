// Backtrack the puzzle starting with the fields of lowest valid numbers to be filled in

use crate::*;

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

#[test]
fn game1() {
    let board = Board::load_game(include_str!("games/game1.setup"));

    let mut steps = 0;
    let board_s = solve(board, &mut steps).ok_or("puzzle has no solutions").unwrap();
    println!("solution took {steps} solve steps");
    
    let solution = Board::load_game(include_str!("games/game1.solution"));

    assert_eq!(board_s, solution);
}
