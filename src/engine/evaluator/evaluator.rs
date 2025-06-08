use crate::engine::{board::board::Board};

pub trait Evaluator {
    fn evaluate_board(&mut self, board: &Board) -> f32;
}
