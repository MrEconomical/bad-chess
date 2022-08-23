// Imports

use crate::board::{ SIZE, CASTLE_COLUMNS };
use crate::board::Side::{ self, White, Black };
use crate::board::{ Square, Point };
use crate::board::PieceType::{ Pawn, Knight, Bishop, Rook, Queen };
use crate::board::Move;
use crate::board::CastleDirection;

// Move parameters

const KNIGHT_MOVES: [(i32, i32); 8] = [(2, 1), (-2, 1), (2, -1), (-2, -1), (1, 2), (-1, 2), (1, -2), (-1, -2)];
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
const ROOK_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const KING_MOVES: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

// Get pseudolegal pawn moves for pawn on board

#[allow(clippy::collapsible_else_if)]
pub fn get_pawn_moves(
    side: Side,
    row: usize,
    col: usize,
    board: &[[Square; SIZE]; SIZE],
    pawn_double_moved: Option<Point>
) -> Vec<Point> {
    // Check forward moves

    let mut moves = vec![];
    if side == White {
        if board[row + 1][col] == Square::Empty {
            moves.push(Point(row + 1, col));
            if row == 1 && board[row + 2][col] == Square::Empty {
                moves.push(Point(row + 2, col));
            }
        }
    } else {
        if board[row - 1][col] == Square::Empty {
            moves.push(Point(row - 1, col));
            if row == SIZE - 2 && board[row - 2][col] == Square::Empty {
                moves.push(Point(row - 2, col));
            }
        }
    }

    // Check diagonal captures

    if col > 0 {
        if side == White {
            if let Square::Full(piece) = &board[row + 1][col - 1] {
                if piece.side == Black {
                    moves.push(Point(row + 1, col - 1));
                }
            }
        } else {
            if let Square::Full(piece) = &board[row - 1][col - 1] {
                if piece.side == White {
                    moves.push(Point(row - 1, col - 1));
                }
            }
        }
    }
    if col < SIZE - 1 {
        if side == White {
            if let Square::Full(piece) = &board[row + 1][col + 1] {
                if piece.side == Black {
                    moves.push(Point(row + 1, col + 1));
                }
            }
        } else {
            if let Square::Full(piece) = &board[row - 1][col + 1] {
                if piece.side == White {
                    moves.push(Point(row - 1, col + 1));
                }
            }
        }
    }

    // Check en passant moves

    if let Some(point) = pawn_double_moved {
        if row == point.0 {
            if col == point.1 + 1 {
                if side == White && board[row + 1][col - 1] == Square::Empty {
                    moves.push(Point(row + 1, col - 1));
                } else if board[row - 1][col - 1] == Square::Empty {
                    moves.push(Point(row - 1, col - 1));
                }
            } else if col + 1 == point.1 {
                if side == White && board[row + 1][col + 1] == Square::Empty {
                    moves.push(Point(row + 1, col + 1));
                } else if board[row + 1][col - 1] == Square::Empty {
                    moves.push(Point(row + 1, col - 1));
                }
            }
        }
    }

    moves
}

// Get pseudolegal knight moves for knight on board

pub fn get_knight_moves(
    side: Side,
    row: usize,
    col: usize,
    board: &[[Square; SIZE]; SIZE]
) -> Vec<Point> {
    let mut moves = vec![];
    let size = SIZE as i32;

    for dir in KNIGHT_MOVES {
        let square = (row as i32 + dir.0, col as i32 + dir.1);
        if square.0 >= 0 && square.0 < size && square.1 >= 0 && square.1 < size {
            let mov = Point(square.0 as usize, square.1 as usize);
            if let Square::Full(piece) = board[mov.0][mov.1] {
                if piece.side != side {
                    moves.push(mov);
                }
                continue;
            }
            moves.push(mov);
        }
    }

    moves
}

// Get pseudolegal bishop moves for bishop on board

pub fn get_bishop_moves(
    side: Side,
    row: usize,
    col: usize,
    board: &[[Square; SIZE]; SIZE]
) -> Vec<Point> {
    let mut moves = vec![];
    let size = SIZE as i32;

    for dir in BISHOP_DIRECTIONS {
        let mut square = (row as i32 + dir.0, col as i32 + dir.1);
        while square.0 >= 0 && square.0 < size && square.1 >= 0 && square.1 < size {
            let mov = Point(square.0 as usize, square.1 as usize);
            if let Square::Full(piece) = board[mov.0][mov.1] {
                if piece.side != side {
                    moves.push(mov);
                }
                break;
            }
            moves.push(mov);
            square.0 += dir.0;
            square.1 += dir.1;
        }
    }

    moves
}

// Get pseudolegal rook moves for rook on board

pub fn get_rook_moves(
    side: Side,
    row: usize,
    col: usize,
    board: &[[Square; SIZE]; SIZE]
) -> Vec<Point> {
    let mut moves = vec![];
    let size = SIZE as i32;

    for dir in ROOK_DIRECTIONS {
        let mut square = (row as i32 + dir.0, col as i32 + dir.1);
        while square.0 >= 0 && square.0 < size && square.1 >= 0 && square.1 < size {
            let mov = Point(square.0 as usize, square.1 as usize);
            if let Square::Full(piece) = board[mov.0][mov.1] {
                if piece.side != side {
                    moves.push(mov);
                }
                break;
            }
            moves.push(mov);
            square.0 += dir.0;
            square.1 += dir.1;
        }
    }

    moves
}

// Get pseudolegal queen moves for queen on board

