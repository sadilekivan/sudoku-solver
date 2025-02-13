use crate::{Board, SudokuSolver, Tracked};

pub mod lrtb;
pub mod lap;
pub mod lapmc;

#[allow(dead_code)]
fn test_case<S: SudokuSolver<Tracked<Board>>>() {
    let setup_v = Board::load_game(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/games/easy50_by_projecteuler-p096.setup"))).unwrap().into_iter().map(Tracked::<Board>::from).collect::<Vec<Tracked<Board>>>();
    let solution_v = Board::load_game(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/games/easy50_by_projecteuler-p096.solution"))).unwrap().into_iter().map(Tracked::<Board>::from).collect::<Vec<Tracked<Board>>>();

    let mut solve_steps_v: Vec<u32> = Default::default();
    let mut backtrack_steps_v: Vec<u32> = Default::default();

    for (id, (setup, solution)) in setup_v.into_iter().zip(solution_v).enumerate() {
        let my_solution = S::solve(setup).unwrap();
        
        solve_steps_v.push(my_solution.get_write_count());
        backtrack_steps_v.push(my_solution.get_clear_count());
        assert_eq!(my_solution, solution, "testing puzzle {id}");
    }

    println!("{:?}/{:?} average [solve/backtrack] steps", solve_steps_v.iter().sum::<u32>() as f32 / solve_steps_v.len() as f32, backtrack_steps_v.iter().sum::<u32>() as f32 / backtrack_steps_v.len() as f32);
}