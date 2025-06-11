use king_crab::Board;
use king_crab::MoveGenerator;

use king_crab::CNNEvaluator;
use king_crab::HalfkaEvaluator;
use king_crab::Searcher;
use king_crab::TranspositionTable;

use std::env;
use std::process;
 
 fn main() {

   let args: Vec<String> = env::args().collect();

   if args.len() < 2 {
        eprintln!("Usage: {} <cnn_model_path> <cnn_depth> <halfka_model_path> <halfka_depth> [fen]", args[0]);
        process::exit(1);
   }

   let cnn_model_path = &args[1];
   let cnn_depth = args.get(2)
       .and_then(|s| s.parse::<u8>().ok())
       .unwrap_or(5);

   let halfka_model_folder_path = &args[3];
   let halfka_depth = args.get(4)
       .and_then(|s| s.parse::<u8>().ok())
       .unwrap_or(6);


    let mut board: Board = Board::new();
    board.from_fen(None).unwrap();
    println!("Board:\n{}", board);

   let mut evaluator1 = CNNEvaluator::new(cnn_model_path,)
         .unwrap();
   let mut evaluator2 = HalfkaEvaluator::new(halfka_model_folder_path,)
         .unwrap();

   let move_generator = MoveGenerator::new();

   let mut transposition_table = TranspositionTable::new(20);

   let mut searcher = Searcher::new(
      &mut evaluator1,
      &move_generator, 
      &mut transposition_table);

   let time1 = std::time::Instant::now();
   let result1 = searcher.search(&board, cnn_depth);

   if let Some(best_move) = result1 {
      println!("Best move with CNN evaluation: {}", best_move);
   } else {
      println!("No best move found.");
   }

   println!("Search time with CNN at depth {}: {} ms",
            cnn_depth,
            time1.elapsed().as_millis());

   searcher.switch_evaluator(&mut evaluator2);
   let time2 = std::time::Instant::now();
   let result2 = searcher.search(&board, halfka_depth);

   if let Some(best_move) = result2 {
      println!("Best move with Halfka evaluation: {}", best_move);
   } else {
      println!("No best move found.");
   }

   println!("Search time with Halfka at depth {}: {} ms",
            halfka_depth,
            time2.elapsed().as_millis());
 }