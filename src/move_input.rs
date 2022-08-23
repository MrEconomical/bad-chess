// Imports

use crate::board::{ SIZE, PROMOTION_PIECES };
use crate::board::Side::{ White, Black };
use crate::board::{ Square, Point };
use crate::board::PieceType::{ self, Pawn, King };
use crate::board::CastleDirection;
use crate::board::Game;
use crate::moves;

use std::io;

// Parsed player move

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct PlayerMove {
    pub piece: PieceType,
    pub from: File,
    pub to: Point,
    pub typ: MoveType
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum File {
    Row(usize),
    Column(usize),
    Any
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum MoveType {
    Move,
    Capture,
    Promotion(PieceType),
    CapturePromotion(PieceType),
    Castle(CastleDirection)
}

// Read and parse player move input from terminal

pub fn get_player_move() -> Result<(String, PlayerMove), String> {
    // Read terminal input

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Err(String::from("Error reading input, please try again"));
    }
    let input = String::from(input.trim());

    // Parse move

    let move_data = parse_move(&input)?;
    Ok((input, move_data))
}

// Parse player input into move data

pub fn parse_move(input: &str) -> Result<PlayerMove, String> {
    // Check input characters

    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 2 {
        return Err(format!("'{input}' isn't a valid move, please try again"));
    }

    // Check castle moves

    if input == "O-O" {
        return Ok(PlayerMove {
            piece: King,
            from: File::Any,
            to: Point(0, 0), // Unused to field
            typ: MoveType::Castle(CastleDirection::H)
        });
    } else if input == "O-O-O" {
        return Ok(PlayerMove {
            piece: King,
            from: File::Any,
            to: Point(0, 0), // Unused to field
            typ: MoveType::Castle(CastleDirection::A)
        });
    }

    // Get board size indexing parameters

    let char_start = 'a' as usize;
    let rows: Vec<char> = (1..SIZE + 1).map(|c| char::from_digit(c as u32, 10).unwrap()).collect();
    let cols: Vec<char> = (char_start..char_start + SIZE).map(|r| r as u8 as char).collect();

    // Check pawn moves

    if cols.contains(&chars[0]) {
        if chars.len() == 2 && rows.contains(&chars[1]) {
            // Pawn forward move

            let col = chars[0] as usize - char_start;
            return Ok(PlayerMove {
                piece: Pawn,
                from: File::Column(col),
                to: Point(
                    char::to_digit(chars[1], 10).unwrap() as usize - 1,
                    col
                ),
                typ: MoveType::Move
            });
        } else if chars.len() == 4 {
            if chars[1] == 'x' && cols.contains(&chars[2]) && rows.contains(&chars[3]) {
                // Pawn diagonal capture

                return Ok(PlayerMove {
                    piece: Pawn,
                    from: File::Column(chars[0] as usize - char_start),
                    to: Point(
                        char::to_digit(chars[3], 10).unwrap() as usize - 1,
                        chars[2] as usize - char_start
                    ),
                    typ: MoveType::Capture
                });
            } else if (chars[1] == char::from_digit(SIZE as u32, 10).unwrap() || chars[1] == '1') && chars[2] == '=' {
                // Pawn promotion

                if let Some(promote) = PieceType::get_type(chars[3]) {
                    if PROMOTION_PIECES.contains(&promote) {
                        let col = chars[0] as usize - char_start;
                        return Ok(PlayerMove {
                            piece: Pawn,
                            from: File::Column(col),
                            to: Point(
                                char::to_digit(chars[1], 10).unwrap() as usize - 1,
                                col
                            ),
                            typ: MoveType::Promotion(promote)
                        });
                    }
                    return Err(format!("'{input}' isn't a valid pawn promotion, please try again"));
                }
                return Err(format!("'{input}' isn't a valid pawn promotion, please try again"));
            }
            return Err(format!("'{input}' isn't a valid pawn capture, please try again"));
        } else if chars.len() == 6 && chars[1] == 'x' && cols.contains(&chars[2]) && rows.contains(&chars[3]) && chars[4] == '=' {
            // Pawn capture promotion

            if let Some(promote) = PieceType::get_type(chars[5]) {
                if PROMOTION_PIECES.contains(&promote) {
                    return Ok(PlayerMove {
                        piece: Pawn,
                        from: File::Column(chars[0] as usize - char_start),
                        to: Point(
                            char::to_digit(chars[3], 10).unwrap() as usize - 1,
                            chars[2] as usize - char_start
                        ),
                        typ: MoveType::CapturePromotion(promote)
                    });
                }
                return Err(format!("'{input}' isn't a valid pawn promotion, please try again"));
            }
            return Err(format!("'{input}' isn't a valid pawn promotion, please try again"));
        }
        return Err(format!("'{input}' isn't a valid pawn move, please try again"));
    }

    // Check piece moves

    if let Some(piece) = PieceType::get_type(chars[0]) {
        if chars.len() == 3 && cols.contains(&chars[1]) && rows.contains(&chars[2]) {
            // Regular piece move

            return Ok(PlayerMove {
                piece,
                from: File::Any,
                to: Point(
                    char::to_digit(chars[2], 10).unwrap() as usize - 1,
                    chars[1] as usize - char_start
                ),
                typ: MoveType::Move
            });
        } else if chars.len() == 4 && cols.contains(&chars[2]) && rows.contains(&chars[3]) {
            // Capture or disambiguating piece move

            let to = Point(
                char::to_digit(chars[3], 10).unwrap() as usize - 1,
                chars[2] as usize - char_start
            );

            if chars[1] == 'x' {
                // Piece capture

                return Ok(PlayerMove {
                    piece,
                    from: File::Any,
                    to,
                    typ: MoveType::Capture
                });
            } else if rows.contains(&chars[1]) || cols.contains(&chars[1]) {
                // Disambiguating piece move

                return Ok(PlayerMove {
                    piece,
                    from: if rows.contains(&chars[1]) {
                        File::Row(char::to_digit(chars[1], 10).unwrap() as usize - 1)
                    } else {
                        File::Column(chars[1] as usize - char_start)
                    },
                    to,
                    typ: MoveType::Move
                });
            }
            return Err(format!("'{input}' isn't a valid {} move, please try again", piece.get_name()));
        } else if chars.len() == 5 && chars[2] == 'x' && cols.contains(&chars[3]) && rows.contains(&chars[4]) {
            // Disambiguating piece capture

            let from = if rows.contains(&chars[1]) {
                File::Row(char::to_digit(chars[1], 10).unwrap() as usize - 1)
            } else if cols.contains(&chars[1]) {
                File::Column(chars[1] as usize - char_start)
            } else {
                return Err(format!("'{input}' isn't a valid {} move, please try again", piece.get_name()));
            };

            return Ok(PlayerMove {
                piece,
                from,
                to: Point(
                    char::to_digit(chars[4], 10).unwrap() as usize - 1,
                    chars[3] as usize - char_start
                ),
                typ: MoveType::Capture
            });
        }
        return Err(format!("'{input}' isn't a valid {} move, please try again", piece.get_name()));
    }

    // Invalid piece specified

    Err(format!("'{input}' isn't a valid move, please try again"))
}

// Validate move input with basic checks

pub fn validate_move(input: &str, move_data: PlayerMove, game: &Game) -> Result<(), String> {
    match move_data.typ {
        MoveType::Move => {
            if game.board[move_data.to.0][move_data.to.1] != Square::Empty {
                return Err(format!("'{input}' isn't a valid move, please try again"));
            }
            if move_data.piece == Pawn && (move_data.to.0 == 0 || move_data.to.0 == SIZE - 1) {
                return Err(format!("'{input}' must be a pawn promotion, please try again"));
            }
        },
        MoveType::Capture => if game.board[move_data.to.0][move_data.to.1] == Square::Empty {
            return Err(format!("'{input}' isn't a valid capture, please try again"));
        },
        MoveType::Promotion(_) => if (game.turn == White && move_data.to.0 < SIZE - 1) || (game.turn == Black && move_data.to.0 > 0) {
            return Err(format!("'{input}' isn't a valid pawn promotion, please try again"));
        },
        MoveType::CapturePromotion(_) => {
            if game.board[move_data.to.0][move_data.to.1] == Square::Empty {
                return Err(format!("'{input}' isn't a valid capture, please try again"));
            }
            if (game.turn == White && move_data.to.0 < SIZE - 1) || (game.turn == Black && move_data.to.0 > 0) {
                return Err(format!("'{input}' isn't a valid pawn promotion, please try again"));
            }
        },
        MoveType::Castle(dir) => {
            if !game.castle_rights.has_right(game.turn, dir) {
                return Err(format!("'{input}' isn't a valid castle move, please try again"));
            }
            if !moves::can_castle(
                game.turn,
                dir,
                &game.board,
                game.king_positions.get_pos(game.turn)
            ) {
                return Err(format!("'{input}' castles through pieces or check, please try again"));
            }
        }
    };

    Ok(())
}