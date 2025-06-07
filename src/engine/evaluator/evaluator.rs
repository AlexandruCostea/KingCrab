use crate::engine::{board::board::Board};

pub trait Evaluator {
    fn evaluate_board(&self, board: &Board) -> f32;
}
