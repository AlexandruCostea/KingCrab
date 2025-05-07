use core::fmt;
use std::{fmt::Display, str::FromStr};
use crate::engine::board::definitions::{Square, HALF_MOVE_MAX, MAX_GAME_MOVES};

use super::{board::Board, definitions::{Castling, File, Piece, Rank, Side, SQUARE_BITBOARDS}};

use if_chain::if_chain;


const FEN_PARTS_COUNT: usize = 6;
const PIECE_TYPES: &str = "kqrbnpKQRBNP";
const EP_WHITE: [Square; 8] = [
    Square::A3, Square::B3, Square::C3, Square::D3,
    Square::E3, Square::F3, Square::G3, Square::H3,
];
const EP_BLACK: [Square; 8] = [
    Square::A6, Square::B6, Square::C6, Square::D6,
    Square::E6, Square::F6, Square::G6, Square::H6,
];
const SIDES: &str = "wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const EM_DASH: char = '–';
const SPACE: char = ' ';

const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
pub enum FenError {
    IncorrectLengthError,
    PieceSquarePartError,
    PlaySidePartError,
    CastlingRightsPartError,
    EnPassantPartError,
    HalfMovePartError,
    FullMovePartError,
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::IncorrectLengthError => "Error in FEN string: Must have 6 parts",
            Self::PieceSquarePartError => "Error in FEN pieces and squares part",
            Self::PlaySidePartError => "Error in FEN play side part",
            Self::CastlingRightsPartError => "Error in FEN castling rights part",
            Self::EnPassantPartError => "Error in FEN en passant part",
            Self::HalfMovePartError => "Error in FEN half-move part",
            Self::FullMovePartError => "Error in FEN full-move part",
        };
        write!(f, "{error}")
    }
}

struct FenParser {
    fen_string: String,
    board: Board,
}

impl FenParser {
    fn new(fen_string: String, board: Board) -> Self {
        Self { fen_string, board }
    }
}

pub type FenResult = Result<(), FenError>;
pub type SplitResult = Result<Vec<String>, FenError>;
type FenPartParser = fn(board: &mut Board, part: &str) -> FenResult;

impl Board {
    pub fn from_fen(&mut self, fen: Option<&str>) -> Result<(), FenError> {
        let fen_string = fen.unwrap_or(FEN_STARTING_POSITION);

        let mut temp_board = self.clone();
        temp_board.reset();

        let mut fen_parser = FenParser::new(
                                            fen_string.to_string(),
                                            temp_board);

        let mut temp_board = fen_parser.parse()?;


        temp_board.init();
        *self = temp_board;

        Ok(())
    }
}

impl FenParser {

