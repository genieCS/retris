pub enum LR {
    Left,
    Right,
}

impl LR {
    pub fn to_lrd(self) -> LRD {
        LRD::LR(self)
    }
}

pub enum LRD {
    LR(LR),
    Down,
}

impl LRD {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            LRD::LR(LR::Left)=>  (-1, 0),
            LRD::LR(LR::Right) => (1, 0),
            LRD::Down => (0, 1),
        }
    }
}
