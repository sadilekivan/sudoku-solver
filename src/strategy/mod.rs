use glam::Vec3;

use crate::{
    board_tracker::{SudokuSolverTelemetry, TrackedSudokuBoard},
    sudoku_board::{SudokuBoard, SudokuBoardOps},
};

pub mod linear_backtracking;
pub mod minimal_choice_backtracking;

pub trait SudokuSolver<T: SudokuBoardOps> {
    fn solve(board: T) -> Option<T>;
}

pub fn run_sudoku_solver<S: SudokuSolver<TrackedSudokuBoard>>() {
    let game_setup_v = SudokuBoard::load_games_old(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/games/easy50_by_projecteuler-p096.setup"
    )))
    .unwrap()
    .into_iter()
    .map(TrackedSudokuBoard::from)
    .collect::<Vec<TrackedSudokuBoard>>();
    let solution_v = SudokuBoard::load_games_old(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/games/easy50_by_projecteuler-p096.solution"
    )))
    .unwrap()
    .into_iter()
    .collect::<Vec<SudokuBoard>>();

    let mut telemetry_vec: Vec<SudokuSolverTelemetry> = Vec::new();

    for (id, (setup, solution)) in game_setup_v.into_iter().zip(solution_v).enumerate() {
        let my_solution = S::solve(setup).expect("Could not solve puzzle!");

        telemetry_vec.push(my_solution.get_telemetry());
        assert_eq!(my_solution.inner, solution, "testing puzzle {id}");
    }

    let samples: f32 = telemetry_vec.len() as f32;
    let telemetry_sum: Vec3 = telemetry_vec.into_iter().map(Vec3::from).sum();
    let telemetry_avg: SudokuSolverTelemetry = (telemetry_sum / samples).into();

    dbg!(telemetry_avg);
}
