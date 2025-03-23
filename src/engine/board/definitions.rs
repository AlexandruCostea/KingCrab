extern crate num_enum;
use num_enum::TryFromPrimitive;


pub type ZobristKey = u64;
pub type Bitboard = u64;


#[repr(usize)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Side {
    White = 0,
    Black = 1
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TryFromPrimitive)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}


#[repr(usize)]
#[derive(TryFromPrimitive)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}


pub struct NrOf;

impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SIDES: usize = 2;
    pub const SQUARES: usize = 64;
    pub const CASTLING_PERMISSIONS: usize = 16;
}

pub const MAX_GAME_MOVES: usize = 1024;