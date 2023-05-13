#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
mod board;
mod cell;
mod error;
mod ship;
mod ui;
mod util;

use cell::Cell;

#[macro_use]
extern crate crossterm;

use crossterm::terminal::{Clear, SetTitle};

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode on terminal");
    std::panic::set_hook(Box::new(|info| {
        crossterm::terminal::disable_raw_mode().ok();
        eprintln!("{info}");
    }));
    let mut stdout = std::io::stdout();
    execute!(stdout, SetTitle("Battleship! by valkyrie_pilot"))?;
    let play_mode = ui::menu::select_play_mode(&mut stdout)?;
    let mut cursor = Cell::new(0, 0);
    let mut p1 = ui::setup::do_place(&mut stdout, &mut cursor, 1, "Player 1: Place your ships")?;
    ui::show_pass(&mut stdout, 2)?;
    let mut p2 = ui::setup::do_place(&mut stdout, &mut cursor, 2, "Player 2: Place your ships")?;
    let winner;
    loop {
        ui::play::turn(&mut stdout, &mut p1, &mut p2, &mut cursor, 1)?;
        if p2.lost() {
            winner = "Player 1 wins!".to_string();
            break;
        }
        ui::play::turn(&mut stdout, &mut p2, &mut p1, &mut cursor, 2)?;
        if p1.lost() {
            winner = "Player 2 wins!".to_string();
            break;
        }
    }
    execute!(stdout, Clear(crossterm::terminal::ClearType::All))?;
    crossterm::terminal::disable_raw_mode().ok();
    // todo: fancy end screen
    println!("{winner}");
    Ok(())
}
