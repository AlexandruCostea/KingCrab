mod engine;
 
use engine::board::{board::Board};
use engine::move_generator::chess_move::ChessMove;
use engine::definitions::Square;
 
 fn main() {
    let mut board: Board = Board::new();
    board.from_fen(None).unwrap();
    println!("Board:\n{}", board);

    let move1 = ChessMove::double_pawn_push(Square::E2, Square::E4);
    board.make_move(move1);

    println!("After move:\n{}", board);

    let move2 = ChessMove::quiet(Square::E7, Square::E6);
    board.make_move(move2);

    println!("After move:\n{}", board);

    let move3 = ChessMove::quiet(Square::E4, Square::E5);
    board.make_move(move3);

    println!("After move:\n{}", board);

    let move4 = ChessMove::double_pawn_push(Square::D7, Square::D5);
    board.make_move(move4);

    println!("After move:\n{}", board);

    let move5 = ChessMove::en_passant(Square::E5, Square::D6);
    board.make_move(move5);

    println!("After move:\n{}", board);

    board.undo_move();
    println!("After undo:\n{}", board);
 }