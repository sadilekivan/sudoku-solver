use sudoku_solver::strategy::{
    linear_backtracking::LinearBacktracking,
    minimal_choice_backtracking::MinimalChoiceBacktracking, run_sudoku_solver,
};

fn main() {
    run_sudoku_solver::<LinearBacktracking>();
    run_sudoku_solver::<MinimalChoiceBacktracking>();
}
