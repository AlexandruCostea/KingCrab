mod engine;
 
use engine::board::{board::Board};
use engine::move_generator::move_generator::MoveGenerator;

use crate::engine::evaluator::cnn_evaluator::CNNEvaluator;
use crate::engine::evaluator::evaluator::Evaluator;
use crate::engine::searcher;
use crate::engine::searcher::searcher::Searcher;
 
 fn main() {
    let mut board: Board = Board::new();
    board.from_fen(None).unwrap();
    println!("Board:\n{}", board);

   let evaluator = CNNEvaluator::new("/home/alexcostea/KingCrab/evaluation_models/depthwise-cnn.onnx").unwrap();

    let move_generator = MoveGenerator::new();

   let searcher = Searcher::new(&evaluator, &move_generator);
   let result = searcher.search(&mut board, 3);

   if let Some(best_move) = result.best_move {
      println!("Best move: {}", best_move);
      println!("Score: {}", result.score);
   } else {
      println!("No best move found.");
   }
 }