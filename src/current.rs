use crate::block::Block;
use crate::lrd::LRD;
use cursive::{
    Vec2,
    theme::{Color, ColorStyle},
};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const BACKGROUND_FRONT: Color = Color::Rgb(0,0,0);
pub const BACKGROUND_BACK: Color = Color::Rgb(255,255,255);

#[derive(Clone, Debug)]
pub struct Current {
    pub block: Block,
    pub pos: Vec2,
}

impl Current {
    pub fn can_move_lrd(&self, lrd: &LRD, colors: &[[ColorStyle; BOARD_WIDTH]; BOARD_HEIGHT]) -> (bool, bool) {
        let delta = lrd.delta();
        let xy = self.pos;
        let mut stop = false;
        let background = ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK);
        for block in &self.block.vectors() {
            let next_x = xy.x  as i32 + block.x as i32  + delta.0;
            let next_y =  xy.y as i32 + block.y as i32 + delta.1;
            if next_x < 0 || next_x >= BOARD_WIDTH as i32 || next_y < 0 || next_y >= BOARD_HEIGHT as i32 || colors[next_y as usize][next_x as usize] != background
            {
                return (false, false);
            }
            if next_y + 1 == BOARD_HEIGHT as i32 || next_y + 1 < BOARD_HEIGHT as i32 && colors[next_y as usize + 1][next_x as usize] != background
            {
                stop = true;
            }
        }
        (true, stop)
    }

    pub fn move_lrd(&mut self, lrd: LRD, colors: &[[ColorStyle; BOARD_WIDTH]; BOARD_HEIGHT]) -> (bool, bool) {
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

    pub fn can_rotate(&mut self, colors: &[[ColorStyle; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
        let next_block = self.block.rotate();
        let background = ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK);
        let warning_color = ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(200,200,0));
        let mut possible = false;
        let mut chance = 4;
        while !possible && chance > 0 {
            possible = true;
            for block in &next_block.vectors() {
                let next_x = self.pos.x  as i32 + block.x as i32;
                let next_y = self.pos.y as i32 + block.y as i32;
                if next_x < 0 {
                    possible = false;
                    self.pos.x += 1;
                    break;
                } else if next_x >= BOARD_WIDTH as i32 {
                    possible = false;
                    self.pos.x -= 1;
                    break;
                } else if next_y < 0 {
                    possible = false;
                    self.pos.y += 1;
                    break;
                } else if next_y >= BOARD_HEIGHT as i32 {
                    possible = false;
                    self.pos.y -= 1;
                    break;
                } else if colors[next_y as usize][next_x as usize] != background && colors[next_y as usize][next_x as usize] != warning_color {
                    possible = false;
                    break;
                }
            }
            chance -= 1;
        }
        true
    }

    pub fn rotate(&mut self, colors: &[[ColorStyle; BOARD_WIDTH]; BOARD_HEIGHT]) {
        let can_rotate = self.can_rotate(colors);
        if can_rotate {
            self.block = self.block.rotate();
        }
    }
}