pub fn get_queen_moves(
    side: Side,
    row: usize,
    col: usize,
    board: &[[Square; SIZE]; SIZE]
) -> Vec<Point> {
    let mut moves = vec![];
    let size = SIZE as i32;

    for dir in BISHOP_DIRECTIONS.iter().chain(ROOK_DIRECTIONS.iter()) {
        let mut square = (row as i32 + dir.0, col as i32 + dir.1);
        while square.0 >= 0 && square.0 < size && square.1 >= 0 && square.1 < size {
            let mov = Point(square.0 as usize, square.1 as usize);
            if let Square::Full(piece) = board[mov.0][mov.1] {
                if piece.side != side {
                    moves.push(mov);
                }
                break;
            }
            moves.push(mov);
            square.0 += dir.0;
            square.1 += dir.1;
        }
    }

    moves
}

// Get pseudolegal king moves for king on board

pub fn get_king_moves(
    side: Side,
    row: usize,
    col: usize,
    board: &[[Square; SIZE]; SIZE]
) -> Vec<Point> {
    let mut moves = vec![];
    let size = SIZE as i32;

    for dir in KING_MOVES {
        let square = (row as i32 + dir.0, col as i32 + dir.1);
        if square.0 >= 0 && square.0 < size && square.1 >= 0 && square.1 < size {
            let mov = Point(square.0 as usize, square.1 as usize);
            if let Square::Full(piece) = board[mov.0][mov.1] {
                if piece.side != side {
                    moves.push(mov);
                }
                continue;
            }
            moves.push(mov);
        }
    }

    moves
}

// Filter legal moves from possible moves

pub fn filter_legal_moves(
    side: Side,
    possible: Vec<Move>,
    board: &mut [[Square; SIZE]; SIZE],
    king_pos: Point
) -> impl Iterator<Item=Move> + '_ {
    possible.into_iter().filter(move |mov| {
        // Make move on board

        let replaced = board[mov.to.0][mov.to.1];
        board[mov.to.0][mov.to.1] = board[mov.from.0][mov.from.1];
        board[mov.from.0][mov.from.1] = Square::Empty;

        let legal = !in_check(
            side,
            if mov.from == king_pos { mov.to } else { king_pos },
            board
        );

        // Unmake move on board

        board[mov.from.0][mov.from.1] = board[mov.to.0][mov.to.1];
        board[mov.to.0][mov.to.1] = replaced;

        legal
    })
}

// Check if king at position is in check

pub fn in_check(side: Side, pos: Point, board: &[[Square; SIZE]; SIZE]) -> bool {
    // Check rook and queen horizontal attacks

    let size = SIZE as i32;
    for dir in ROOK_DIRECTIONS {
        let mut square = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        while square.0 >= 0 && square.0 < size && square.1 > 0 && square.1 < size {
            if let Square::Full(piece) = board[square.0 as usize][square.1 as usize] {
                if (piece.typ == Rook || piece.typ == Queen) && piece.side != side {
                    return true;
                }
                break;
            }
            square.0 += dir.0;
            square.1 += dir.1;
        }
    }

    // Check bishop and queen diagonal attacks

    for dir in BISHOP_DIRECTIONS {
        let mut square = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
        while square.0 >= 0 && square.0 < size && square.1 > 0 && square.1 < size {
            if let Square::Full(piece) = board[square.0 as usize][square.1 as usize] {
                if (piece.typ == Bishop || piece.typ == Queen) && piece.side != side {
                    return true;
                }
                break;
            }
            square.0 += dir.0;
            square.1 += dir.1;
        }
    }
    
    // Check knight attacks

    for mov in KNIGHT_MOVES {
        let row = pos.0 as i32 + mov.0;
        let col = pos.1 as i32 + mov.1;
        if row >= 0 && row < size && col >= 0 && col < size {
            if let Square::Full(piece) = board[row as usize][col as usize] {
                if piece.typ == Knight && piece.side != side {
                    return true;
                }
            }
        }
    }

    // Check pawn attacks

    if side == White && pos.0 < SIZE - 1 {
        if pos.1 > 0 {
            if let Square::Full(piece) = board[pos.0 + 1][pos.1 - 1] {
                if piece.typ == Pawn && piece.side == Black {
                    return true;
                }
            }
        }
        if pos.1 < SIZE - 1 {
            if let Square::Full(piece) = board[pos.0 + 1][pos.1 + 1] {
                if piece.typ == Pawn && piece.side == Black {
                    return true;
                }
            }
        }
    } else if pos.0 > 0 {
        if pos.1 > 0 {
            if let Square::Full(piece) = board[pos.0 - 1][pos.1 - 1] {
                if piece.typ == Pawn && piece.side == White {
                    return true;
                }
            }
        }
        if pos.1 < SIZE - 1 {
            if let Square::Full(piece) = board[pos.0 - 1][pos.1 + 1] {
                if piece.typ == Pawn && piece.side == White {
                    return true;
                }
            }
        }
    }

    // Not in check

    false
}

// Check if king can castle in specified direction given castling right

pub fn can_castle(
    side: Side,
    dir: CastleDirection,
    board: &[[Square; SIZE]; SIZE],
    king_pos: Point
) -> bool {
    // Check across columns for pieces or checks

    let cols = match dir {
        CastleDirection::A => CASTLE_COLUMNS.0,
        CastleDirection::H => CASTLE_COLUMNS.1
    };

    for col in cols {
        if col != king_pos.1 {
            if let Square::Full(_) = board[king_pos.0][col] {
                return false;
            }
        }
        if in_check(side, Point(king_pos.0, col), board) {
            return false;
        }
    }

    true
}