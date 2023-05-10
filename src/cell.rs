#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub const fn new(x: usize, y: usize) -> Self {
        if x > 9 || y > 9 {
            panic!("usize out of range for cell");
        }
        Self { x, y }
    }
}
