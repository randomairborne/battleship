use std::sync::Arc;

use crate::cell::Cell;

use super::ShipState;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShipSet {
    carrier: ShipState,
    battleship: ShipState,
    destroyer: ShipState,
    submarine: ShipState,
    patrol: ShipState,
    refs: RawShipBoard,
}

impl ShipSet {
    pub fn ship_in(&self, cell: Cell) -> Option<ShipState> {
        self.refs[cell.x()][cell.y()]
    }
    pub fn contains_ship(&self, cell: Cell) -> bool {
        self.ship_in(cell).is_some()
    }
    pub fn occupied_cells(&self) -> Vec<Cell> {
        // 17 is the max number of cells we could take up
        // this hyper-optimizes allocations
        // because *that's* the expensive operation here
        let mut out: Vec<Cell> = Vec::with_capacity(17);
        out.append(&mut self.carrier.occupies());
        out.append(&mut self.battleship.occupies());
        out.append(&mut self.destroyer.occupies());
        out.append(&mut self.submarine.occupies());
        out.append(&mut self.patrol.occupies());

        out
    }
}

type RawShipBoard = [[Option<ShipState>; 10]; 10];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShipSetBuilder {
    carrier: Option<ShipState>,
    battleship: Option<ShipState>,
    destroyer: Option<ShipState>,
    submarine: Option<ShipState>,
    patrol: Option<ShipState>,
}

impl ShipSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(&self) -> Option<ShipSet> {
        if !self.is_valid() {
            return None;
        }
        let carrier = self.carrier?;
        let battleship = self.battleship?;
        let destroyer = self.destroyer?;
        let submarine = self.submarine?;
        let patrol = self.patrol?;
        // thank you, copy requirement for [None; 10]
        let mut refs: RawShipBoard = [[None; 10]; 10];
        for cell in carrier.occupies() {
            refs[cell.x()][cell.y()] = Some(carrier);
        }
        for cell in battleship.occupies() {
            refs[cell.x()][cell.y()] = Some(battleship);
        }
        for cell in destroyer.occupies() {
            refs[cell.x()][cell.y()] = Some(destroyer);
        }
        for cell in submarine.occupies() {
            refs[cell.x()][cell.y()] = Some(submarine);
        }
        for cell in patrol.occupies() {
            refs[cell.x()][cell.y()] = Some(patrol);
        }
        Some(ShipSet {
            carrier,
            battleship,
            destroyer,
            submarine,
            patrol,
            refs,
        })
    }
    pub fn occupied_cells(&self) -> Vec<Cell> {
        // 17 is the max number of cells we could take up
        // this hyper-optimizes allocations
        // because *that's* the expensive operation here
        let mut out: Vec<Cell> = Vec::with_capacity(17);
        if let Some(carrier) = self.carrier {
            out.append(&mut carrier.occupies());
        }
        if let Some(battleship) = self.battleship {
            out.append(&mut battleship.occupies());
        }
        if let Some(destroyer) = self.destroyer {
            out.append(&mut destroyer.occupies());
        }
        if let Some(submarine) = self.submarine {
            out.append(&mut submarine.occupies());
        }
        if let Some(patrol) = self.patrol {
            out.append(&mut patrol.occupies());
        }
        out
    }
    pub fn is_valid(&self) -> bool {
        if let Some(carrier) = self.carrier {
            if carrier.overflows() {
                return false;
            }
        }
        if let Some(battleship) = self.battleship {
            if battleship.overflows() {
                return false;
            }
        }
        if let Some(destroyer) = self.destroyer {
            if destroyer.overflows() {
                return false;
            }
        }
        if let Some(submarine) = self.submarine {
            if submarine.overflows() {
                return false;
            }
        }
        if let Some(patrol) = self.patrol {
            if patrol.overflows() {
                return false;
            }
        }
        let cells = self.occupied_cells();
        let mut uniq = std::collections::HashSet::new();
        cells.into_iter().all(move |x| uniq.insert(x))
    }
    pub fn contains_ship(&self, cell: Cell) -> bool {
        self.occupied_cells().contains(&cell)
    }
    pub fn carrier(&mut self, ship: ShipState) {
        self.carrier = Some(ship);
    }
    pub fn battleship(&mut self, ship: ShipState) {
        self.battleship = Some(ship);
    }
    pub fn destroyer(&mut self, ship: ShipState) {
        self.destroyer = Some(ship);
    }
    pub fn submarine(&mut self, ship: ShipState) {
        self.submarine = Some(ship);
    }
    pub fn patrol(&mut self, ship: ShipState) {
        self.patrol = Some(ship);
    }
}
