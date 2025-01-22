use sudoku_solver::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let game_id = args[1].parse::<usize>().unwrap();
    let data = include_str!("../../games/easy50_by_projecteuler-p096.setup");
    let mut board = Board::load_game(data).unwrap();
    board.remove(game_id).draw_board();
}