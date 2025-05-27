mod engine;
 
use engine::board::{board::Board};
use engine::move_generator::move_generator::MoveGenerator;
 
 fn main() {
    let mut board: Board = Board::new();
    board.from_fen(Some("8/8/1B3B2/8/3p2Q1/4k3/1B6/3Q2K1 w - - 0 1")).unwrap();
    println!("Board:\n{}", board);

    let move_generator = MoveGenerator::new();
    let moves = move_generator.generate_moves(&mut board);

    println!("Legal moves: {}", moves.len());

    for move1 in &moves {
        println!("{}", move1);
    }
 }