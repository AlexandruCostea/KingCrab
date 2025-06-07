extern crate num_enum;

use std::{fmt::Display, str::FromStr};
use num_enum::TryFromPrimitive;



// Type Aliases and Constants

pub type ZobristKey = u64;
pub type Bitboard = u64;

pub const FILE_BITBOARDS: [Bitboard; NrOf::FILES] = init_file_bitboards();
pub const RANK_BITBOARDS: [Bitboard; NrOf::RANKS] = init_rank_bitboards();

pub const SQUARE_BITBOARDS: [Bitboard; NrOf::SQUARES] = init_square_bitboards();

pub const MAX_GAME_MOVES: usize = 1024;
pub const HALF_MOVE_MAX: u8 = 100;

pub const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const MAX_POSITION_SCORE: f32 = 100000.0;
pub const MIN_POSITION_SCORE: f32 = -100000.0;


// Chess Elments

pub struct NrOf;

impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SIDES: usize = 2;
    pub const SQUARES: usize = 64;
    pub const CASTLING_PERMISSIONS: usize = 16;
    pub const RANKS: usize = 8;
    pub const FILES: usize = 8;
}


#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Debug, TryFromPrimitive)]
pub enum Side {
    White = 0,
    Black = 1
}


#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Debug, TryFromPrimitive, Hash, Eq)]
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
#[derive(Clone, Copy, PartialEq, TryFromPrimitive)]
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


#[repr(usize)]
pub enum Rank {
    R8 = 7,
}


#[repr(usize)]
pub enum File {
    A = 0,
}


#[repr(usize)]
pub enum Castling {
    WhiteKing = 1,
    WhiteQueen = 2,
    BlackKing = 4,
    BlackQueen = 8,
}


impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "A1" => Ok(Square::A1), "B1" => Ok(Square::B1), "C1" => Ok(Square::C1),
            "D1" => Ok(Square::D1), "E1" => Ok(Square::E1), "F1" => Ok(Square::F1),
            "G1" => Ok(Square::G1), "H1" => Ok(Square::H1),

            "A2" => Ok(Square::A2), "B2" => Ok(Square::B2), "C2" => Ok(Square::C2),
            "D2" => Ok(Square::D2), "E2" => Ok(Square::E2), "F2" => Ok(Square::F2),
            "G2" => Ok(Square::G2), "H2" => Ok(Square::H2),

            "A3" => Ok(Square::A3), "B3" => Ok(Square::B3), "C3" => Ok(Square::C3),
            "D3" => Ok(Square::D3), "E3" => Ok(Square::E3), "F3" => Ok(Square::F3),
            "G3" => Ok(Square::G3), "H3" => Ok(Square::H3),

            "A4" => Ok(Square::A4), "B4" => Ok(Square::B4), "C4" => Ok(Square::C4),
            "D4" => Ok(Square::D4), "E4" => Ok(Square::E4), "F4" => Ok(Square::F4),
            "G4" => Ok(Square::G4), "H4" => Ok(Square::H4),

            "A5" => Ok(Square::A5), "B5" => Ok(Square::B5), "C5" => Ok(Square::C5),
            "D5" => Ok(Square::D5), "E5" => Ok(Square::E5), "F5" => Ok(Square::F5),
            "G5" => Ok(Square::G5), "H5" => Ok(Square::H5),

            "A6" => Ok(Square::A6), "B6" => Ok(Square::B6), "C6" => Ok(Square::C6),
            "D6" => Ok(Square::D6), "E6" => Ok(Square::E6), "F6" => Ok(Square::F6),
            "G6" => Ok(Square::G6), "H6" => Ok(Square::H6),

            "A7" => Ok(Square::A7), "B7" => Ok(Square::B7), "C7" => Ok(Square::C7),
            "D7" => Ok(Square::D7), "E7" => Ok(Square::E7), "F7" => Ok(Square::F7),
            "G7" => Ok(Square::G7), "H7" => Ok(Square::H7),

            "A8" => Ok(Square::A8), "B8" => Ok(Square::B8), "C8" => Ok(Square::C8),
            "D8" => Ok(Square::D8), "E8" => Ok(Square::E8), "F8" => Ok(Square::F8),
            "G8" => Ok(Square::G8), "H8" => Ok(Square::H8),

            _ => Err(()),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::A1 => write!(f, "a1"), Square::B1 => write!(f, "b1"), Square::C1 => write!(f, "c1"),
            Square::D1 => write!(f, "d1"), Square::E1 => write!(f, "e1"), Square::F1 => write!(f, "f1"),
            Square::G1 => write!(f, "g1"), Square::H1 => write!(f, "h1"),
            Square::A2 => write!(f, "a2"), Square::B2 => write!(f, "b2"), Square::C2 => write!(f, "c2"),
            Square::D2 => write!(f, "d2"), Square::E2 => write!(f, "e2"), Square::F2 => write!(f, "f2"),
            Square::G2 => write!(f, "g2"), Square::H2 => write!(f, "h2"),
            Square::A3 => write!(f, "a3"), Square::B3 => write!(f, "b3"), Square::C3 => write!(f, "c3"),
            Square::D3 => write!(f, "d3"), Square::E3 => write!(f, "e3"), Square::F3 => write!(f, "f3"),
            Square::G3 => write!(f, "g3"), Square::H3 => write!(f, "h3"),
            Square::A4 => write!(f, "a4"), Square::B4 => write!(f, "b4"), Square::C4 => write!(f, "c4"),
            Square::D4 => write!(f, "d4"), Square::E4 => write!(f, "e4"), Square::F4 => write!(f, "f4"),
            Square::G4 => write!(f, "g4"), Square::H4 => write!(f, "h4"),
            Square::A5 => write!(f, "a5"), Square::B5 => write!(f, "b5"), Square::C5 => write!(f, "c5"),
            Square::D5 => write!(f, "d5"), Square::E5 => write!(f, "e5"), Square::F5 => write!(f, "f5"),
            Square::G5 => write!(f, "g5"), Square::H5 => write!(f, "h5"),
            Square::A6 => write!(f, "a6"), Square::B6 => write!(f, "b6"), Square::C6 => write!(f, "c6"),
            Square::D6 => write!(f, "d6"), Square::E6 => write!(f, "e6"), Square::F6 => write!(f, "f6"),
            Square::G6 => write!(f, "g6"), Square::H6 => write!(f, "h6"),
            Square::A7 => write!(f, "a7"), Square::B7 => write!(f, "b7"), Square::C7 => write!(f, "c7"),
            Square::D7 => write!(f, "d7"), Square::E7 => write!(f, "e7"), Square::F7 => write!(f, "f7"),
            Square::G7 => write!(f, "g7"), Square::H7 => write!(f, "h7"),
            Square::A8 => write!(f, "a8"), Square::B8 => write!(f, "b8"), Square::C8 => write!(f, "c8"),
            Square::D8 => write!(f, "d8"), Square::E8 => write!(f, "e8"), Square::F8 => write!(f, "f8"),
            Square::G8 => write!(f, "g8"), Square::H8 => write!(f, "h8"),
        }
    }
}


impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::King => write!(f, "K"),
            Piece::Queen => write!(f, "Q"),
            Piece::Rook => write!(f, "R"),
            Piece::Bishop => write!(f, "B"),
            Piece::Knight => write!(f, "N"),
            Piece::Pawn => write!(f, "P"),
            Piece::None => write!(f, "None"),
        }
    }
}


const fn init_file_bitboards() -> [Bitboard; NrOf::FILES] {
    const BITBOARD_FILE_A: Bitboard = 0x0101_0101_0101_0101;
    let mut files = [0; NrOf::FILES];
    let mut i = 0;

    while i < (NrOf::FILES) {
        files[i] = BITBOARD_FILE_A << i;
        i += 1;
    }

    files
}

const fn init_rank_bitboards() -> [Bitboard; NrOf::RANKS] {
    pub const BITBOARD_RANK_1: Bitboard = 0xFF;
    let mut ranks = [0; NrOf::RANKS];
    let mut i = 0;

    while i < NrOf::RANKS {
        ranks[i] = BITBOARD_RANK_1 << (i * 8);
        i += 1;
    }

    ranks
}

const fn init_square_bitboards() -> [Bitboard; NrOf::SQUARES] {
    let mut squares = [0; NrOf::SQUARES];
    let mut i = 0;

    while i < NrOf::SQUARES {
        squares[i] = 1u64 << i;
        i += 1;
    }

    squares
}