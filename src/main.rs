mod engine;
 
use engine::board::{board::Board};
use engine::move_generator::move_generator::MoveGenerator;

use crate::engine::evaluator::cnn_evaluator::CNNEvaluator;
use crate::engine::evaluator::evaluator::Evaluator;
use crate::engine::evaluator::halfka_evaluator::HalfkaEvaluator;
use crate::engine::searcher::searcher::Searcher;
use crate::engine::searcher::transposition_table::TranspositionTable;
 
 fn main() {
    let mut board: Board = Board::new();
    board.from_fen(None).unwrap();
    println!("Board:\n{}", board);

   let mut evaluator = HalfkaEvaluator::new("/home/alexcostea/KingCrab/evaluation_models/halfka/halfka-22").unwrap();
   // let mut evaluator = CNNEvaluator::new("/home/alexcostea/KingCrab/evaluation_models/cnns/depthwise-cnn.onnx").unwrap();

   let move_generator = MoveGenerator::new();

   let mut transposition_table = TranspositionTable::new(20);

   let mut searcher = Searcher::new(&mut evaluator, &move_generator, &mut transposition_table);
   let result = searcher.search(&board, 7);

   if let Some(best_move) = result.best_move {
      println!("Best move: {}", best_move);
      println!("Score: {}", result.score);
   } else {
      println!("No best move found.");
   }
 }