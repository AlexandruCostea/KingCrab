use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::engine::definitions::{NrOf, Piece, Side, Square, ZobristKey};


type PieceKeys = [[[ZobristKey; NrOf::SQUARES]; NrOf::PIECE_TYPES]; NrOf::SIDES];
type CastlingKeys = [ZobristKey; NrOf::CASTLING_PERMISSIONS];
type SideKeys = [ZobristKey; NrOf::SIDES];
type EnPassantKeys = [ZobristKey; NrOf::SQUARES + 1];

const RNG_SEED: [u8; 32] = [125; 32];


pub struct ZobristKeys {
    pub piece_keys: PieceKeys,
    pub castling_keys: CastlingKeys,
    pub side_keys: SideKeys,
    pub en_passant_keys: EnPassantKeys,
}


impl ZobristKeys {
    pub fn new() -> ZobristKeys {
        let mut rng: ChaChaRng = ChaChaRng::from_seed(RNG_SEED);

        let mut piece_keys: PieceKeys = [[[0; NrOf::SQUARES]; NrOf::PIECE_TYPES]; NrOf::SIDES];
        let mut castling_keys: CastlingKeys = [0; NrOf::CASTLING_PERMISSIONS];
        let mut side_keys: SideKeys = [0; NrOf::SIDES];
        let mut en_passant_keys: EnPassantKeys = [0; NrOf::SQUARES + 1];


        piece_keys
            .iter_mut()
            .for_each(|side| {
                side
                    .iter_mut()
                    .for_each(|piece| {
                        piece
                            .iter_mut()
                            .for_each(|square| {*square = rng.random();});
                    });
            });
        

        castling_keys
            .iter_mut()
            .for_each(|permission| {*permission = rng.random();});


        side_keys
            .iter_mut()
            .for_each(|side| {*side = rng.random();});


        en_passant_keys
            .iter_mut()
            .for_each(|en_passant_square| {*en_passant_square = rng.random();});


        ZobristKeys {
            piece_keys,
            castling_keys,
            side_keys,
            en_passant_keys,
        }
    }


    pub fn piece(&self, side: Side, piece: Piece, square: Square) -> ZobristKey {
        self.piece_keys[side as usize][piece as usize][square as usize]
    }
  
    pub fn castling(&self, castling_permissions: u8) -> ZobristKey {
        self.castling_keys[castling_permissions as usize]
    }
  
    pub fn side(&self, side: Side) -> u64 {
        self.side_keys[side as usize]
    }
  
    pub fn en_passant(&self, en_passant: Option<u8>) -> ZobristKey {
        match en_passant {
            Some(ep) => self.en_passant_keys[ep as usize],
            None => self.en_passant_keys[NrOf::SQUARES],
        }
    }
}