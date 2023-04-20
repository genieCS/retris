use cursive::{
    View, Vec2,
    Printer, 
    theme::{Color, ColorStyle},
};
use crate::block::Block;
use crate::lrd::{LRD, BOARD_WIDTH, BOARD_HEIGHT};

const BACKGROUND_FRONT: Color = Color::Rgb(0,0,0);
const BACKGROUND_BACK: Color = Color::Rgb(255,255,255);
pub struct CurrentBlock {
    block: Block,
    pos: Vec2,
}

impl CurrentBlock {
    fn can_move_lrd(&self, lrd: &LRD, board: &[[ColorStyle; BOARD_WIDTH]; BOARD_HEIGHT]) -> (bool, bool) {
        let delta = lrd.delta();
        let xy = self.pos;
        let mut stop = false;
        let background = ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK);
        for block in &self.block.shape.vectors() {
            let next_x = xy.x  as i32 + block.x as i32  + delta.0;
            let next_y =  xy.y as i32 + block.y as i32 + delta.1;
            if next_x < 0 || next_x >= BOARD_WIDTH as i32 || next_y < 0 || next_y >= BOARD_HEIGHT as i32 || board[next_y as usize][next_x as usize] != background
            {
                return (false, stop);
            }
            if next_y + 1 == BOARD_HEIGHT as i32 || next_y + 1 < BOARD_HEIGHT as i32 && board[next_y as usize + 1][next_x as usize] != background
            {
                stop = true;
            }
        }
        (true, stop)
    }

    fn move_lrd(&mut self, lrd: LRD) {
        let delta = lrd.delta();
        let x = self.pos.x as i32 + delta.0;
        let y = self.pos.y as i32 + delta.1;
        self.pos.x = x as usize;
        self.pos.y = y as usize;
    }
}

pub struct Container {
    current: CurrentBlock,
    board: [[ColorStyle; BOARD_WIDTH]; BOARD_HEIGHT],
    x_padding: usize,
    y_padding: usize,
}

impl Container {
    pub fn new() -> Self {
        Self {
            current: CurrentBlock {
                block: Block::new(),
                pos: Vec2::new(4, 0),
            },
            board: [[ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK); BOARD_WIDTH]; BOARD_HEIGHT],
            x_padding: 1,
            y_padding: 5,
        }
    }

    pub fn move_lrd(&mut self, lrd: LRD) -> bool {
        let (can_move, stop) = self.current.can_move_lrd(&lrd, &self.board);
        if !can_move {
            return true;
        }
        self.current.move_lrd(lrd);
        stop
    }

    pub fn drop_down(&mut self) {
        let mut stopped = false;
        while !stopped {
            stopped = self.move_lrd(LRD::Down)
        }
    }

    fn draw_background(&self, printer: &Printer) {
        for i in 0..BOARD_WIDTH {
            printer.print((2*i + 1 + self.x_padding, self.y_padding), "_");
        }
        for j in 0..BOARD_HEIGHT {
            for i in 0..BOARD_WIDTH {
                printer.print((2*i + self.x_padding, j + self.y_padding + 1), "|_");
            }
            printer.print((20+self.x_padding, j + self.y_padding + 1), "|");
        }
    }

    fn draw_current_block(&self, printer: &Printer) {
        let xy = self.current.pos;
        for block in &self.current.block.shape.vectors() {
            let x = xy.x as i32 + block.x as i32;
            let y = xy.y as i32 + block.y as i32;
                printer.with_color(self.current.block.color, |printer| {
                    printer.print((2*x as usize + self.x_padding + 1, y as usize + self.y_padding + 1), "_|");
                });
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Container {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
        self.draw_current_block(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(2*BOARD_WIDTH + 3, BOARD_HEIGHT + 10)
    }
}