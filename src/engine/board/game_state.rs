use crate::engine::definitions::{Side, ZobristKey};


#[derive(Clone, Copy)]
pub struct GameState {
    pub active_side: Side,
    pub castling: u8,
    pub half_move_clock: u8,
    pub en_passant: Option<u8>,
    pub full_move_number: u16,
    pub zobrist_key: ZobristKey,
    // pub next_move: Move,
}


impl GameState {
    pub fn new() -> Self {
        GameState {
            active_side: Side::White,
            castling: 0,
            en_passant: None,
            half_move_clock: 0,
            full_move_number: 0,
            zobrist_key: 0,
            // next_move: Move::default(),
        }
    }


    pub fn clear(&mut self) {
        self.active_side = Side::White;
        self.castling = 0;
        self.en_passant = None;
        self.half_move_clock = 0;
        self.full_move_number = 0;
        self.zobrist_key = 0;
    }
}