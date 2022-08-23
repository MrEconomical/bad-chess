// Imports

use crate::board::{ SIZE, INITIAL_POSITIONS, CASTLED_POSITIONS };
use crate::board::Side::{ self, White, Black };
use crate::board::{ Square, Point };
use crate::board::Piece;
use crate::board::PieceType::{ Pawn, Knight, Bishop, Rook, Queen, King };
use crate::board::Move;
use crate::board::CastleDirection;
use crate::board::Game;
use crate::moves;
use crate::move_input;
use crate::move_input::{ PlayerMove, File, MoveType };

use colored::Colorize;

// Result of game (win, draw, or none)

pub enum GameResult {
    Win(Side),
    Draw(DrawType),
    None
}

pub enum DrawType {
    Repetition,
    Stalemate,
    Material,
    FiftyMove
}

// Implement play game methods on game

impl Game {
    // Get player move input and make move

    pub fn player_move(&mut self) -> Result<(), String> {
        // Display board and get player move data

        self.display();
        println!();
        match self.turn {
            White => println!("White to play:\n"),
            Black => println!("Black to play:\n")
        }
        let (input, move_data) = move_input::get_player_move()?;
        move_input::validate_move(&input, move_data, self)?;

        // Handle castle moves

        if let MoveType::Castle(dir) = move_data.typ {
            let king_pos = self.king_positions.get_pos(self.turn);
            let (rook_pos, king_mov, rook_mov) = match self.turn {
                White => {
                    match dir {
                        CastleDirection::A => (
                            INITIAL_POSITIONS.WHITE_ROOKS.0,
                            CASTLED_POSITIONS.WHITE_KING.0,
                            CASTLED_POSITIONS.WHITE_ROOKS.0
                        ),
                        CastleDirection::H => (
                            INITIAL_POSITIONS.WHITE_ROOKS.1,
                            CASTLED_POSITIONS.WHITE_KING.1,
                            CASTLED_POSITIONS.WHITE_ROOKS.1
                        )
                    }
                },
                Black => {
                    match dir {
                        CastleDirection::A => (
                            INITIAL_POSITIONS.BLACK_ROOKS.0,
                            CASTLED_POSITIONS.BLACK_KING.0,
                            CASTLED_POSITIONS.BLACK_ROOKS.0
                        ),
                        CastleDirection::H => (
                            INITIAL_POSITIONS.BLACK_ROOKS.1,
                            CASTLED_POSITIONS.BLACK_KING.1,
                            CASTLED_POSITIONS.BLACK_ROOKS.1
                        )
                    }
                }
            };

            // Move pieces

            self.board[king_mov.0][king_mov.1] = self.board[king_pos.0][king_pos.1];
            self.board[rook_mov.0][rook_mov.1] = self.board[rook_pos.0][rook_pos.1];
            self.board[king_pos.0][king_pos.1] = Square::Empty;
            self.board[rook_pos.0][rook_pos.1] = Square::Empty;

            // Update king position and castle rights

            match self.turn {
                White => {
                    self.king_positions.white = king_mov;
                    self.castle_rights.white = (false, false);
                },
                Black => {
                    self.king_positions.black = king_mov;
                    self.castle_rights.black = (false, false);
                }
            }

            // Switch active turn

            self.turn = self.turn.get_opposite();
            return Ok(());
        }
        
        // Check if king is in check after possible moves

        let possible = get_possible_moves(move_data, self);
        if possible.is_empty() {
            return Err(format!("'{input}' isn't a valid move, please try again"));
        }

        let legal: Vec<Move> = moves::filter_legal_moves(
            self.turn,
            possible,
            &mut self.board,
            self.king_positions.get_pos(self.turn)
        ).collect();
        if legal.is_empty() {
            return Err(format!("'{input}' leaves the king in check, please try again"));
        } else if legal.len() > 1 {
            return Err(format!("'{input}' is an ambiguous move, please try again"));
        }

        // Make single legal move

        let mov = legal[0];
        match move_data.typ {
            MoveType::Promotion(piece) | MoveType::CapturePromotion(piece) =>
                self.board[mov.to.0][mov.to.1] = Square::full(self.turn, piece),
            _ =>
                self.board[mov.to.0][mov.to.1] = self.board[mov.from.0][mov.from.1]
        }
        self.board[mov.from.0][mov.from.1] = Square::Empty;        

        // Update king position and castle rights

        if move_data.piece == King {
            match self.turn {
                White => {
                    self.king_positions.white = mov.to;
                    self.castle_rights.white = (false, false);
                },
                Black => {
                    self.king_positions.black = mov.to;
                    self.castle_rights.black = (false, false);
                }
            }
        } else if move_data.piece == Rook {
            match self.turn {
                White => {
                    if mov.from == INITIAL_POSITIONS.WHITE_ROOKS.0 {
                        self.castle_rights.white.0 = false;
                    } else if mov.from == INITIAL_POSITIONS.WHITE_ROOKS.1 {
                        self.castle_rights.white.1 = false;
                    }
                },
                Black => {
                    if mov.from == INITIAL_POSITIONS.BLACK_ROOKS.0 {
                        self.castle_rights.black.0 = false;
                    } else if mov.from == INITIAL_POSITIONS.BLACK_ROOKS.1 {
                        self.castle_rights.black.1 = false;
                    }
                }
            }
        }

        // Update last active ply counter

        match move_data.typ {
            MoveType::Move => {
                if move_data.piece == Pawn {
                    self.last_active_ply = 0;
                } else {
                    self.last_active_ply += 1;
                }
            },
            MoveType::Capture | MoveType::Promotion(_) | MoveType::CapturePromotion(_) => self.last_active_ply = 0,
            MoveType::Castle(_) => self.last_active_ply += 1
        }

        // Switch active turn

        self.turn = self.turn.get_opposite();
        Ok(())
    }

