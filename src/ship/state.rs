use crate::cell::Cell;

use super::{ShipRotation, ShipType};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ShipState {
    rot: ShipRotation,
    pos: Cell,
    sunk: bool,
    kind: ShipType,
}

impl ShipState {
    pub const fn new(pos: Cell, rot: ShipRotation, sunk: bool, kind: ShipType) -> Self {
        Self {
            rot,
            pos,
            sunk,
            kind,
        }
    }
    pub const fn sunk(&self) -> bool {
        self.sunk
    }
    pub const fn kind(&self) -> ShipType {
        self.kind
    }
    pub const fn length(&self) -> usize {
        match self.kind {
            ShipType::AircraftCarrier => 5,
            ShipType::Battleship => 4,
            ShipType::Destroyer | ShipType::Submarine => 3,
            ShipType::PatrolBoat => 2,
        }
    }
    pub const fn overflows(&self) -> bool {
        match self.rot {
            ShipRotation::Up => self.pos.y() < self.length() - 1,
            ShipRotation::Down => self.pos.y() + self.length() > 10,
            ShipRotation::Left => self.pos.x() < self.length() - 1,
            ShipRotation::Right => self.pos.x() + self.length() > 10,
        }
    }
    pub fn occupies(&self) -> Vec<Cell> {
        let mut occupies: Vec<Cell> = Vec::with_capacity(self.length());
        for i in 0..self.length() {
            let mut pos_x = self.pos.x();
            let mut pos_y = self.pos.y();
            match self.rot {
                ShipRotation::Up => {
                    if pos_y < i {
                        pos_y += 10;
                    }
                }
                ShipRotation::Left => {
                    if pos_x < i {
                        pos_x += 10;
                    }
                }
                _ => {}
            };
            let (cell_x, cell_y) = match self.rot {
                ShipRotation::Up => (pos_x, pos_y - i),
                ShipRotation::Down => (pos_x, pos_y + i),
                ShipRotation::Left => (pos_x - i, pos_y),
                ShipRotation::Right => (pos_x + i, pos_y),
            };
            occupies.push(Cell::new(cell_x, cell_y));
        }
        occupies
    }
}
