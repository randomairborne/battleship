use crate::cell::Cell;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShipSet {
    carrier: Option<ShipState>,
    battleship: Option<ShipState>,
    destroyer: Option<ShipState>,
    submarine: Option<ShipState>,
    patrol: Option<ShipState>,
}

impl ShipSet {
    pub fn new() -> Self {
        Self::default()
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
    pub const fn lost(&self) -> bool {
        if let Some(ship) = self.carrier {
            if !ship.sunk {
                return false;
            }
        }
        if let Some(ship) = self.battleship {
            if !ship.sunk {
                return false;
            }
        }
        if let Some(ship) = self.destroyer {
            if !ship.sunk {
                return false;
            }
        }
        if let Some(ship) = self.submarine {
            if !ship.sunk {
                return false;
            }
        }
        if let Some(ship) = self.patrol {
            if !ship.sunk {
                return false;
            }
        }
        true
    }
    pub fn cell_contains_ship(&self, cell: &Cell) -> bool {
        self.occupied_cells().contains(cell)
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
    pub const fn rot(&self) -> ShipRotation {
        self.rot
    }
    pub const fn pos(&self) -> Cell {
        self.pos
    }
    pub const fn sunk(&self) -> bool {
        self.sunk
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
            let mut cell = Cell::new(cell_x, cell_y);
            cell.ship = Some(self.kind);
            occupies.push(cell);
        }
        occupies
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShipRotation {
    Up,
    Down,
    Left,
    Right,
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
