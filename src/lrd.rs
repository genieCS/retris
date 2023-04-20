pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub enum LRD {
    Left,
    Right,
    Down,
}

impl LRD {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            LRD::Left => (-1, 0),
            LRD::Right => (1, 0),
            LRD::Down => (0, 1),
        }
     }
}