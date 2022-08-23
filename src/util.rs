// Imports

use std::process;

// Clear screen with error handling

pub fn clear_screen() {
    if let Err(error) = clearscreen::clear() {
        eprintln!("Fatal error clearing terminal: {}", error);
        process::exit(1);
    }
}