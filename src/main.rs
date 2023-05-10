use std::io::Write;

use board::Board;
use cell::Cell;

use crate::board::{ShipRotation, ShipState};
use crossterm::{cursor::MoveTo, execute, queue, style::PrintStyledContent, terminal::Clear};

mod board;
mod cell;
mod error;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode on terminal");
    ctrlc::set_handler(move || {
        crossterm::terminal::disable_raw_mode()
            .expect("Failed to disable raw mode, terminal may be corrupted!");
        println!("Thanks for playing!");
        std::process::exit(0);
    })
    .expect("Failed to set ctrl_c handler, terminal may be corrupted!");
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut cursor_location = (0usize, 0usize);
    loop {
        queue!(stdout, Clear(crossterm::terminal::ClearType::All))?;
        
        for x in 1..11 {
            for y in 1..11 {
                queue!(stdout, MoveTo(0, 0))?;
                queue!(stdout, PrintStyledContent())
            }
        }
        stdout.flush()?;
    }
}
