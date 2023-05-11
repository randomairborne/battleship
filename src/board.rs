use crate::{
    cell::Cell,
    ship::{ShipSet, ShipType},
};

#[derive(Debug, Clone)]
pub struct Board {
    locals: RawBoard,
    pub ships: ShipSet,
}

impl Board {
    /// If this function errors, then the ship state was invalid
    pub fn new() -> Self {
        Self {
            locals: [[Shot::Empty; 10]; 10],
            ships: ShipSet::default(),
        }
    }
    pub fn fire(&mut self, cell: &Cell) -> Option<Shot> {
        if *self.shot_mut(cell) != Shot::Empty {
            return None;
        }
        let outcome = if self.contains_ship(cell) {
            Shot::Hit(cell.ship.unwrap())
        } else {
            Shot::Miss
        };
        self.update_cell(cell, outcome);
        Some(outcome)
    }
    pub fn contains_ship(&self, cell: &Cell) -> bool {
        self.ships.cell_contains_ship(cell)
    }
    fn update_cell(&mut self, cell: &Cell, value: Shot) {
        let shot = self.shot_mut(cell);
        *shot = value;
    }
    pub const fn shot(&self, cell: &Cell) -> Shot {
        self.locals[cell.x()][cell.y()]
    }
    fn shot_mut(&mut self, cell: &Cell) -> &mut Shot {
        &mut self.locals[cell.x()][cell.y()]
    }
}

type RawBoard = [[Shot; 10]; 10];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Shot {
    Hit(ShipType),
    Miss,
    Empty,
}

impl Default for Shot {
    fn default() -> Self {
        Self::Empty
    }
}
