use super::{chess_move::ChessMove, definitions::{Piece, Side, Square, MAX_GAME_MOVES}, game_state::GameState};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RecordedMove {
    pub mv: ChessMove,
    pub prev_state: GameState,
    pub captured_piece: Option<(Piece, Side, Square)>,
}

impl RecordedMove {
    pub fn new_empty() -> Self {
        RecordedMove {
            mv: ChessMove::quiet(Square::A1, Square::A1),
            prev_state: GameState::new(),
            captured_piece: None,
        }
    }

    pub fn new(mv: ChessMove, prev_state: GameState, captured_piece: Option<(Piece, Side, Square)>) -> Self {
        RecordedMove {
            mv,
            prev_state,
            captured_piece,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GameHistory {
    list: [RecordedMove; MAX_GAME_MOVES],
    count: usize,
}


impl GameHistory {
    pub fn new() -> Self {
        GameHistory {
            list: [RecordedMove::new_empty(); MAX_GAME_MOVES],
            count: 0,
        }
    }


    pub fn push(&mut self, new_recorded_move: RecordedMove) {
        self.list[self.count] = new_recorded_move;
        self.count += 1;
    }


    pub fn pop(&mut self) -> Option<RecordedMove> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.list[self.count])
        } else {
            None
        }
    }


    pub fn get_ref(&self, index: usize) -> &RecordedMove {
        &self.list[index]
    }


    pub fn len(&self) -> usize {
        self.count
    }


    pub fn clear(&mut self) {
        self.count = 0;
    }
}