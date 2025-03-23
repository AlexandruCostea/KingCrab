mod engine;
 
use std::collections::HashSet;

use engine::board::{definitions::ZobristKey, zobrist::ZobristKeys};
 
 fn main() {
    let a = ZobristKeys::new();
    let mut set: HashSet<ZobristKey> = std::collections::HashSet::new();

    for side in a.piece_keys.iter() {
        for piece in side.iter() {
            for square in piece.iter() {
                if set.contains(square) {
                    println!("Collision detected!");
                    return;
                }
                set.insert(*square);
            }
        }
    }

    for castling in a.castling_keys.iter() {
        if set.contains(castling) {
            println!("Collision detected!");
            return;
        }
        set.insert(*castling);
    }

    for side in a.side_keys.iter() {
        if set.contains(side) {
            println!("Collision detected!");
            return;
        }
        set.insert(*side);
    }

    for en_passant in a.en_passant_keys.iter() {
        if set.contains(en_passant) {
            println!("Collision detected!");
            return;
        }
        set.insert(*en_passant);
    }

    println!("No collisions detected!");
}
