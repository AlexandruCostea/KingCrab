mod engine;
 
use engine::board::board::Board;
 
 fn main() {
    let mut board: Board = Board::new();
    println!("Initial board: {}", board);

    board.from_fen(None).unwrap();
    println!("Board after FEN: {}", board);
}
