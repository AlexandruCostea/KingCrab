use crate::engine::{board::board::Board, definitions::{Bitboard, Castling, SQUARE_BITBOARDS}};
use super::{chess_move::ChessMove, move_sorter::MoveSorter,
    magics::{build_bishop_attack_table, build_rook_attack_table,
        BISHOP_BLOCKER_MASKS, BISHOP_MAGICS, KING_BASE_ATTACKS,
        KNIGHT_BASE_ATTACKS, PAWN_BLACK_ATTACKS, PAWN_WHITE_ATTACKS,
        ROOK_BLOCKER_MASKS, ROOK_MAGICS}};
use crate::engine::definitions::{Side, Square, Piece};


pub struct MoveGenerator {
    move_sorter: MoveSorter,
    rook_attack_table: Vec<Vec<Bitboard>>,
    bishop_attack_table: Vec<Vec<Bitboard>>,
}

impl MoveGenerator {

    pub fn new() -> Self {
        let rook_attack_table = build_rook_attack_table();
        let bishop_attack_table = build_bishop_attack_table();
        let move_sorter = MoveSorter::new();
        MoveGenerator {
            move_sorter,
            rook_attack_table,
            bishop_attack_table,
        }
    }

    pub fn generate_moves(&self, board: &mut Board) -> Vec<ChessMove> {
        let mut moves = self.generate_legal_moves(board);
        for mv in &mut moves {
            board.make_move(*mv);
            mv.is_check = self.is_king_in_check(board, board.get_active_side());
            mv.is_checkmate = if mv.is_check {
                !self.exist_legal_moves(board)
            } else {
                false
            };
            board.undo_move();
        }
        self.move_sorter.sort_moves(board, &mut moves);
        moves
    }

    pub fn generate_legal_moves(&self, board: &mut Board) -> Vec<ChessMove> {
        let pseudo_moves = self.generate_pseudo_legal_moves(board);
        pseudo_moves
            .into_iter()
            .filter(|mv| self.is_legal_move(board, *mv))
            .collect()
    }

    pub fn exist_legal_moves(&self, board: &mut Board) -> bool {
        let pseudo_moves = self.generate_pseudo_legal_moves(board);
        pseudo_moves
            .into_iter()
            .any(|mv| self.is_legal_move(board, mv))
    }

    fn is_legal_move(&self, board: &mut Board, mv: ChessMove) -> bool {
        board.make_move(mv);
        let result = !self.is_king_in_check(&board, board.get_opponent());
        board.undo_move();

        result
    }

    pub fn is_king_in_check(&self, board: &Board, side: Side) -> bool {
        let king_square = board.get_king_square(side);
        let opposing_side = Side::try_from(side as usize ^ 1).unwrap();
        self.is_square_attacked(board, king_square, opposing_side)
    }

    fn is_square_attacked(&self, board: &Board, square: Square, by_side: Side) -> bool {
        let occupancy = board.get_full_occupancy();
        let sq = square as usize;

        let pawns = board.get_pieces(by_side, Piece::Pawn);
        let rooks = board.get_pieces(by_side, Piece::Rook);
        let knights = board.get_pieces(by_side, Piece::Knight);
        let bishops = board.get_pieces(by_side, Piece::Bishop);
        let queens = board.get_pieces(by_side, Piece::Queen);
        let kings = board.get_pieces(by_side, Piece::King);


        let pawn_attackers = match by_side {
            Side::White => PAWN_BLACK_ATTACKS[sq],
            Side::Black => PAWN_WHITE_ATTACKS[sq],
        };

        if pawns & pawn_attackers != 0 {
            return true;
        }


        if knights & KNIGHT_BASE_ATTACKS[sq] != 0 {
            return true;
        }

        if kings & KING_BASE_ATTACKS[sq] != 0 {
            return true;
        }

        
        let rook_like = rooks | queens;
        let rook_mask = ROOK_BLOCKER_MASKS[sq];
        let rook_magic = ROOK_MAGICS[sq];
        let rook_shift = 64 - rook_mask.count_ones();
        let rook_index = ((occupancy & rook_mask)
                                .wrapping_mul(rook_magic)) >> rook_shift;
        let rook_attacks = self.rook_attack_table[sq][rook_index as usize];
        if rook_attacks & rook_like != 0 {
            return true;
        }

        let bishop_like = bishops | queens;
        let bishop_mask = BISHOP_BLOCKER_MASKS[sq];
        let bishop_magic = BISHOP_MAGICS[sq];
        let bishop_shift = 64 - bishop_mask.count_ones();
        let bishop_index = ((occupancy & bishop_mask)
                                .wrapping_mul(bishop_magic)) >> bishop_shift;
        let bishop_attacks = self.bishop_attack_table[sq][bishop_index as usize];
        if bishop_attacks & bishop_like != 0 {
            return true;
        }

        false
    }


