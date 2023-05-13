mod set;
mod state;

pub use set::{ShipSet, ShipSetBuilder};
pub use state::ShipState;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShipRotation {
    Up,
    Down,
    Left,
    Right,
}

impl ShipRotation {
    pub fn next(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };
    }
    pub fn prev(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShipType {
    AircraftCarrier,
    Battleship,
    Destroyer,
    Submarine,
    PatrolBoat,
}

impl ShipType {
    /// Returns `true` if complete
    pub fn next(&mut self) -> bool {
        *self = match self {
            Self::AircraftCarrier => Self::Battleship,
            Self::Battleship => Self::Destroyer,
            Self::Destroyer => Self::Submarine,
            Self::Submarine => Self::PatrolBoat,
            Self::PatrolBoat => return true,
        };
        false
    }
}

impl std::fmt::Display for ShipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::AircraftCarrier => "Aircraft Carrier",
            Self::Battleship => "Battleship",
            Self::Destroyer => "Destroyer",
            Self::Submarine => "Submarine",
            Self::PatrolBoat => "Patrol Boat",
        };
        f.write_str(name)
    }
}
