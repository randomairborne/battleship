use board::Board;
use cell::Cell;

use crate::board::{ShipRotation, ShipState};

mod board;
mod cell;
mod error;

pub use error::Error;

fn main() {
    let mut board = Board::new(
        ShipState::new(0, 0, ShipRotation::Down).expect("invalid ship state"),
        ShipState::new(1, 0, ShipRotation::Down).expect("invalid ship state"),
        ShipState::new(2, 0, ShipRotation::Down).expect("invalid ship state"),
        ShipState::new(3, 0, ShipRotation::Down).expect("invalid ship state"),
        ShipState::new(4, 0, ShipRotation::Down).expect("invalid ship state"),
    )
    .expect("Invalid board layout");
    println!("{board}");
    let mut shot = String::with_capacity(3);
    loop {
        shot.clear();
        std::io::stdin().read_line(&mut shot).expect("Failed to read line");
        let chars: Vec<char> = shot.trim_end().chars().collect();
        if chars.len() != 2 {
            eprintln!("Wrong length for shot: {}", chars.len());
            continue;
        }
        let l_idx = chars[0].to_ascii_uppercase() as usize - 'A' as usize;
        let n_idx = chars[1].to_digit(10).expect("invalid digit") - 1;
        let n_idx = n_idx as usize;
        if board.shot(&Cell::new(n_idx, l_idx)).is_none() {
            println!("You already took this shot!");
            continue;
        }
        println!("{board}");
    }
}
