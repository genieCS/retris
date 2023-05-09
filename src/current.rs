use crate::block::Block;
use crate::color_grid::ColorGrid;
use crate::lrd::LRD;
use cursive::{
    Vec2,
    theme::ColorStyle,
};


#[derive(Clone, Debug)]
pub struct Current {
    pub block: Block,
    pub pos: Vec2,
}

impl Current {
    pub fn new(block: Block, _pos: Vec2, board_width: usize, board_height: usize) -> Self {
        let mut pos = _pos;
        let mut chance = 4;
        while chance > 0 {
            let mut possible = true;
            for cell in block.cells() {
                let x = pos.x as i32 + cell.0;
                let y = pos.y as i32 + cell.1;
                if x < 0 {
                    pos.x += 1;
                    possible = false;
                    break;
                } else if x >= board_width as i32 {
                    pos.x -= 1;
                    possible = false;
                    break;
                } else if y < 0 {
                    pos.y += 1;
                    possible = false;
                    break;
                } else if y >= board_height  as i32{
                    pos.y -= 1;
                    possible = false;
                    break;
                }
            }
            if possible {
                break;
            }
            chance -= 1;
        }

        Self {
            block,
            pos,
        }
    }

    pub fn move_lrd(&mut self, lrd: LRD, colors: &ColorGrid) -> (bool, bool) {
        let (can_move, stop) = self.can_move_lrd(&lrd, colors);
        if !can_move {
            return (false, stop)
        }
        let delta = lrd.delta();
        let x = self.pos.x as i32 + delta.0;
        let y = self.pos.y as i32 + delta.1;
        self.pos.x = x as usize;
        self.pos.y = y as usize;
        (true, stop)
    }

    fn can_move_lrd(&self, lrd: &LRD, colors: &ColorGrid) -> (bool, bool) {
        let delta = lrd.delta();
        let mut stop = false;
        let board_width = colors.width() as i32;
        let board_height = colors.height() as i32;
        for cell in self {
            let next_x = cell.x as i32 + delta.0;
            let next_y =  cell.y as i32 + delta.1;
            if next_x < 0 || next_x >= board_width || next_y < 0 || next_y >= board_height || colors.is_occupied(next_x as usize, next_y as usize)
            {
                return (false, false);
            }
            if next_y + 1 == board_height || colors.is_occupied(next_x as usize, next_y as usize + 1)
            {
                stop = true;
            }
        }
        (true, stop)
    }

    pub fn rotate(&mut self, colors: &ColorGrid) {
        let next = self.block.rotate();
        let mut chance = 4;
        let board_width = colors.width() as i32;
        let board_height = colors.height() as i32;
        while chance > 0 {
            let mut possible = true;
            for cell in next.cells() {
                let next_x = self.pos.x  as i32 + cell.0 as i32;
                let next_y = self.pos.y as i32 + cell.1 as i32;
                if next_x < 0 {
                    possible = false;
                    self.pos.x += 1;
                    break;
                } else if next_x >= board_width {
                    possible = false;
                    self.pos.x -= 1;
                    break;
                } else if next_y < 0 {
                    possible = false;
                    self.pos.y += 1;
                    break;
                } else if next_y >= board_height {
                    possible = false;
                    self.pos.y -= 1;
                    break;
                } else if colors.is_occupied(next_x as usize, next_y as usize) {
                    possible = false;
                    break;
                }
            }
            if possible {
                break;
            }
            chance -= 1;
        }
        if chance > 0 {
            self.block = next;
        }
    }

    pub fn color(&self) -> ColorStyle {
        self.block.color()
    }

    pub fn hint(&self, colors: &ColorGrid) -> Current {
        let mut hint = self.clone();
        let mut stopped = false;
        while !stopped {
            let (moved, hit_bottom) = hint.move_lrd(LRD::Down, colors);
            stopped = hit_bottom || !moved;
        }
        hint
    }
}

impl IntoIterator for &Current {
    type Item = Vec2;
    type IntoIter = std::vec::IntoIter<Vec2>;


    fn into_iter(self) -> Self::IntoIter {
        let pos = self.pos;
        self.block.cells().into_iter().map(|cell| {
            let x = pos.x as i32 + cell.0;
            let y = pos.y as i32 + cell.1;
            Vec2::new(x as usize, y as usize)
        }).collect::<Vec<Vec2>>().into_iter()
    }
}