    // Get game result (win, draw, or none)

    pub fn get_game_result(&mut self) -> GameResult {
        // Check for 50 move rule (100 plies)

        if self.last_active_ply == 100 {
            return GameResult::Draw(DrawType::FiftyMove);
        }

        // Count legal moves for each side excluding castling

        let mut white_moves = 0;
        let mut black_moves = 0;

        for r in 0..SIZE {
            for c in 0..SIZE {
                if let Square::Full(piece) = self.board[r][c] {
                    // Check for legal moves

                    let possible = get_moves(piece, r, c, self)
                                                  .into_iter()
                                                  .map(|point| Move { from: Point(r, c), to: point })
                                                  .collect();
                    let count = moves::filter_legal_moves(
                        self.turn,
                        possible,
                        &mut self.board,
                        self.king_positions.get_pos(self.turn)
                    ).count();

                    match piece.side {
                        White => white_moves += count,
                        Black => black_moves += count
                    }
                }
            }
        }

        // Checkmate or stalemate with no moves left

        if white_moves == 0 {
            if moves::in_check(White, self.king_positions.get_pos(White), &self.board) {
                return GameResult::Win(Black);
            }
            return GameResult::Draw(DrawType::Stalemate);
        } else if black_moves == 0 {
            if moves::in_check(Black, self.king_positions.get_pos(Black), &self.board) {
                return GameResult::Win(White);
            }
            return GameResult::Draw(DrawType::Stalemate);
        }

        // No result yet

        GameResult::None
    }

    // Print board position to terminal

    pub fn display(&self) {
        // Get row and column ranges from active turn

        let (row_range, col_range): (Vec<usize>, Vec<usize>) = match self.turn {
            White => ((0..SIZE).rev().collect(), (0..SIZE).collect()),
            Black => ((0..SIZE).collect(), (0..SIZE).rev().collect())
        };

        // Print top columns labels

        print!("  ");
        let char_start = 'a' as usize;
        for c in &col_range {
            print!("{} ", (char_start + c) as u8 as char);
        }
        println!();

        // Print board squares and row labels

        let char_start = 'a' as usize;
        for r in &row_range {
            print!("{} ", r + 1);
            for c in &col_range {
                // Color square with piece and background

                let square = format!("{} ", self.board[*r][*c].get_char());
                let square = match self.board[*r][*c] {
                    Square::Full(piece) => match piece.side {
                        White => square.truecolor(255, 255, 255),
                        Black => square.truecolor(120, 70,20)
                    },
                    Square::Empty => square.white()
                };

                if r % 2 == c % 2 {
                    print!("{}", square.on_truecolor(57, 57, 57));
                } else {
                    print!("{}", square.on_truecolor(75, 75, 75));
                }
            }
            println!(" {}", r + 1);
        }

        // Print bottom columns labels

        print!("  ");
        for c in &col_range {
            print!("{} ", (char_start + c) as u8 as char);
        }
        println!();
    }
}

// Generate possible moves given move input

fn get_possible_moves(move_data: PlayerMove, game: &Game) -> Vec<Move> {
    let mut possible = vec![];

    for r in 0..SIZE {
        for c in 0..SIZE {
            // Check row and column

            match move_data.from {
                File::Row(row) => if row != r { continue; },
                File::Column(col) => if col != c { continue; },
                File::Any => ()
            }

            if let Square::Full(piece) = game.board[r][c] {
                if piece.side == game.turn && piece.typ == move_data.piece {
                    // Filter moves by to square

                    for mov in get_moves(piece, r, c, game) {
                        if mov == move_data.to {
                            possible.push(Move {
                                from: Point(r, c),
                                to: mov
                            });
                        }
                    }
                }
            }
        }
    }

    possible
}

// Generate moves for piece

fn get_moves(piece: Piece, row: usize, col: usize, game: &Game) -> Vec<Point> {
    match piece.typ {
        Pawn => moves::get_pawn_moves(piece.side, row, col, &game.board, game.pawn_double_moved),
        Knight => moves::get_knight_moves(piece.side, row, col, &game.board),
        Bishop => moves::get_bishop_moves(piece.side, row, col, &game.board),
        Rook => moves::get_rook_moves(piece.side, row, col, &game.board),
        Queen => moves::get_queen_moves(piece.side, row, col, &game.board),
        King => moves::get_king_moves(piece.side, row, col, &game.board)
    }
}