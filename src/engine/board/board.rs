use std::sync::Arc;
use std::fmt::{self, Display, Formatter};


use super::{definitions::{Bitboard, NrOf, Piece, Side, Square}, game_history::GameHistory, game_state::GameState, zobrist::ZobristKeys};


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
    pub sides: [Bitboard; NrOf::SIDES],
    pub pieces: [[Bitboard; NrOf::PIECE_TYPES]; NrOf::SIDES],
    pub piece_list: [Piece; NrOf::SQUARES],
    pub game_state: GameState,
    pub game_history: GameHistory,
    pub zobrist_keys: Arc<ZobristKeys>,

}


impl Board {

    pub fn new() -> Self {
        Board {
            sides: [0; NrOf::SIDES],
            pieces: [[0; NrOf::PIECE_TYPES]; NrOf::SIDES],
            piece_list: [Piece::None; NrOf::SQUARES],
            game_state: GameState::new(),
            game_history: GameHistory::new(),
            zobrist_keys: Arc::new(ZobristKeys::new()),
        }
    }


    pub fn reset(&mut self) {
        self.sides = [0; NrOf::SIDES];
        self.pieces = [[0; NrOf::PIECE_TYPES]; NrOf::SIDES];
        self.piece_list = [Piece::None; NrOf::SQUARES];
        self.game_state.clear();
        self.game_history.clear();
    }


    pub fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.pieces[side as usize][piece as usize]
    }


    pub fn get_bitboards(&self, side: Side) -> &[Bitboard; NrOf::PIECE_TYPES] {
        &self.pieces[side as usize]
    }


    pub fn occupancy(&self) -> Bitboard {
        self.sides[Side::White as usize] | self.sides[Side::Black as usize]
    }


    pub fn active_side(&self) -> Side {
        self.game_state.active_side
    }


    pub fn opponent(&self) -> Side {
        let opponet = (self.game_state.active_side as usize) ^ 1;
        Side::try_from(opponet).unwrap()
    }


    pub fn king_square(&self, side: Side) -> Square {
        let king_square = self.pieces[side as usize][Piece::King as usize]
            .trailing_zeros() as usize;
        Square::try_from(king_square).unwrap()

    }
}


// Initialization methods
impl Board {

    pub fn init(&mut self) {
        let pieces_per_side_bitboards = self.init_pieces_per_side_bitboards();
        self.sides[Side::White as usize] = pieces_per_side_bitboards.0;
        self.sides[Side::Black as usize] = pieces_per_side_bitboards.1;


        self.init_piece_list();
        self.init_zobrist_key();

        // self.game_state.next_move = Move::new(0);
    }


    fn init_pieces_per_side_bitboards(&self) -> (Bitboard, Bitboard) {
        let mut bitboard_white: Bitboard = 0;
        let mut bitboard_black: Bitboard = 0;

        for (bb_w, bb_b) in self.pieces[Side::White as usize]
            .iter()
            .zip(self.pieces[Side::Black as usize].iter())
        {
            bitboard_white |= *bb_w;
            bitboard_black |= *bb_b;
        }

        (bitboard_white, bitboard_black)
    }