    fn generate_pseudo_legal_moves(&self, board: &Board) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        let side = board.game_state.active_side;
        let full_occupancy = board.get_full_occupancy();
        let own_pieces = board.get_side_occupancy(side);
        let enemy_pieces = full_occupancy & !own_pieces;

        for i in 0..64 {
            let square_bitboard = SQUARE_BITBOARDS[i];
            if own_pieces & square_bitboard == 0 {
                continue;
            }

            let from_square = Square::try_from(i).unwrap();
            let piece = board.piece_list[from_square as usize];

            match piece {
                Piece::Pawn => {
                    let mut pawn_moves = self.generate_pawn_moves(
                                                            board, i, side,
                                                            full_occupancy, enemy_pieces);
                    moves.append(&mut pawn_moves);
                },
                Piece::Knight => {
                    let mut knight_moves = self.generate_knight_moves(
                                                        i, own_pieces, enemy_pieces);
                    moves.append(&mut knight_moves);
                },
                Piece::Bishop => {
                    let mut bishop_moves = self.generate_bishop_moves(
                                                            board, i, full_occupancy,
                                                            enemy_pieces, Piece::Bishop);
                    moves.append(&mut bishop_moves);
                },
                Piece::Rook => {
                    let mut rook_moves = self.generate_rook_moves(
                                                            board, i, full_occupancy,
                                                            enemy_pieces, Piece::Rook);
                    moves.append(&mut rook_moves);
                },
                Piece::Queen => {
                    let mut rook_like_moves = self.generate_rook_moves(
                                                                board, i, full_occupancy,
                                                                enemy_pieces, Piece::Queen);
                    let mut bishop_like_moves = self.generate_bishop_moves(
                                                                board, i, full_occupancy,
                                                                enemy_pieces, Piece::Queen);

                    moves.append(&mut rook_like_moves);
                    moves.append(&mut bishop_like_moves);
                },
                Piece::King => {
                    let mut king_moves = self.generate_king_moves(
                                                            board, i, side,
                                                            own_pieces, enemy_pieces);
                    moves.append(&mut king_moves);
                }
                Piece::None => unreachable!(),
            }
        }

