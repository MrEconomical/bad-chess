// Imports

use std::ops::Range;

use crate::zobrist::ZobristTable;

// Game parameters

#[allow(non_snake_case)]
pub struct Positions {
    pub WHITE_KING: Point,
    pub BLACK_KING: Point,
    pub WHITE_ROOKS: (Point, Point),
    pub BLACK_ROOKS: (Point, Point)
}

#[allow(non_snake_case)]
pub struct CastledPositions {
    pub WHITE_KING: (Point, Point),
    pub BLACK_KING: (Point, Point),
    pub WHITE_ROOKS: (Point, Point),
    pub BLACK_ROOKS: (Point, Point)
}

pub struct CastleColumns(pub Range<usize>, pub Range<usize>);

pub const SIZE: usize = 8;
pub const PIECE_TYPES: usize = 12;
pub const PROMOTION_PIECES: [PieceType; 4] = [Knight, Bishop, Rook, Queen];
pub const INITIAL_POSITIONS: Positions = Positions {
    WHITE_KING: Point(0, 4),
    BLACK_KING: Point(7, 4),
    WHITE_ROOKS: (Point(0, 0), Point(0, 7)),
    BLACK_ROOKS: (Point(7, 0), Point(7, 7))
};
pub const CASTLED_POSITIONS: CastledPositions = CastledPositions {
    WHITE_KING: (Point(0, 2), Point(0, 6)),
    BLACK_KING: (Point(7, 2), Point(7, 6)),
    WHITE_ROOKS: (Point(0, 3), Point(0, 5)),
    BLACK_ROOKS: (Point(7, 3), Point(7, 5))
};
pub const CASTLE_COLUMNS: CastleColumns = CastleColumns(2..5, 4..7);

// Player side

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
pub enum Side {
    White,
    Black
}

use Side::{ White, Black };

impl Side {
    // Get opposite side

    pub fn get_opposite(&self) -> Side {
        match self {
            White => Black,
            Black => White
        }
    }
}

// Square on chess board

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
pub enum Square {
    Full(Piece),
    Empty
}

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
pub struct Point(pub usize, pub usize);

impl Square {
    // Get full square with piece

    pub fn full(side: Side, typ: PieceType) -> Square {
        Square::Full(
            Piece { side, typ }
        )
    }

    // Get character for square

    pub fn get_char(&self) -> &'static str {
        match self {
            Square::Full(piece) => match piece.side {
                White => piece.get_char(),
                Black => piece.get_char()
            },
            Square::Empty => " "
        }
    }
}

// Chess piece

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
pub struct Piece {
    pub side: Side,
    pub typ: PieceType
}

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

use PieceType::{ Pawn, Knight, Bishop, Rook, Queen, King };

impl Piece {
    // Get character for piece

    pub fn get_char(&self) -> &'static str {
        match self.typ {
            Pawn => "♟︎",
            Knight => "♞",
            Bishop => "♝",
            Rook => "♜",
            Queen => "♛",
            King => "♚"
        }
    }
}

impl PieceType {
    // Get piece type from character

    pub fn get_type(char: char) -> Option<PieceType> {
        match char {
            'P' => Some(Pawn),
            'N' => Some(Knight),
            'B' => Some(Bishop),
            'R' => Some(Rook),
            'Q' => Some(Queen),
            'K' => Some(King),
            _ => None
        }
    }

    // Get piece name for piece type

    pub fn get_name(&self) -> &'static str {
        match self {
            Pawn => "pawn",
            Knight => "knight",
            Bishop => "bishop",
            Rook => "rook",
            Queen => "queen",
            King => "king"
        }
    }
}

// Move between two squares

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Move {
    pub from: Point,
    pub to: Point
}

// King coordinates on board for black and white

#[derive(Debug)]
pub struct KingPositions {
    pub white: Point,
    pub black: Point
}

impl KingPositions {
    // Get king position for side

    pub fn get_pos(&self, side: Side) -> Point {
        match side {
            White => self.white,
            Black => self.black
        }
    }
}

// King castling rights (a file, h file)

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct CastleRights {
    pub white: (bool, bool),
    pub black: (bool, bool)
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum CastleDirection {
    A,
    H
}

impl CastleRights {
    // Get castle right for side and direction

    pub fn has_right(&self, side: Side, dir: CastleDirection) -> bool {
        match side {
            White => match dir {
                CastleDirection::A => self.white.0,
                CastleDirection::H => self.white.1
            },
            Black => match dir {
                CastleDirection::A => self.black.0,
                CastleDirection::H => self.black.1
            }
        }
    }
}

// Chess game state

#[derive(Debug)]
pub struct Game {
    pub turn: Side,                       // Current active player up to move
    pub board: [[Square; SIZE]; SIZE],    // 8x8 chess board of squares
    pub king_positions: KingPositions,    // Position of kings on board
    pub castle_rights: CastleRights,      // Option to castle for both sides
    pub pawn_double_moved: Option<Point>, // Track last double pawn move for en passant
    pub last_active_ply: u32,             // Track distance from last active ply for 50 move rule
    pub zobrist_table: ZobristTable<u32>
}

impl Game {
    // Create new game in starting position

    pub fn new() -> Game {
        let board = [
            [
                Square::full(White, Rook),
                Square::full(White, Knight),
                Square::full(White, Bishop),
                Square::full(White, Queen),
                Square::full(White, King),
                Square::full(White, Bishop),
                Square::full(White, Knight),
                Square::full(White, Rook)
            ],
            [
                Square::full(White, Pawn),
                Square::full(White, Pawn),
                Square::full(White, Pawn),
                Square::full(White, Pawn),
                Square::full(White, Pawn),
                Square::full(White, Pawn),
                Square::full(White, Pawn),
                Square::full(White, Pawn)
            ],
            [Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty],
            [Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty],
            [Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty],
            [Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty, Square::Empty],
            [
                Square::full(Black, Pawn),
                Square::full(Black, Pawn),
                Square::full(Black, Pawn),
                Square::full(Black, Pawn),
                Square::full(Black, Pawn),
                Square::full(Black, Pawn),
                Square::full(Black, Pawn),
                Square::full(Black, Pawn)
            ],
            [
                Square::full(Black, Rook),
                Square::full(Black, Knight),
                Square::full(Black, Bishop),
                Square::full(Black, Queen),
                Square::full(Black, King),
                Square::full(Black, Bishop),
                Square::full(Black, Knight),
                Square::full(Black, Rook)
            ]
        ];
        let castle_rights = CastleRights {
            white: (true, true),
            black: (true, true)
        };

        Game {
            turn: White,
            board,
            king_positions: KingPositions {
                white: INITIAL_POSITIONS.WHITE_KING,
                black: INITIAL_POSITIONS.BLACK_KING
            },
            castle_rights,
            pawn_double_moved: None,
            last_active_ply: 0,
            zobrist_table: ZobristTable::new(16, White, &board, castle_rights, None)
        }
    }
}