    fn init_zobrist_key(&mut self) {

        self.game_state.zobrist_key = 0;

        let bitboards_white: &[Bitboard] = &self.pieces[Side::White as usize];
        let bitboards_black: &[Bitboard] = &self.pieces[Side::Black as usize];


        for (piece_type, (white, black)) in bitboards_white
            .iter()
            .zip(bitboards_black.iter()).enumerate() {
            // Assume the first iteration; piece_type will be 0 (KING). The
            // following two statements will thus get all the pieces of
            // type "KING" for white and black. (This will obviously only
            // be one king, but with rooks, there will be two in the
            // starting position.)
            let mut white_pieces: Bitboard = *white;
            let mut black_pieces: Bitboard = *black;

            // Iterate through all the piece locations of the current piece
            // type. Get the square the piece is on, and then hash that
            // square/piece combination into the zobrist key.
            while white_pieces > 0 {
                let square: usize = white_pieces.trailing_zeros() as usize;
                self.game_state.zobrist_key ^= self.zobrist_keys
                                                    .piece(Side::White, 
                                                        Piece::try_from(piece_type).unwrap(),
                                                        Square::try_from(square).unwrap());
                white_pieces &= white_pieces - 1;
            }


            while black_pieces > 0 {
                let square = black_pieces.trailing_zeros() as usize;
                self.game_state.zobrist_key ^= self.zobrist_keys
                                                    .piece(Side::Black, 
                                                        Piece::try_from(piece_type).unwrap(),
                                                        Square::try_from(square).unwrap());
                black_pieces &= black_pieces - 1;
            }
        }

        // Hash the castling, active color, and en-passant state into the key.
        self.game_state.zobrist_key ^= self.zobrist_keys.castling(self.game_state.castling);
        self.game_state.zobrist_key ^= self.zobrist_keys.side(self.game_state.active_side);
        self.game_state.zobrist_key ^= self.zobrist_keys.en_passant(self.game_state.en_passant);
    }



    fn init_piece_list(&mut self) {
        let bitboards_white: &[Bitboard] = &self.pieces[Side::White as usize];
        let bitboards_black: &[Bitboard] = &self.pieces[Side::Black as usize];

        self.piece_list = [Piece::None; NrOf::SQUARES];

        for (piece_type, (white, black)) 
            in bitboards_white.iter().zip(bitboards_black.iter()).enumerate() {

            let mut white_pieces: Bitboard = *white;
            let mut black_pieces: Bitboard = *black;

            while white_pieces != 0 {
                let square: usize = white_pieces.trailing_zeros() as usize;
                self.piece_list[square] = Piece::try_from(piece_type).unwrap();
                white_pieces &= white_pieces - 1;
            }

            while black_pieces != 0 {
                let square: usize = black_pieces.trailing_zeros() as usize;
                self.piece_list[square] = Piece::try_from(piece_type).unwrap();
                black_pieces &= black_pieces - 1;
            }
        }
    }
}


impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Board Position:")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let piece = self.piece_list[square_index];
                let symbol = match piece {
                    Piece::Pawn => {
                        if self.sides[Side::White as usize] & (1 << square_index) != 0 {
                            '♟'
                        } else {
                            '♙'
                        }
                    }
                    Piece::Knight => {
                        if self.sides[Side::White as usize] & (1 << square_index) != 0 {
                            '♞'
                        } else {
                            '♘'
                        }
                    }
                    Piece::Bishop => {
                        if self.sides[Side::White as usize] & (1 << square_index) != 0 {
                            '♝'
                        } else {
                            '♗'
                        }
                    }
                    Piece::Rook => {
                        if self.sides[Side::White as usize] & (1 << square_index) != 0 {
                            '♜'
                        } else {
                            '♖'
                        }
                    }
                    Piece::Queen => {
                        if self.sides[Side::White as usize] & (1 << square_index) != 0 {
                            '♛'
                        } else {
                            '♕'
                        }
                    }
                    Piece::King => {
                        if self.sides[Side::White as usize] & (1 << square_index) != 0 {
                            '♚'
                        } else {
                            '♔'
                        }
                    }
                    Piece::None => {
                        if (rank + file) % 2 == 0 {
                            '░'
                        } else {
                            '▓'
                        }
                    }
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, "\nSide to move: {:?}", self.active_side())?;
        writeln!(f, "Castling rights: {:?}", self.game_state.castling)?;
        writeln!(f, "En passant square: {:?}", self.game_state.en_passant)?;
        writeln!(f, "Halfmove clock: {}", self.game_state.half_move_clock)?;
        writeln!(f, "Fullmove number: {}", self.game_state.full_move_number)?;
        writeln!(f, "Zobrist key: {:016x}", self.game_state.zobrist_key)?;

        Ok(())
    }
}
