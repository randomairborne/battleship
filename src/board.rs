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
    pub fn locals(&self) -> RawBoard {
        self.locals
    }
    pub fn ships(&self) -> ShipSet {
        self.ships
    }
    pub fn shot(&mut self, cell: &Cell) -> Option<Shot> {
        if *self.shot_mut(cell) != Shot::Empty {
            return None;
        }
        let outcome = if self.contains_ship(cell) {
            Shot::Hit
        } else {
            Shot::Miss
        };
        self.update_cell(cell, outcome);
        Some(outcome)
    }
    pub fn contains_ship(&self, cell: &Cell) -> bool {
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
        let letter = 'A' as u32;
        for idx in 0..10 {
            let row_letter = char::from_u32(letter + idx).unwrap_or('X');
            write!(f, "{row_letter} ")?;
            for (rowi, row) in self.locals.iter().enumerate() {
                let bg_color = if self.contains_ship(&Cell::new(rowi, idx as usize)) {
                    "100"
                } else {
                    "104"
                };
                match row[idx as usize] {
                    Shot::Hit => write!(f, "\u{001b}[31;{bg_color}m \u{25cf} \u{001b}[0m"),
                    Shot::Miss => write!(f, "\u{001b}[0;{bg_color}m \u{25cf} \u{001b}[0m"),
                    Shot::Empty => write!(f, "\u{001b}[0;{bg_color}m   \u{001b}[0m"),
                }?;
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
            let pos = $ship.pos;
            for i in 0..$length {
                let (cell_x, cell_y) = match $ship.rot {
                    ShipRotation::Up => ($ship.pos.x, $ship.pos.y - i),
                    ShipRotation::Down => ($ship.pos.x, $ship.pos.y + i),
                    ShipRotation::Left => ($ship.pos.x - i, $ship.pos.y),
                    ShipRotation::Right => ($ship.pos.x + i, $ship.pos.y),
                };
                $cells.push(Cell::new(cell_x, cell_y));
            }
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShipSetBuilder {
    pub carrier: Option<ShipState>,
    pub battleship: Option<ShipState>,
    pub destroyer: Option<ShipState>,
    pub submarine: Option<ShipState>,
    pub patrol: Option<ShipState>,
}

impl ShipSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(self) -> Option<ShipSet> {
        let set = ShipSet {
            carrier: self.carrier?,
            battleship: self.battleship?,
            destroyer: self.destroyer?,
            submarine: self.submarine?,
            patrol: self.patrol?,
        };
        Some(set)
    }
    pub fn carrier(mut self, ship: ShipState) -> Self {
        self.carrier = Some(ship);
        self
    }
    pub fn battleship(mut self, ship: ShipState) -> Self {
        self.battleship = Some(ship);
        self
    }
    pub fn destroyer(mut self, ship: ShipState) -> Self {
        self.destroyer = Some(ship);
        self
    }
    pub fn submarine(mut self, ship: ShipState) -> Self {
        self.submarine = Some(ship);
        self
    }
    pub fn patrol(mut self, ship: ShipState) -> Self {
        self.patrol = Some(ship);
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ShipState {
    rot: ShipRotation,
    pos: Cell,
    sunk: bool,
}

impl ShipState {
    pub fn new(pos: Cell, rot: ShipRotation, sunk: bool) -> Self {
        Self { rot, pos, sunk }
    }
    pub fn rot(&self) -> ShipRotation {
        self.rot
    }
    pub fn pos(&self) -> Cell {
        self.pos
    }
    pub fn sunk(&self) -> bool {
        self.sunk
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShipRotation {
    Up,
    Down,
    Left,
    Right,
}
