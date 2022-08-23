// Imports

mod board;
mod moves;
mod zobrist;
mod game;
mod engine;
mod move_input;
mod util;

use crate::board::Side::{ White, Black };
use crate::board::Game;
use crate::game::{ GameResult, DrawType };

use std::io;
use colored::Colorize;

// Run bad chess

fn main() {
    // Run game loop

    loop {
        // Get input game mode

        display_home_screen();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            util::clear_screen();
            println!("{}", "Error reading input, please try again".bold());
            continue;
        }

        let input = input.trim();
        let input: u32 = match input.parse() {
            Ok(value) => {
                if value != 1 && value != 2 {
                    util::clear_screen();
                    println!("{}", format!("'{input}' isn't a valid game mode, please try again").bold());
                    continue;
                }
                value
            },
            Err(_) => {
                util::clear_screen();
                println!("{}", format!("'{input}' isn't a valid game mode, please try again").bold());
                continue;
            }
        };

        // Start new game

        if input == 1 {
            continue;
        } else {
            start_player_game();
        }
    }
}

// Display game selection home screen

fn display_home_screen() {
    println!("
           ,....,
        ,::::::<
       ,::/^\\\"``.
      ,::/, `   e`.
     ,::; |        '.
     ,::|  \\___,-.  c)      .o8                       .o8               oooo
     ;::|     \\   '-'      \"888                      \"888               `888
     ;::|      \\            888oooo.   .oooo.    .oooo888      .ooooo.   888 .oo.    .ooooo.   .oooo.o  .oooo.o
     ;::|   _.=`\\           d88' `88b `P  )88b  d88' `888     d88' `\"Y8  888P\"Y88b  d88' `88b d88(  \"8 d88(  \"8
     `;:|.=` _.=`\\          888   888  .oP\"888  888   888     888        888   888  888ooo888 `\"Y88b.  `\"Y88b.
       '|_.=`   __\\         888   888 d8(  888  888   888     888   .o8  888   888  888    .o o.  )88b o.  )88b
       `\\_..==`` /          `Y8bod8P' `Y888\"\"8o `Y8bod88P\"    `Y8bod8P' o888o o888o `Y8bod8P' 8\"\"888P' 8\"\"888P'
        .'.___.-'.
       /          \\
      ('--......--')
      /'--......--'\\
      `\"--......--\"
    
    1. New game against computer opponent
    2. New two-player game
    ");
}

// Start two player game

fn start_player_game() {
    util::clear_screen();
    let mut game = Game::new();

    loop {
        // Make player move

        println!("Player (black) vs. Player (white)\n");
        let mov = game.player_move();
        util::clear_screen();
        
        if let Err(error) = mov {
            println!("{}\n", error.bold());
            continue;
        }

        // Check for game result

        match game.get_game_result() {
            GameResult::Win(side) => {
                match side {
                    White => println!("{}\n", "White wins by checkmate!".bold()),
                    Black => println!("{}\n", "Black wins by checkmate!".bold())
                }
                game.display();
                break;
            },
            GameResult::Draw(typ) => {
                match typ {
                    DrawType::Repetition => println!("{}\n", "Game is a draw by repetition".bold()),
                    DrawType::Stalemate => println!("{}\n", "Game is a draw by stalemate".bold()),
                    DrawType::Material => println!("{}\n", "Game is a draw by insufficient material".bold()),
                    DrawType::FiftyMove => println!("{}\n", "Game is a draw by the fifty move rule".bold())
                }
                game.display();
                break;
            },
            GameResult::None => ()
        }
    }

    // Wait for enter to continue

    println!();
    println!("Press enter to continue:");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    util::clear_screen();
}