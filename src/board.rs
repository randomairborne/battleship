use crate::{
    cell::Cell,
    ship::{ShipSet, ShipType},
};

#[derive(Debug, Clone)]
pub struct Board<'a> {
    locals: RawBoard,
    pub ships: ShipSet<'a>,
}

impl<'a> Board<'a> {
    /// If this function errors, then the ship state was invalid
    pub fn new(ships: ShipSet<'a>) -> Self {
        Self {
            locals: [[Shot::Empty; 10]; 10],
            ships,
        }
    }
    pub fn fire(&mut self, cell: &Cell) -> Option<Shot> {
        if *self.shot_mut(cell) != Shot::Empty {
            return None;
        }

        let outcome = if let Some(shipref) = self.ships.ref_for(*cell) {
            Shot::Hit(shipref.kind())
        } else {
            Shot::Miss
        };
        self.update_cell(cell, outcome);
        Some(outcome)
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

pub type RawBoard = [[Shot; 10]; 10];

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