        moves
    }

    fn generate_pawn_moves(&self, board: &Board, from: usize, side: Side,
        full_occupancy: Bitboard, enemy_pieces: Bitboard) -> Vec<ChessMove> {
        let mut pawn_moves = Vec::new();
        let from_signed = from as isize;
        let square = Square::try_from(from).unwrap();
        let (single_push, double_push, promotion,
            double, left_capture, right_capture) = 
            if side == Side::White {
            (   
                from_signed + 8,
                from_signed + 16,
                vec![Square::A7, Square::B7, Square::C7, Square::D7, Square::E7,
                Square::F7, Square::G7, Square::H7],
                vec![Square::A2, Square::B2, Square::C2, Square::D2, Square::E2,
                Square::F2, Square::G2, Square::H2],
                from_signed + 7, from_signed + 9
            )
        } else {
            (
                from_signed - 8, from_signed - 16,
                vec![Square::A2, Square::B2, Square::C2, Square::D2, Square::E2,
                Square::F2, Square::G2, Square::H2],
                vec![Square::A7, Square::B7, Square::C7, Square::D7, Square::E7,
                Square::F7, Square::G7, Square::H7],
                from_signed - 7, from_signed - 9
            )
        };
        let single_push_square = match single_push < 0 {
            false => Square::try_from(single_push as usize),
            true => Square::try_from(64)
        };

        let double_push_square = match double_push < 0 {
            false => Square::try_from(double_push as usize),
            true => Square::try_from(64)
        };
        
    
        let single_push = match single_push_square {
            Ok(sq) => Some(SQUARE_BITBOARDS[sq as usize]),
            Err(_) => None,
        };
        let double_push = match double_push_square {
            Ok(sq) => Some(SQUARE_BITBOARDS[sq as usize]),
            Err(_) => None,
        };

        let left_capture_square = match left_capture < 0 {
            false => Square::try_from(left_capture as usize),
            true => Square::try_from(64)
        };
        let right_capture_square = match right_capture < 0 {
            false => Square::try_from(right_capture as usize),
            true => Square::try_from(64)
        };

        let left_capture = match left_capture_square {
            Ok(sq) => Some(SQUARE_BITBOARDS[sq as usize]),
            Err(_) => None,
        };
        let right_capture = match right_capture_square {
            Ok(sq) => Some(SQUARE_BITBOARDS[sq as usize]),
            Err(_) => None,
        };
        let left_edges = vec![Square::A1, Square::A2, Square::A3, Square::A4,
                                        Square::A5, Square::A6, Square::A7, Square::A8];
        let right_edges = vec![Square::H1, Square::H2, Square::H3, Square::H4,
                                        Square::H5, Square::H6, Square::H7, Square::H8];

        // Pawn pushes
        if let Some(single_push_bitboard) = single_push {
            let sp_square = single_push_square.unwrap();
            if single_push_bitboard & full_occupancy == 0 {
                if promotion.contains(&square) {
                    for promotion_piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                        pawn_moves.push(ChessMove::promotion(
                                    square, sp_square,
                                    promotion_piece, false));
                    }
                } else {
                    pawn_moves.push(ChessMove::quiet(Piece::Pawn, square, sp_square));
                }

                if let Some(double_push_bitboard) = double_push {
                    let dp_square = double_push_square.unwrap();
                    if double.contains(&square) && (double_push_bitboard & full_occupancy == 0) {
                        pawn_moves.push(ChessMove::double_pawn_push(square, dp_square));
                    }
                }
            }
        }

        // Pawn captures
        if let Some(left_capture_bitboard) = left_capture {
            let lc_square = left_capture_square.unwrap();
            if !(side == Side::White && left_edges.contains(&square)) && 
                !(side == Side::Black && right_edges.contains(&square)) {
                if left_capture_bitboard & enemy_pieces != 0 {
                    if promotion.contains(&square) {
                        for promotion_piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                            pawn_moves.push(ChessMove::promotion(
                                square, lc_square, promotion_piece, true));
                        }
                    } else {
                        pawn_moves.push(ChessMove::capture(Piece::Pawn, square, lc_square));
                    }
                } else if board.get_ep_square().is_some() {
                    let ep_square = board.get_ep_square().unwrap();
                    if ep_square == lc_square {
                        pawn_moves.push(ChessMove::en_passant(square, lc_square));
                    }
                }
            }
        }

        if let Some(right_capture_bitboard) = right_capture {
            let rc_square = right_capture_square.unwrap();
            if !(side == Side::White && right_edges.contains(&square)) &&
                !(side == Side::Black && left_edges.contains(&square)) {
                if right_capture_bitboard & enemy_pieces != 0 {
                    if promotion.contains(&square) {
                        for promotion_piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                            pawn_moves.push(ChessMove::promotion(square, rc_square,
                                                promotion_piece, true));
                        }
                    } else {
                        pawn_moves.push(ChessMove::capture(Piece::Pawn, square, rc_square));
                    }
                } else if board.get_ep_square().is_some() {
                    let ep_square = board.get_ep_square().unwrap();
                    if ep_square == rc_square {
                        pawn_moves.push(ChessMove::en_passant(square, rc_square));
                    }
                }
            }
        }
        
        pawn_moves
    }

    fn generate_knight_moves(&self, from: usize,
        own_pieces: Bitboard, enemy_pieces: Bitboard) -> Vec<ChessMove> {
        let mut knight_moves = Vec::new();
        let square = Square::try_from(from).unwrap();
        let knight_attacks = KNIGHT_BASE_ATTACKS[from];

        for i in 0..64 {
            if knight_attacks & SQUARE_BITBOARDS[i] != 0 {
                let to_square = Square::try_from(i).unwrap();
                if own_pieces & SQUARE_BITBOARDS[i] == 0 {
                    if enemy_pieces & SQUARE_BITBOARDS[i] == 0 {
                        knight_moves.push(ChessMove::quiet(
                            Piece::Knight, square, to_square));
                    } else {
                    knight_moves.push(ChessMove::capture(
                        Piece::Knight, square, to_square));
                    }
                }
            }
        }

        knight_moves
    }

    fn generate_bishop_moves(&self, board: &Board, from: usize,
        full_occupancy: Bitboard, enemy_pieces: Bitboard, piece_type: Piece) -> Vec<ChessMove> {
        let mut bishop_moves = Vec::new();
        let square = Square::try_from(from).unwrap();
        let bishop_mask = BISHOP_BLOCKER_MASKS[from];
        let bishop_magic = BISHOP_MAGICS[from];
        let bishop_shift = 64 - bishop_mask.count_ones();
        let bishop_index = ((full_occupancy & bishop_mask)
                                    .wrapping_mul(bishop_magic)) >> bishop_shift;
        let bishop_attacks = self.bishop_attack_table[from][bishop_index as usize];

        for i in 0..64 {
            if bishop_attacks & SQUARE_BITBOARDS[i] != 0 {
                let to_square = Square::try_from(i).unwrap();
                if board.piece_list[i] == Piece::None {
                    bishop_moves.push(ChessMove::quiet(piece_type, square, to_square));
                } else if enemy_pieces & SQUARE_BITBOARDS[i] != 0 {
                    bishop_moves.push(ChessMove::capture(piece_type, square, to_square));
                }
            }
        }

        bishop_moves
    }

    fn generate_rook_moves(&self, board: &Board, from: usize,
        full_occupancy: Bitboard, enemy_pieces: Bitboard, piece_type: Piece) -> Vec<ChessMove> {
        let mut rook_moves = Vec::new();
        let square = Square::try_from(from).unwrap();
        let rook_mask = ROOK_BLOCKER_MASKS[from];
        let rook_magic = ROOK_MAGICS[from];
        let rook_shift = 64 - rook_mask.count_ones();
        let rook_index = ((full_occupancy & rook_mask)
                                .wrapping_mul(rook_magic)) >> rook_shift;
        let rook_attacks = self.rook_attack_table[from][rook_index as usize];

        for i in 0..64 {
            if rook_attacks & SQUARE_BITBOARDS[i] != 0 {
                let to_square = Square::try_from(i).unwrap();
                if board.piece_list[i] == Piece::None {
                    rook_moves.push(ChessMove::quiet(piece_type, square, to_square));
                } else if enemy_pieces & SQUARE_BITBOARDS[i] != 0 {
                    rook_moves.push(ChessMove::capture(piece_type, square, to_square));
                }
            }
        }

        rook_moves
    }

    fn generate_king_moves(&self, board: &Board, from: usize, side: Side,
        own_pieces: Bitboard, enemy_pieces: Bitboard) -> Vec<ChessMove> {
        let mut king_moves = Vec::new();
        let square = Square::try_from(from).unwrap();
        let king_attacks = KING_BASE_ATTACKS[from];

        // Normal King moves
        for i in 0..64 {
            if king_attacks & SQUARE_BITBOARDS[i] != 0 {
                let to_square = Square::try_from(i).unwrap();
                if own_pieces & SQUARE_BITBOARDS[i] == 0 {
                    if enemy_pieces & SQUARE_BITBOARDS[i] == 0 {
                        king_moves.push(ChessMove::quiet(
                                Piece::King, square, to_square));
                    } else {
                        king_moves.push(ChessMove::capture(
                                Piece::King, square, to_square));
                    }
                }
            }
        }

        // Castling moves
        let castling_rights = board.game_state.castling;
        let (kingisde_flag, kingside_squares, queenside_flag,
            queenside_squares, opponent) = if side == Side::White {
            (
                Castling::WhiteKing as u8,
                vec![Square::E1, Square::F1, Square::G1, Square::H1],
                Castling::WhiteQueen as u8,
                vec![Square::E1, Square::D1, Square::C1, Square::B1, Square::A1],
                Side::Black
            )
        } else {
            (
                Castling::BlackKing as u8,
                vec![Square::E8, Square::F8, Square::G8, Square::H8],
                Castling::BlackQueen as u8,
                vec![Square::E8, Square::D8, Square::C8, Square::B8, Square::A8],
                Side::White
            )
        };

        if castling_rights & kingisde_flag != 0 {
            if board.piece_list[kingside_squares[0] as usize] == Piece::King &&
                board.piece_list[kingside_squares[1] as usize] == Piece::None &&
                board.piece_list[kingside_squares[2] as usize] == Piece::None &&
                board.piece_list[kingside_squares[3] as usize] == Piece::Rook {
                if !self.is_square_attacked(board, kingside_squares[0], opponent) &&
                    !self.is_square_attacked(board, kingside_squares[1], opponent) &&
                    !self.is_square_attacked(board, kingside_squares[2], opponent) {
                    king_moves.push(ChessMove::castle(
                                kingside_squares[0], kingside_squares[2], true));
                }
            }
        }
        if castling_rights & queenside_flag != 0 {
            if board.piece_list[queenside_squares[0] as usize] == Piece::King &&
                board.piece_list[queenside_squares[1] as usize] == Piece::None &&
                board.piece_list[queenside_squares[2] as usize] == Piece::None &&
                board.piece_list[queenside_squares[3] as usize] == Piece::None &&
                board.piece_list[queenside_squares[4] as usize] == Piece::Rook {
                if !self.is_square_attacked(board, queenside_squares[0], opponent) &&
                    !self.is_square_attacked(board, queenside_squares[1], opponent) &&
                    !self.is_square_attacked(board, queenside_squares[2], opponent) {
                    king_moves.push(ChessMove::castle(
                                queenside_squares[0], queenside_squares[2], false));
                }
            }
        }

        king_moves
    }
}
