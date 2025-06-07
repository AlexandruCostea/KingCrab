
use crate::engine::{board::board::Board, definitions::{Side, MAX_POSITION_SCORE, MIN_POSITION_SCORE}, evaluator::evaluator::Evaluator, move_generator::{chess_move::ChessMove, move_generator::MoveGenerator}};

pub struct SearchResult {
    pub best_move: Option<ChessMove>,
    pub score: f32
}

pub struct Searcher<'a> {
    pub evaluator: &'a dyn Evaluator,
    pub movegen: &'a MoveGenerator,
}

impl<'a> Searcher<'a> {
    pub fn new( evaluator: &'a dyn Evaluator, movegen: &'a MoveGenerator) -> Self {
        Searcher {
            evaluator,
            movegen,
        }
    }

    pub fn search(&self, board: &mut Board, depth: u8) -> SearchResult {
        let mut board_clone = board.clone();
        return self.search_move(&mut board_clone, depth, MIN_POSITION_SCORE, MAX_POSITION_SCORE);
    }

    pub fn search_move(&self, board: &mut Board, depth: u8, mut alpha: f32, beta: f32) -> SearchResult {

        if board.draw_by_fifty_move_rule() || board.draw_by_threefold_repetition() || board.draw_by_insufficient_material() {
            return SearchResult {
                best_move: None,
                score: 0.0,
            };
        }

        if board.game_history.len() > 0 {
            let last_move = board.game_history.get_ref(board.game_history.len() - 1);
            if last_move.mv.is_checkmate {
                return match board.get_active_side() {
                    // don't forget that the side switches after a move
                    Side::White => SearchResult {
                        best_move: None,
                        score: MIN_POSITION_SCORE,
                    },
                    Side::Black => SearchResult {
                        best_move: None,
                        score: MAX_POSITION_SCORE,
                    },
                };
            }
        }

        if depth == 0 {
            return SearchResult {
                best_move: None,
                score: self.evaluator.evaluate_board(board),
            };
        }

        let mut best_result: SearchResult = SearchResult {
            best_move: None,
            score: MIN_POSITION_SCORE,
        };

        let moves = self.movegen.generate_moves(board);

        for mv in moves {
            if mv.is_checkmate {
                return match board.get_active_side() {
                    Side::White => SearchResult {
                        best_move: Some(mv),
                        score: MAX_POSITION_SCORE,
                    },
                    Side::Black => SearchResult {
                        best_move: Some(mv),
                        score: MIN_POSITION_SCORE,
                    },
                };
            }
            board.make_move(mv);

            let mut result = self.search_move(board, depth - 1, -beta, -alpha);
            result.score = -result.score;
            board.undo_move();

            if result.score > best_result.score {
                best_result.score = result.score;
                best_result.best_move = Some(mv);
            }
            if result.score > alpha {
                alpha = result.score;
            }

            if alpha >= beta {
                break;
            }
        }

        best_result
    }
}
