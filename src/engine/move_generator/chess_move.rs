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
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub flags: ChessMoveFlags,
}

impl ChessMove {
    pub fn quiet(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            promotion: None,
            flags: ChessMoveFlags::QUIET,
        }
    }

    pub fn capture(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            promotion: None,
            flags: ChessMoveFlags::CAPTURE,
        }
    }

    pub fn double_pawn_push(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            promotion: None,
            flags: ChessMoveFlags::DOUBLE_PAWN_PUSH,
        }
    }

    pub fn castle(from: Square, to: Square, king_side: bool) -> Self {
        Self {
            from,
            to,
            promotion: None,
            flags: if king_side {
                ChessMoveFlags::KING_CASTLE
            } else {
                ChessMoveFlags::QUEEN_CASTLE
            },
        }
    }

    pub fn en_passant(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            promotion: None,
            flags: ChessMoveFlags::EN_PASSANT | ChessMoveFlags::CAPTURE,
        }
    }

    pub fn promotion(from: Square, to: Square, promotion: Piece, is_capture: bool) -> Self {
        let mut flags = ChessMoveFlags::PROMOTION;
        if is_capture {
            flags |= ChessMoveFlags::CAPTURE;
        }
        Self {
            from,
            to,
            promotion: Some(promotion),
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
