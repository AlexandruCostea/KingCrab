use std::fmt::Display;

use crate::engine::definitions::{Square, Piece};

use bitflags::bitflags;


bitflags! {
    #[derive(Clone, Copy)]
    pub struct ChessMoveFlags: u8 {
        const QUIET             = 1;
        const CAPTURE           = 2;
        const DOUBLE_PAWN_PUSH  = 4;
        const KING_CASTLE       = 8;
        const QUEEN_CASTLE      = 16;
        const EN_PASSANT        = 32;
        const PROMOTION         = 64;
    }
}

#[derive(Clone, Copy)]
pub struct ChessMove {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub flags: ChessMoveFlags,
}

impl ChessMove {
    pub fn quiet(piece: Piece, from: Square, to: Square) -> Self {
        Self {
            piece,
            from,
            to,
            promotion: None,
            is_check: false,
            is_checkmate: false,
            flags: ChessMoveFlags::QUIET,
        }
    }

    pub fn capture(piece: Piece, from: Square, to: Square) -> Self {
        Self {
            piece,
            from,
            to,
            promotion: None,
            is_check: false,
            is_checkmate: false,
            flags: ChessMoveFlags::CAPTURE,
        }
    }

    pub fn double_pawn_push(from: Square, to: Square) -> Self {
        let piece = Piece::Pawn;
        Self {
            piece,
            from,
            to,
            promotion: None,
            is_check: false,
            is_checkmate: false,
            flags: ChessMoveFlags::DOUBLE_PAWN_PUSH,
        }
    }

    pub fn castle(from: Square, to: Square, king_side: bool) -> Self {
        let piece = Piece::King;
        Self {
            piece,
            from,
            to,
            promotion: None,
            is_check: false,
            is_checkmate: false,
            flags: if king_side {
                ChessMoveFlags::KING_CASTLE
            } else {
                ChessMoveFlags::QUEEN_CASTLE
            },
        }
    }

    pub fn en_passant(from: Square, to: Square) -> Self {
        let piece = Piece::Pawn;
        Self {
            piece,
            from,
            to,
            promotion: None,
            is_check: false,
            is_checkmate: false,
            flags: ChessMoveFlags::EN_PASSANT | ChessMoveFlags::CAPTURE,
        }
    }

    pub fn promotion(from: Square, to: Square, promotion: Piece, is_capture: bool) -> Self {
        let piece = Piece::Pawn;
        let mut flags = ChessMoveFlags::PROMOTION;
        if is_capture {
            flags |= ChessMoveFlags::CAPTURE;
        }
        Self {
            piece,
            from,
            to,
            promotion: Some(promotion),
            is_check: false,
            is_checkmate: false,
            flags,
        }
    }

    pub fn is_quiet(&self) -> bool {
        self.flags.contains(ChessMoveFlags::QUIET)
    }

    pub fn is_capture(&self) -> bool {
        self.flags.contains(ChessMoveFlags::CAPTURE)
    }

    pub fn is_double_pawn_push(&self) -> bool {
        self.flags.contains(ChessMoveFlags::DOUBLE_PAWN_PUSH)
    }

    pub fn is_king_castling(&self) -> bool {
        self.flags.contains(ChessMoveFlags::KING_CASTLE)
    }

    pub fn is_queen_castling(&self) -> bool {
        self.flags.contains(ChessMoveFlags::QUEEN_CASTLE)
    }

    pub fn is_en_passant(&self) -> bool {
        self.flags.contains(ChessMoveFlags::EN_PASSANT)
    }

    pub fn is_promotion(&self) -> bool {
        self.flags.contains(ChessMoveFlags::PROMOTION)
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut move_str= String::new();
        if self.is_king_castling() {
            move_str = "0-0".to_string();
        } else if self.is_queen_castling() {
            move_str = "0-0-0".to_string();
        } else if self.is_quiet() {
            if self.piece == Piece::Pawn {
                move_str = format!("{}{}", self.from, self.to);
            } else {
                move_str = format!("{}{}{}", self.piece, self.from, self.to);
            }
        } else if self.is_double_pawn_push() {
            move_str = format!("{}{}", self.from, self.to);
        } else if self.is_en_passant() {
            move_str = format!("{}x{} e.p.", self.from, self.to);
        } else if self.is_promotion() {
            if let Some(promotion_piece) = self.promotion {
                if self.is_capture() {
                    move_str = format!("{}x{}={}", self.from, self.to, promotion_piece);
                } else {
                    move_str = format!("{}{}={}", self.from, self.to, promotion_piece);
                }
            } else {
                move_str = format!("{}{}", self.from, self.to);
            }
        } else if self.is_capture() {
            if self.piece == Piece::Pawn {
                move_str = format!("{}x{}", self.from, self.to);
            } else {
                move_str = format!("{}{}x{}", self.piece, self.from, self.to);
            }
        }

        if self.is_checkmate {
            move_str.push('#');
        } else if self.is_check {
            move_str.push('+');
        }
        write!(f, "{}", move_str)?;
        Ok(())
    }
}