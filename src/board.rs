use std::fmt::Write;

use crate::{cell::Cell, Error};

#[derive(Debug, Clone)]
pub struct Board {
    locals: RawBoard,
    ships: ShipSet,
}

impl Board {
    /// If this function errors, then the ship state was invalid
    pub fn new(
        carrier: ShipState,
        battleship: ShipState,
        destroyer: ShipState,
        submarine: ShipState,
        patrol: ShipState,
    ) -> Result<Self, Error> {
        let me = Self {
            locals: [[Shot::Empty; 10]; 10],
            ships: ShipSet {
                carrier,
                battleship,
                destroyer,
                submarine,
                patrol,
            },
        };
        if !me.ships.is_valid() {
            return Err(Error::InvalidShipState);
        }
        Ok(me)
    }
    pub fn shot(&mut self, cell: &Cell) -> Shot {
        let outcome = if self.contains_ship(cell) {
            Shot::Hit
        } else {
            Shot::Miss
        };
        self.update_cell(cell, outcome);
        outcome
    }
    fn contains_ship(&self, cell: &Cell) -> bool {
        self.ships.occupied_cells().contains(cell)
    }
    fn update_cell(&mut self, cell: &Cell, value: Shot) {
        let shot = self.shot_mut(cell);
        *shot = value;
    }
    fn shot_mut(&mut self, cell: &Cell) -> &mut Shot {
        &mut self.locals[cell.x][cell.y]
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   1  2  3  4  5  6  7  8  9 10")?;
        let mut letter = 'A' as u32;
        for row in self.locals {
            let row_letter = char::from_u32(letter).unwrap_or('X');
            write!(f, "{row_letter} ")?;
            letter += 1;
            for item in row {
                let out = match item {
                    Shot::Hit => " \u{001b}[31m\u{25cf}\u{001b}[0m ",
                    Shot::Miss => " \u{001b}[0m\u{25ce}\u{001b}[0m ",
                    Shot::Empty => "\u{001b}[34m\u{2591}\u{2591}\u{2591}\u{001b}[0m",
                };
                f.write_str(out)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type RawBoard = [[Shot; 10]; 10];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Shot {
    Hit,
    Miss,
    Empty,
}

impl Default for Shot {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ShipSet {
    carrier: ShipState,
    battleship: ShipState,
    destroyer: ShipState,
    submarine: ShipState,
    patrol: ShipState,
}

impl ShipSet {
    pub fn occupied_cells(&self) -> Vec<Cell> {
        // 17 is the max number of cells we could take up
        // this hyper-optimizes allocations
        // because *that's* the expensive operation here
        let mut out: Vec<Cell> = Vec::with_capacity(17);
        crate::add_cells_for_ship!(out, self.carrier, 5);
        crate::add_cells_for_ship!(out, self.battleship, 4);
        crate::add_cells_for_ship!(out, self.destroyer, 3);
        crate::add_cells_for_ship!(out, self.submarine, 3);
        crate::add_cells_for_ship!(out, self.patrol, 2);
        out
    }
    pub fn is_valid(&self) -> bool {
        let cells = self.occupied_cells();
        let mut uniq = std::collections::HashSet::new();
        cells.into_iter().all(move |x| uniq.insert(x))
    }
}
#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! add_cells_for_ship {
        ($cells:expr, $ship:expr, $length:literal) => {
            for i in 0..$length {
                let (cell_x, cell_y) = match $ship.rot {
                    ShipRotation::Up => ($ship.x, $ship.y - i),
                    ShipRotation::Down => ($ship.x, $ship.y + i),
                    ShipRotation::Left => ($ship.x - i, $ship.y),
                    ShipRotation::Right => ($ship.x + i, $ship.y),
                };
                $cells.push(Cell::new(cell_x, cell_y));
            }
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ShipState {
    rot: ShipRotation,
    x: usize,
    y: usize,
}

impl ShipState {
    pub fn new(x: usize, y: usize, rot: ShipRotation) -> Result<Self, Error> {
        if x > 9 || y > 9 {
            return Err(Error::InvalidShipState);
        };
        Ok(Self { rot, x, y })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShipRotation {
    Up,
    Down,
    Left,
    Right,
}
