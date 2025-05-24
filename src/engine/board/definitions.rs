extern crate num_enum;

use std::str::FromStr;
use num_enum::TryFromPrimitive;


pub type ZobristKey = u64;
pub type Bitboard = u64;

pub const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";



#[repr(usize)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TryFromPrimitive)]
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
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Clone, Copy, Hash)]
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


#[repr(usize)]
#[derive(TryFromPrimitive)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
}

#[repr(usize)]
#[derive(TryFromPrimitive)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

type BitboardFiles = [Bitboard; NrOf::FILES];
type BitboardRanks = [Bitboard; NrOf::RANKS];
type BitboardSquares = [Bitboard; NrOf::SQUARES];


pub const FILE_BITBOARDS: BitboardFiles = init_file_bitboards();
pub const RANK_BITBOARDS: BitboardRanks = init_rank_bitboards();
pub const SQUARE_BITBOARDS: BitboardSquares = init_square_bitboards();

#[repr(usize)]
#[derive(TryFromPrimitive)]
pub enum Castling {
    WhiteKing = 1,
    WhiteQueen = 2,
    BlackKing = 4,
    BlackQueen = 8,
    All = 15,
}

type CastlingPermissionSquares = [u8; NrOf::SQUARES];

const CASTLING_PERMS: CastlingPermissionSquares = castling_permissions_per_square();
const fn castling_permissions_per_square() -> CastlingPermissionSquares {
    // All squares grant full permissions initially.
    // Moving a piece from that square does not affect castling permissions.
    let mut cp: CastlingPermissionSquares = [Castling::All as u8; NrOf::SQUARES];

    // Disable permissions once any rook or king is moved from starting square
    cp[Square::A1 as usize] &= !(Castling::WhiteQueen as u8);
    cp[Square::E1 as usize] &= !(Castling::WhiteKing as u8) & !(Castling::WhiteQueen as u8);
    cp[Square::H1 as usize] &= !(Castling::WhiteKing as u8);
    cp[Square::A8 as usize] &= !(Castling::BlackQueen as u8);
    cp[Square::E8 as usize] &= !(Castling::BlackKing as u8) & !(Castling::BlackQueen as u8);
    cp[Square::H8 as usize] &= !(Castling::BlackKing as u8);

    cp
}


pub struct NrOf;

impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SIDES: usize = 2;
    pub const SQUARES: usize = 64;
    pub const CASTLING_PERMISSIONS: usize = 16;
    pub const RANKS: usize = 8;
    pub const FILES: usize = 8;
}

pub const MAX_GAME_MOVES: usize = 1024;
pub const HALF_MOVE_MAX: u8 = 100;


const fn init_file_bitboards() -> BitboardFiles {
    const BITBOARD_FILE_A: Bitboard = 0x0101_0101_0101_0101;
    let mut files: BitboardFiles = [0; NrOf::FILES];
    let mut i = 0;

    while i < (NrOf::FILES) {
        files[i] = BITBOARD_FILE_A << i;
        i += 1;
    }

    files
}

const fn init_rank_bitboards() -> BitboardRanks {
    pub const BITBOARD_RANK_1: Bitboard = 0xFF;
    let mut ranks = [0; NrOf::RANKS];
    let mut i = 0;

    while i < NrOf::RANKS {
        ranks[i] = BITBOARD_RANK_1 << (i * 8);
        i += 1;
    }

    ranks
}

const fn init_square_bitboards() -> BitboardSquares {
    let mut squares: BitboardSquares = [0; NrOf::SQUARES];
    let mut i = 0;

    while i < NrOf::SQUARES {
        squares[i] = 1u64 << i;
        i += 1;
    }

    squares
}