    fn parse(& mut self) -> Result<Board, FenError> {
        let fen_parts = Self::split_fen_string(&self.fen_string)?;

        let fen_parsers = FenParser::create_part_parsers();

        for (part, parser) in fen_parts.iter().zip(fen_parsers.iter()) {
            let result = parser(&mut self.board, part);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        Ok(self.board.clone())
    }

    fn split_fen_string(fen_string: &str) -> SplitResult {
        const SHORT_LENGTH: usize = 4;

        let mut fen_string: Vec<String> = fen_string
            .replace(EM_DASH, DASH.encode_utf8(&mut [0; 4]))
            .split(SPACE)
            .map(String::from)
            .collect();
    
        if fen_string.len() == SHORT_LENGTH {
            fen_string.append(&mut vec![String::from("0"), String::from("1")]);
        }
    
        if fen_string.len() != FEN_PARTS_COUNT {
            return Err(FenError::IncorrectLengthError);
        }
    
        Ok(fen_string)
    }

    fn create_part_parsers() -> [FenPartParser; FEN_PARTS_COUNT] {
        [
            FenParser::pieces,
            FenParser::color,
            FenParser::castling,
            FenParser::en_passant,
            FenParser::half_move_clock,
            FenParser::full_move_number,
        ]
    }

    fn pieces(board: &mut Board, part: &str) -> FenResult {
        let mut rank = Rank::R8 as u8;
        let mut file = File::A as u8;
    
        for c in part.chars() {
            let square = ((rank * 8) + file) as usize;
            match c {
                'k' => board.pieces[Side::Black as usize][Piece::King as usize] |= SQUARE_BITBOARDS[square],
                'q' => board.pieces[Side::Black as usize][Piece::Queen as usize] |= SQUARE_BITBOARDS[square],
                'r' => board.pieces[Side::Black as usize][Piece::Rook as usize] |= SQUARE_BITBOARDS[square],
                'b' => board.pieces[Side::Black as usize][Piece::Bishop as usize] |= SQUARE_BITBOARDS[square],
                'n' => board.pieces[Side::Black as usize][Piece::Knight as usize] |= SQUARE_BITBOARDS[square],
                'p' => board.pieces[Side::Black as usize][Piece::Pawn as usize] |= SQUARE_BITBOARDS[square],
                'K' => board.pieces[Side::White as usize][Piece::King as usize] |= SQUARE_BITBOARDS[square],
                'Q' => board.pieces[Side::White as usize][Piece::Queen as usize] |= SQUARE_BITBOARDS[square],
                'R' => board.pieces[Side::White as usize][Piece::Rook as usize] |= SQUARE_BITBOARDS[square],
                'B' => board.pieces[Side::White as usize][Piece::Bishop as usize] |= SQUARE_BITBOARDS[square],
                'N' => board.pieces[Side::White as usize][Piece::Knight as usize] |= SQUARE_BITBOARDS[square],
                'P' => board.pieces[Side::White as usize][Piece::Pawn as usize] |= SQUARE_BITBOARDS[square],
                '1'..='8' => {
                    if let Some(x) = c.to_digit(10) {
                        file += x as u8;
                    }
                }
                SPLITTER => {
                    if file != 8 {
                        return Err(FenError::PieceSquarePartError);
                    }
                    rank -= 1;
                    file = 0;
                }
                _ => return Err(FenError::PieceSquarePartError),
            }
    
            if PIECE_TYPES.contains(c) {
                file += 1;
            }
        }
    
        Ok(())
    }

    fn color(board: &mut Board, part: &str) -> FenResult {
        if_chain! {
            if part.len() == 1;
            if let Some(c) = part.chars().next();
            if SIDES.contains(c);
            then {
                match c {
                    'w' => board.game_state.active_side = Side::White,
                    'b' => board.game_state.active_side = Side::Black,
                    _ => (),
                }
                return Ok(());
            }
        }
    
        Err(FenError::PlaySidePartError)
    }

    fn castling(board: &mut Board, part: &str) -> FenResult {

        if (1..=4).contains(&part.len()) {
            for c in part.chars() {
                match c {
                    'K' => board.game_state.castling |= Castling::WhiteKing as u8,
                    'Q' => board.game_state.castling |= Castling::WhiteQueen as u8,
                    'k' => board.game_state.castling |= Castling::BlackKing as u8,
                    'q' => board.game_state.castling |= Castling::BlackQueen as u8,
                    '-' => (),
                    _ => return Err(FenError::CastlingRightsPartError),
                }
            }
            return Ok(());
        }
    
        Err(FenError::CastlingRightsPartError)
    }

    fn en_passant(board: &mut Board, part: &str) -> FenResult {
        if_chain! {
            if part.len() == 1;
            if let Some(x) = part.chars().next();
            if x == DASH;
            then {
                return Ok(());
            }
        }
    
        if part.len() == 2 {
            let square = Square::from_str(part);
            match square {
                Ok(square) if EP_WHITE.contains(&square) || EP_BLACK.contains(&square) => {
                    board.game_state.en_passant = Some(square as u8);
                    return Ok(());
                }
                _ => return Err(FenError::EnPassantPartError),
            };
        }
    
        Err(FenError::EnPassantPartError)
    }

    fn half_move_clock(board: &mut Board, part: &str) -> FenResult {
        if_chain! {
            if (1..=3).contains(&part.len());
            if let Ok(x) = part.parse::<u8>();
            if x <= HALF_MOVE_MAX;
            then {
                board.game_state.half_move_clock = x;
                return Ok(());
            }
        }

        Err(FenError::HalfMovePartError)
    }

    fn full_move_number(board: &mut Board, part: &str) -> FenResult {
        if_chain! {
            if !part.is_empty() && part.len() <= 4;
            if let Ok(x) = part.parse::<u16>();
            if x <= (MAX_GAME_MOVES as u16);
            then {
                board.game_state.full_move_number = x;
                return Ok(());
            }
        }

        Err(FenError::FullMovePartError)
    }


}