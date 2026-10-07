#[cfg(test)]
use glam::Vec3;

#[cfg(test)]
use crate::{
    board::{
        tracked::{SolverTelemetry, TrackedSudokuBoard as TrackedBoard},
        Board,
    },
    solver::Solver,
};

#[cfg(test)]
pub fn run_tracked_solver<S: Solver<TrackedBoard>>() {
    let game_setup_v = Board::load_games_old(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/games/easy50_by_projecteuler-p096.setup"
    )))
    .unwrap()
    .into_iter()
    .map(TrackedBoard::from)
    .collect::<Vec<TrackedBoard>>();
    let solution_v = Board::load_games_old(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/games/easy50_by_projecteuler-p096.solution"
    )))
    .unwrap()
    .into_iter()
    .collect::<Vec<Board>>();

    let mut telemetry_vec: Vec<SolverTelemetry> = Vec::new();

    for (id, (setup, solution)) in game_setup_v.into_iter().zip(solution_v).enumerate() {
        let my_solution = S::solve(setup).expect("Could not solve puzzle!");

        telemetry_vec.push(my_solution.get_telemetry());
        assert_eq!(my_solution.inner, solution, "testing puzzle {id}");
    }

    let samples: f32 = telemetry_vec.len() as f32;
    let telemetry_sum: Vec3 = telemetry_vec.into_iter().map(Vec3::from).sum();
    let telemetry_avg: SolverTelemetry = (telemetry_sum / samples).into();

    dbg!(telemetry_avg);
}
