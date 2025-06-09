use std::collections::HashMap;

use crate::engine::{board::board::Board, definitions::Piece};
use super::chess_move::ChessMove;


struct ScoredMove {
    mv: ChessMove,
    score: i32,
}

pub struct MoveSorter{
    piece_scores: HashMap<Piece, i32>,
    mvv_lva_scores: HashMap<(Piece, Piece), i32>,
}

impl MoveSorter {
    pub fn new() -> Self {
        let mut piece_scores: HashMap<Piece, i32> = HashMap::new();
        piece_scores.insert(Piece::Pawn, 100);
        piece_scores.insert(Piece::Knight, 300);
        piece_scores.insert(Piece::Bishop, 325);
        piece_scores.insert(Piece::Rook, 500);
        piece_scores.insert(Piece::Queen, 900);
        piece_scores.insert(Piece::King, 5000);

        let mut mvv_lva_scores: HashMap<(Piece, Piece), i32> = HashMap::new();

        for attacker in piece_scores.keys() {
            for victim in piece_scores.keys() {
                let score = piece_scores[victim] * 10 - piece_scores[attacker];
                mvv_lva_scores.insert((*attacker, *victim), score);
            }
        }
        MoveSorter {
            piece_scores,
            mvv_lva_scores,
        }
    }


    pub fn sort_moves(&self, board: &Board, moves: &mut Vec<ChessMove>) {
        let mut scored_moves: Vec<ScoredMove> = moves.iter()
            .map(|mv| {
                let score = if mv.is_checkmate {
                    100_000
                } else {
                    if mv.is_capture() {
                    let attacker = board.piece_list[mv.from as usize];
                    let victim = board.piece_list[mv.to as usize];
                    self.mvv_lva_scores.get(&(attacker, victim))
                                    .cloned().unwrap_or(0)
                    } else {
                        if mv.is_promotion() {
                            let piece = mv.promotion.unwrap();
                            self.piece_scores.get(&piece)
                                        .cloned().unwrap_or(0)
                        } else {
                            if mv.is_check {
                                500
                            }
                            else {
                                0
                            }
                        }
                    }
                };
                ScoredMove { mv: *mv, score: score }
            })
            .collect();

        scored_moves.sort_by(|a, b| b.score.cmp(&a.score));

        *moves = scored_moves.into_iter().map(|sm| sm.mv).collect()
    }
}