// Imports

use crate::board::{ SIZE, PIECE_TYPES };
use crate::board::Side::{ self, White, Black };
use crate::board::{ Square, Point };
use crate::board::Piece;
use crate::board::PieceType::{ Pawn, Knight, Bishop, Rook, Queen, King };
use crate::board::CastleRights;

use std::collections::HashMap;
use nohash_hasher::BuildNoHashHasher;
use rand_chacha::rand_core::{ RngCore, SeedableRng };

// Hash parameters

const ZOBRIST_SEED: u64 = 37811;

// Random number parameters for Zobrist hashes

#[derive(Debug)]
pub struct Zobrist {
    pub piece_table: [[[u64; SIZE]; SIZE]; PIECE_TYPES],
    pub black_turn: u64,
    pub white_castle_rights: (u64, u64),
    pub black_castle_rights: (u64, u64),
    pub en_passant: [u64; SIZE]
}

impl Zobrist {
    // Generate pseudorandom numbers for hashing

    pub fn new() -> Zobrist {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(ZOBRIST_SEED);

        let mut piece_table = [[[0; SIZE]; SIZE]; PIECE_TYPES];
        #[allow(clippy::needless_range_loop)]
        for p in 0..PIECE_TYPES {
            for r in 0..SIZE {
                for c in 0..SIZE {
                    piece_table[p][r][c] = rng.next_u64();
                }
            }
        }

        let mut en_passant = [0; SIZE];
        #[allow(clippy::needless_range_loop)]
        for c in 0..SIZE {
            en_passant[c] = rng.next_u64();
        }

        Zobrist {
            piece_table,
            black_turn: rng.next_u64(),
            white_castle_rights: (rng.next_u64(), rng.next_u64()),
            black_castle_rights: (rng.next_u64(), rng.next_u64()),
            en_passant
        }
    }

    // Get key for piece at square

    pub fn get_piece_key(&self, piece: Piece, square: Point) -> u64 {
        let index = match piece.side {
            White => 0,
            Black => 6
        };
        let index = index + match piece.typ {
            Pawn => 0,
            Knight => 1,
            Bishop => 2,
            Rook => 3,
            Queen => 4,
            King => 5
        };
        self.piece_table[index][square.0][square.1]
    }
}

// Zobrist hash table mapping seen positions

#[derive(Debug)]
pub struct ZobristTable<V> {
    zobrist: Zobrist,
    table: HashMap<u64, V, BuildNoHashHasher<u64>>,
    key: u64
}

impl<V> ZobristTable<V> {
    // Create new table with start capacity and key

    pub fn new(
        capacity: usize,
        side: Side,
        board: &[[Square; SIZE]; SIZE],
        castle_rights: CastleRights,
        pawn_double_moved: Option<Point>
    ) -> ZobristTable<V> {
        // Intialize Zobrist parameters

        let zobrist = Zobrist::new();
        let mut zobrist_key = match side {
            White => 0,
            Black => zobrist.black_turn
        };

        // Update hash with piece positions

        #[allow(clippy::needless_range_loop)]
        for r in 0..SIZE {
            for c in 0..SIZE {
                if let Square::Full(piece) = board[r][c] {
                    zobrist_key ^= zobrist.get_piece_key(piece, Point(r, c));
                }
            }
        }

        // Update hash with en passant moves

        let mut en_passant = None;
        if let Some(point) = pawn_double_moved {
            if point.1 > 0 {
                if let Square::Full(piece) = board[point.0][point.1 - 1] {
                    if piece.typ == Pawn &&
                       ((piece.side == White && board[point.0 + 1][point.1] == Square::Empty) ||
                       (piece.side == Black && board[point.0 - 1][point.1] == Square::Empty)) {
                        en_passant = Some(point.1);
                    }
                }
            }
            if en_passant == None && point.1 < SIZE - 1 {
                if let Square::Full(piece) = board[point.0][point.1 + 1] {
                    if piece.typ == Pawn &&
                       ((piece.side == White && board[point.0 + 1][point.1] == Square::Empty) ||
                       (piece.side == Black && board[point.0 - 1][point.1] == Square::Empty)) {
                        en_passant = Some(point.1);
                    }
                }
            }
        }

        if let Some(col) = en_passant {
            zobrist_key ^= zobrist.en_passant[col];
        }

        // Update hash with castle rights

        if castle_rights.white.0 {
            zobrist_key ^= zobrist.white_castle_rights.0;
        }
        if castle_rights.white.1 {
            zobrist_key ^= zobrist.white_castle_rights.1;
        }
        if castle_rights.black.0 {
            zobrist_key ^= zobrist.black_castle_rights.0;
        }
        if castle_rights.black.1 {
            zobrist_key ^= zobrist.black_castle_rights.1;
        }

        // Initialized table

        ZobristTable {
            zobrist,
            table: HashMap::with_capacity_and_hasher(capacity, BuildNoHashHasher::default()),
            key: zobrist_key
        }
    }
}