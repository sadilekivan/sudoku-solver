pub mod square_array;
pub mod board;
pub mod backtrack_raw;
pub mod backtrack_lvn;

use square_array::*;
use board::*;

// Should convert to a benchmark since I dont have the solutions to compare against
#[test]
#[ignore = "time consuming"]
fn game_many() {
    for game_id in 0..1000 {
        let board = Board::load_puzzles(game_id);
        let mut steps_raw = 0;
        backtrack_raw::solve(board, &mut steps_raw).ok_or("puzzle has no solutions").unwrap();
        let mut steps_lvn = 0;
        backtrack_lvn::solve(board, &mut steps_lvn).ok_or("puzzle has no solutions").unwrap();
        let ratio = steps_raw as f32 / steps_lvn as f32;
        println!("{ratio:.2}");
        if ratio < 1.0 {
            board.draw_board();
        }
    }
}