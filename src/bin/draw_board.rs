use sudoku_solver::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let game_id = args[1].parse::<usize>().unwrap();
    let data = include_str!("../games/game.setup");
    let board = Board::load_game(data, game_id).unwrap();
    board.draw_board();
}