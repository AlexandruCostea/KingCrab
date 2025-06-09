use KingCrab::Board;
use KingCrab::MoveGenerator;

use KingCrab::CNNEvaluator;
use KingCrab::Evaluator;
use KingCrab::HalfkaEvaluator;
use KingCrab::Searcher;
use KingCrab::TranspositionTable;
 
 fn main() {
    let mut board: Board = Board::new();
    board.from_fen(None).unwrap();
    println!("Board:\n{}", board);

   let mut evaluator = HalfkaEvaluator::new("/home/alexcostea/KingCrab/evaluation_models/halfka/halfka-22").unwrap();
   // let mut evaluator = CNNEvaluator::new("/home/alexcostea/KingCrab/evaluation_models/cnns/depthwise-cnn.onnx").unwrap();

   let move_generator = MoveGenerator::new();

   let mut transposition_table = TranspositionTable::new(20);

   let mut searcher = Searcher::new(&mut evaluator, &move_generator, &mut transposition_table);
   let result = searcher.search(&board, 6);

   if let Some(best_move) = result {
      println!("Best move: {}", best_move);
   } else {
      println!("No best move found.");
   }
 }