use std::sync::Arc;
use std::fmt::{self, Display, Formatter};


use crate::engine::move_generator::chess_move::ChessMove;
use crate::engine::definitions::{Castling, FEN_STARTING_POSITION, HALF_MOVE_MAX,
    SQUARE_BITBOARDS, Bitboard, NrOf, Piece, Side, Square};
use super::{fen::{FenError, FenParser}, game_history::{RecordedMove, GameHistory},
    game_state::GameState, zobrist::ZobristKeys};

#[derive(Clone)]
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

    pub fn get_side_occupancy(&self, side: Side) -> Bitboard {
        self.sides[side as usize]
    }

    pub fn get_full_occupancy(&self) -> Bitboard {
        self.sides[Side::White as usize] | self.sides[Side::Black as usize]
    }

    pub fn get_active_side(&self) -> Side {
        self.game_state.active_side
    }

    pub fn get_opponent(&self) -> Side {
        let opponet = (self.game_state.active_side as usize) ^ 1;
        Side::try_from(opponet).unwrap()
    }

    pub fn get_king_square(&self, side: Side) -> Square {
        let king_square = self.pieces[side as usize][Piece::King as usize]
            .trailing_zeros() as usize;
        Square::try_from(king_square).unwrap()

    }

    pub fn get_ep_square(&self) -> Option<Square> {
        match self.game_state.en_passant {
            Some(square) => Some(Square::try_from(square as usize).unwrap()),
            None => None,
        }
    }

    pub fn init(&mut self) {
        let pieces_per_side_bitboards = self.init_pieces_per_side_bitboards();
        self.sides[Side::White as usize] = pieces_per_side_bitboards.0;
        self.sides[Side::Black as usize] = pieces_per_side_bitboards.1;


        self.init_piece_list();
        self.init_zobrist_key();
    }

    pub fn from_fen(&mut self, fen: Option<&str>) -> Result<(), FenError> {
        let fen_string = fen.unwrap_or(FEN_STARTING_POSITION);

        self.reset();

        let mut fen_parser = FenParser::new(
                                            fen_string.to_string(),
                                            self);

        fen_parser.parse()?;


        self.init();

        Ok(())
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

    pub fn make_move(&mut self, chess_move: ChessMove) {
        let prev_state = self.game_state.clone();
        let mut captured_piece = Piece::None;

        if chess_move.is_quiet() {
            let piece = self.piece_list[chess_move.from as usize];
            if piece != Piece::Pawn {
                self.game_state.half_move_clock += 1;
                match piece {
                    Piece::King => {
                        self.clear_castling_rights_for_side(self.get_active_side());
                    },
                    Piece::Rook => {
                        self.clear_castling_rights_for_square(chess_move.from);
                    },
                    _ => (),
                }
            } else {
                self.game_state.half_move_clock = 0;
            }
            self.move_piece(self.get_active_side(),
                     self.piece_list[chess_move.from as usize],
                     chess_move.from, chess_move.to);
            self.clear_ep_square();

        } else if chess_move.is_double_pawn_push() {
            self.move_piece(self.get_active_side(),
                self.piece_list[chess_move.from as usize],
                chess_move.from, chess_move.to);

            self.set_ep_square(match self.get_active_side() {
                Side::White => Square::try_from(chess_move.to as usize - 8).unwrap(),
                Side::Black => Square::try_from(chess_move.to as usize + 8).unwrap(),
            });

            self.game_state.half_move_clock = 0;

        } else if chess_move.is_king_castling() || chess_move.is_queen_castling() {

            let (rook_pos, rook_dest) = match chess_move.to {
                Square::G1 => (Square::H1, Square::F1),
                Square::C1 => (Square::A1, Square::D1),
                Square::G8 => (Square::H8, Square::F8),
                Square::C8 => (Square::A8, Square::D8),
                _ => unreachable!()    
            };

            // Move the king
            self.move_piece(self.get_active_side(),
                self.piece_list[chess_move.from as usize],
                chess_move.from, chess_move.to);

            // Move the rook
            self.move_piece(self.get_active_side(),
                self.piece_list[rook_pos as usize],
                rook_pos, rook_dest);

            self.clear_castling_rights_for_side(self.get_active_side());
            self.game_state.half_move_clock += 1;
            self.clear_ep_square();

        } else if chess_move.is_capture() {
            captured_piece = self.piece_list[chess_move.to as usize];

            match chess_move.is_en_passant() {
                true => {
                    let ep_square = self.game_state.en_passant.unwrap() as usize;
                    let piece_square = match self.get_active_side() {
                        Side::White => ep_square - 8,
                        Side::Black => ep_square + 8,
                    };

                    captured_piece = self.piece_list[piece_square];
                    self.remove_piece(self.get_opponent(),
                    captured_piece, Square::try_from(piece_square).unwrap());
                },
                false => {
                    self.remove_piece(self.get_opponent(),
                    captured_piece, chess_move.to);

                    if captured_piece == Piece::Rook {
                        self.clear_castling_rights_for_square(chess_move.to);
                    }
                }
            }
            
            match chess_move.is_promotion() {
                true => {
                    self.remove_piece(self.get_active_side(),
                    self.piece_list[chess_move.from as usize],
                    chess_move.from);
                    self.place_piece(self.get_active_side(),
                    chess_move.promotion.unwrap(), chess_move.to);
                },
                false => {
                    self.move_piece(self.get_active_side(),
                self.piece_list[chess_move.from as usize],
                    chess_move.from, chess_move.to);
                }
            }

            self.game_state.half_move_clock = 0;
            self.clear_ep_square();

        } else if chess_move.is_promotion() {
            self.remove_piece(self.get_active_side(),
                self.piece_list[chess_move.from as usize],
                chess_move.from);
            self.place_piece(self.get_active_side(),
                chess_move.promotion.unwrap(), chess_move.to);
            self.game_state.half_move_clock = 0;
            self.clear_ep_square();
        } 

        if self.get_active_side() == Side::Black {
            self.game_state.full_move_number += 1;
        }
        
        let captured = if captured_piece != Piece::None {
            let mut captured_square = chess_move.to;
            if chess_move.is_en_passant() {
                captured_square = match self.get_active_side() {
                    Side::White => Square::try_from(chess_move.to as usize - 8).unwrap(),
                    Side::Black => Square::try_from(chess_move.to as usize + 8).unwrap(),
                };  
            }
            Some((captured_piece, self.get_opponent(), captured_square))
        } else {
            None
        };
        self.game_history.push(
            RecordedMove::new(chess_move, prev_state, captured));
        self.switch_active_side();
    }

    pub fn undo_move(&mut self) {
        if let Some(last_move) = self.game_history.pop() {
            let prev_state = last_move.prev_state;
            let prev_moved_piece = self.piece_list[last_move.mv.to as usize];

            if last_move.mv.is_promotion() {
                self.remove_piece(prev_state.active_side, prev_moved_piece, last_move.mv.to);
                self.place_piece(prev_state.active_side, Piece::Pawn, last_move.mv.from);
            } else {
                self.move_piece(prev_state.active_side,
                    prev_moved_piece, last_move.mv.to, last_move.mv.from);
            }

            if last_move.mv.is_queen_castling() || last_move.mv.is_king_castling() {
                let (rook_pos, rook_dest) = match last_move.mv.to {
                    Square::G1 => (Square::H1, Square::F1),
                    Square::C1 => (Square::A1, Square::D1),
                    Square::G8 => (Square::H8, Square::F8),
                    Square::C8 => (Square::A8, Square::D8),
                    _ => unreachable!()
                };
                self.move_piece(prev_state.active_side,
                    self.piece_list[rook_dest as usize], rook_dest, rook_pos);
            }
            if let Some((piece, side, square)) = last_move.captured_piece {
                self.place_piece(side, piece, square);
            }
            self.game_state = prev_state;
        }
    }

    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.pieces[side as usize][piece as usize] ^= SQUARE_BITBOARDS[square as usize];
        self.sides[side as usize] ^= SQUARE_BITBOARDS[square as usize];
        self.piece_list[square as usize] = Piece::None;
        self.game_state.zobrist_key ^= self.zobrist_keys
            .piece(side, piece, square);
    }

    pub fn place_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.pieces[side as usize][piece as usize] |= SQUARE_BITBOARDS[square as usize];
        self.sides[side as usize] |= SQUARE_BITBOARDS[square as usize];
        self.piece_list[square as usize] = piece;
        self.game_state.zobrist_key ^= self.zobrist_keys
            .piece(side, piece, square);
    }

    pub fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.place_piece(side, piece, to);
    }

    pub fn set_ep_square(&mut self, square: Square) {
        self.game_state.zobrist_key ^= self.zobrist_keys
                .en_passant(self.game_state.en_passant);

        self.game_state.en_passant = Some(square as u8);

        self.game_state.zobrist_key ^= self.zobrist_keys
                .en_passant(self.game_state.en_passant);
    }

    pub fn clear_ep_square(&mut self) {
        self.game_state.zobrist_key ^= self.zobrist_keys
                .en_passant(self.game_state.en_passant);

        self.game_state.en_passant = None;

        self.game_state.zobrist_key ^= self.zobrist_keys
                .en_passant(self.game_state.en_passant);
    }

    pub fn switch_active_side(&mut self) {
        self.game_state.zobrist_key ^= self.zobrist_keys
                .side(self.game_state.active_side);

        self.game_state.active_side = self.get_opponent();

        self.game_state.zobrist_key ^= self.zobrist_keys
                .side(self.game_state.active_side);
    }

    pub fn set_castling_rights(&mut self, new_rights: u8) {
        self.game_state.zobrist_key ^= self.zobrist_keys.castling(self.game_state.castling);
        self.game_state.castling = new_rights;
        self.game_state.zobrist_key ^= self.zobrist_keys.castling(self.game_state.castling);
    }

    fn clear_castling_rights_for_square(&mut self, rook_square: Square) {
        let mut new_rights = self.game_state.castling;
        match rook_square {
            Square::A1 => {
                new_rights &= !(Castling::WhiteQueen as u8);
            },
            Square::H1 => {
                new_rights &= !(Castling::WhiteKing as u8);
            },
            Square::A8 => {
                new_rights &= !(Castling::BlackQueen as u8);
            },
            Square::H8 => {
                new_rights &= !(Castling::BlackKing as u8);
            },
            _ => (),
        }
        self.set_castling_rights(new_rights);
    }

    fn clear_castling_rights_for_side(&mut self, side: Side) {
        let mut new_rights = self.game_state.castling;
        match side {
            Side::White => {
                new_rights &= !(Castling::WhiteKing as u8);
                new_rights &= !(Castling::WhiteQueen as u8);
            },
            Side::Black => {
                new_rights &= !(Castling::BlackKing as u8);
                new_rights &= !(Castling::BlackQueen as u8);
            },
        }
        self.set_castling_rights(new_rights);
    }
    

    pub fn draw_by_fifty_move_rule(&self) -> bool {
        self.game_state.half_move_clock >= HALF_MOVE_MAX
    }

    pub fn draw_by_threefold_repetition(&self) -> bool {
        let mut count = 0;
        for i in (0..self.game_history.len()).rev() {
            let previous_state = self.game_history.get_ref(i).prev_state;
            if previous_state.zobrist_key == self.game_state.zobrist_key {
                count += 1;
            }

            if previous_state.half_move_clock == 0 {
                break;
            }
        }
        count >= 3
    }

    pub fn draw_by_insufficient_material(&self) -> bool {
        let white = self.get_bitboards(Side::White);
        let black = self.get_bitboards(Side::Black);

        // Check for mating material: queens, rooks and pawns
        let has_mating_material = white[Piece::Queen as usize] != 0
            || white[Piece::Rook as usize] != 0
            || white[Piece::Pawn as usize] != 0
            || black[Piece::Queen as usize] != 0
            || black[Piece::Rook as usize] != 0
            || black[Piece::Pawn as usize] != 0;

        if has_mating_material {
            return false;
        }

        // Check for insufficient material conditions

        // King vs. King
        let kk = white[Piece::Bishop as usize] == 0
            && white[Piece::Knight as usize] == 0
            && black[Piece::Bishop as usize] == 0
            && black[Piece::Knight as usize] == 0;

        // King & 1 Bishop vs. King
        let kbk = white[Piece::Bishop as usize].count_ones() == 1
            && white[Piece::Knight as usize] == 0
            && black[Piece::Bishop as usize] == 0
            && black[Piece::Knight as usize] == 0;

        // King & 1 Knight vs. King
        let knk = white[Piece::Bishop as usize] == 0
            && white[Piece::Knight as usize].count_ones() == 1
            && black[Piece::Bishop as usize] == 0
            && black[Piece::Knight as usize] == 0;

        // King vs. King & 1 Bishop
        let kkb = white[Piece::Bishop as usize] == 0
            && white[Piece::Knight as usize] == 0
            && black[Piece::Bishop as usize].count_ones() == 1
            && black[Piece::Knight as usize] == 0;

        // King vs. King & 1 Knight
        let kkn = white[Piece::Bishop as usize] == 0
            && white[Piece::Knight as usize] == 0
            && black[Piece::Bishop as usize] == 0
            && black[Piece::Knight as usize].count_ones() == 1;

        // King & 1 Bishop vs. King & 1 Bishop & both bishops on the same color
        let kbkb = white[Piece::Bishop as usize].count_ones() == 1
            && white[Piece::Knight as usize] == 0
            && black[Piece::Bishop as usize].count_ones() == 1
            && black[Piece::Knight as usize] == 0;


        let same_color_sq = if kbkb {
            let white_bishop_square = white[Piece::Bishop as usize].trailing_zeros() as usize;
            let black_bishop_square = black[Piece::Bishop as usize].trailing_zeros() as usize;

            let white_bishop_color = (white_bishop_square / 8 + white_bishop_square % 8) % 2;
            let black_bishop_color = (black_bishop_square / 8 + black_bishop_square % 8) % 2;
            white_bishop_color == black_bishop_color
        } else {
            false
        };
        if kk || kbk || knk || kkb || kkn || (kbkb && same_color_sq) {
            return true;
        }

        false
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

        // writeln!(f, "\nWhite occupancy:")?;
        // for rank in (0..8).rev() {
        //     for file in 0..8 {
        //         let square_index = rank * 8 + file;
        //         if self.sides[Side::White as usize] & (1 << square_index) != 0 {
        //             write!(f, "⬜")?;
        //         } else {
        //             write!(f, "⬛")?;
        //         }
        //     }
        //     writeln!(f)?;
        // }

        // writeln!(f, "\nBlack occupancy:")?;
        // for rank in (0..8).rev() {
        //     for file in 0..8 {
        //         let square_index = rank * 8 + file;
        //         if self.sides[Side::Black as usize] & (1 << square_index) != 0 {
        //             write!(f, "⬜")?;
        //         } else {
        //             write!(f, "⬛")?;
        //         }
        //     }
        //     writeln!(f)?;
        // }
        writeln!(f, "\n White Bitboards")?;

        for i in 0..NrOf::PIECE_TYPES {
            let current_board = self.pieces[0][i];
            // display the bitboard as a 8x8 grid
            for j in (0..8).rev() {
                for k in 0..8 {
                    let square_index = j * 8 + k;
                    if current_board & (1 << square_index) != 0 {
                        write!(f, "⬜")?;
                    } else {
                        write!(f, "⬛")?;
                    }
                }
                writeln!(f)?;
            }
            writeln!(f, "\n")?;
        }

        writeln!(f, "\n Black Bitboards")?;
        for i in 0..NrOf::PIECE_TYPES {
            let current_board = self.pieces[1][i];
            // display the bitboard as a 8x8 grid
            for j in (0..8).rev() {
                for k in 0..8 {
                    let square_index = j * 8 + k;
                    if current_board & (1 << square_index) != 0 {
                        write!(f, "⬜")?;
                    } else {
                        write!(f, "⬛")?;
                    }
                }
                writeln!(f)?;
            }
            writeln!(f, "\n")?;
        }
        writeln!(f, "\nPiece list:")?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let piece = self.piece_list[square_index];
                if piece != Piece::None {
                    writeln!(f, "Rank {}, file {}: {:?}", rank + 1, file + 1, piece)?;
                }
            }
        }
        writeln!(f, "\nSide to move: {:?}", self.get_active_side())?;
        writeln!(f, "Castling rights: {:?}", self.game_state.castling)?;
        writeln!(f, "En passant square: {:?}", self.game_state.en_passant)?;
        writeln!(f, "Halfmove clock: {}", self.game_state.half_move_clock)?;
        writeln!(f, "Fullmove number: {}", self.game_state.full_move_number)?;
        writeln!(f, "Zobrist key: {:016x}", self.game_state.zobrist_key)?;

        Ok(())
    }
}
