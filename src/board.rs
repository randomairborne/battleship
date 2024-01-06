use std::sync::Arc;

use crate::{
    cell::Cell,
    ship::{ShipSet, ShipState},
};

#[derive(Debug, Clone)]
pub struct Board {
    locals: RawBoard,
    pub ships: ShipSet,
}

impl Board {
    /// If this function errors, then the ship state was invalid
    pub const fn new(ships: ShipSet) -> Self {
        Self {
            locals: [[Shot::Empty; 10]; 10],
            ships,
        }
    }
    pub fn fire(&mut self, cell: &Cell) -> Option<Shot> {
        if *self.shot_mut(cell) != Shot::Empty {
            return None;
        }
        let outcome = self.ships.ref_for(*cell).map_or(Shot::Miss, Shot::Hit);
        self.update_cell(cell, outcome.clone());
        Some(outcome)
    }
    pub fn lost(&self) -> bool {
        self.ships
            .occupied_cells()
            .iter()
            .all(|v| matches!(self.shot(v), Shot::Hit(_ship)))
    }
    fn update_cell(&mut self, cell: &Cell, value: Shot) {
        let shot = self.shot_mut(cell);
        *shot = value;
    }
    pub fn shot(&self, cell: &Cell) -> Shot {
        self.locals[cell.x()][cell.y()].clone()
    }
    fn shot_mut(&mut self, cell: &Cell) -> &mut Shot {
        &mut self.locals[cell.x()][cell.y()]
    }
}

pub type RawBoard = [[Shot; 10]; 10];

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Shot {
    Hit(Arc<ShipState>),
    Miss,
    Empty,
}

impl Default for Shot {
    fn default() -> Self {
        Self::Empty
    }
}
