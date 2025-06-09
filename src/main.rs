use king_crab::Board;
use king_crab::MoveGenerator;

use king_crab::CNNEvaluator;
use king_crab::HalfkaEvaluator;
use king_crab::Searcher;
use king_crab::TranspositionTable;
 
 fn main() {
   let cnn_depth = 5;
   let halfka_depth = 6;


    let mut board: Board = Board::new();
    board.from_fen(None).unwrap();
    println!("Board:\n{}", board);

   let mut evaluator1 = CNNEvaluator::new(
         "/home/alexcostea/KingCrab/evaluation_models/cnns/depthwise-cnn.onnx")
         .unwrap();
   let mut evaluator2 = HalfkaEvaluator::new(
         "/home/alexcostea/KingCrab/evaluation_models/halfka/halfka-22")
         .unwrap();

   let move_generator = MoveGenerator::new();

   let mut transposition_table1 = TranspositionTable::new(20);
   let mut transposition_table2 = TranspositionTable::new(20);

   let mut searcher1 = Searcher::new(
      &mut evaluator1,
      &move_generator, 
      &mut transposition_table1);
   let mut searcher2 = Searcher::new(
      &mut evaluator2,
      &move_generator,
      &mut transposition_table2);

   let time1 = std::time::Instant::now();
   let result1 = searcher1.search(&board, cnn_depth);

   if let Some(best_move) = result1 {
      println!("Best move with CNN evaluation: {}", best_move);
   } else {
      println!("No best move found.");
   }

   println!("Search time with CNN at depth {}: {} ms",
            cnn_depth,
            time1.elapsed().as_millis());

   let time2 = std::time::Instant::now();
   let result2 = searcher2.search(&board, halfka_depth);

   if let Some(best_move) = result2 {
      println!("Best move with Halfka evaluation: {}", best_move);
   } else {
      println!("No best move found.");
   }

   println!("Search time with Halfka at depth {}: {} ms",
            halfka_depth,
            time2.elapsed().as_millis());
 }