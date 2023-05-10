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
    board.shot(&Cell::new(0, 3));
    board.shot(&Cell::new(9, 9));
    println!("{board}");
}
