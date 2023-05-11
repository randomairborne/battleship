use std::ops::{AddAssign, SubAssign};

use crate::ship::ShipType;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    x: usize,
    y: usize,
    pub ship: Option<ShipType>,
}

impl Cell {
    pub const fn new(mut x: usize, mut y: usize) -> Self {
        if x >= 10 {
            x -= 10;
        }
        if y >= 10 {
            y -= 10;
        }
        Self { x, y, ship: None }
    }
    pub const fn x(&self) -> usize {
        self.x
    }
    pub const fn y(&self) -> usize {
        self.y
    }
}

impl AddAssign<(usize, usize)> for Cell {
    fn add_assign(&mut self, rhs: (usize, usize)) {
        // this is actually an implementation of modulo 10. weird, right?
        self.x += rhs.0;
        self.y += rhs.1;
        if self.x >= 10 {
            self.x -= 10;
        }
        if self.y >= 10 {
            self.y -= 10;
        }
    }
}

impl SubAssign<(usize, usize)> for Cell {
    fn sub_assign(&mut self, rhs: (usize, usize)) {
        if rhs.0 > self.x {
            self.x += 10;
        }
        if rhs.1 > self.y {
            self.y += 10;
        }
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}
