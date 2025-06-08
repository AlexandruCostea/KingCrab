use crate::engine::{definitions::ZobristKey, move_generator::chess_move::ChessMove};

#[derive(Clone, Copy)]
pub enum Bound {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy)]
pub struct TranspositionTableEntry {
    pub zobrist: ZobristKey,
    pub depth: u8,
    pub score: f32,
    pub flag: Bound,
    pub best_move: Option<ChessMove>,

}

pub struct TranspositionTable {
    entries: Vec<Option<TranspositionTableEntry>>,
    mask: usize, // for fast indexing if size is a large power of two
}

impl TranspositionTable {
    pub fn new(size_bits: usize) -> Self {
        let size = 1 << size_bits;
        TranspositionTable {
            entries: vec![None; size],
            mask: size - 1,
        }
    }

    fn index(&self, zobrist: u64) -> usize {
        (zobrist as usize) & self.mask
    }

    pub fn store(&mut self, zobrist: u64, entry: TranspositionTableEntry) {
        let idx = self.index(zobrist);
        let replace = match self.entries[idx] {
            None => true,
            Some(existing) => entry.depth >= existing.depth,
        };
        if replace {
            self.entries[idx] = Some(entry);
        }
    }

    pub fn retrieve(&self, zobrist: u64) -> Option<TranspositionTableEntry> {
        let idx = self.index(zobrist);
        self.entries[idx].filter(|e| e.zobrist == zobrist)
    }
}
