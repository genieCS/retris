use cursive::{
    event::{Event, EventResult, Key},
    View, Vec2,
    Printer, 
    theme::{Color, ColorStyle},
};
use crate::block::{ Block, BlockWithPos };
use crate::color_grid::ColorGrid;
use crate::lrd::{ LR, LRD };

pub struct Board {
    block: BlockWithPos,
    colors: ColorGrid,
}

impl Board {
    pub fn new(background_color: (ColorStyle, ColorStyle), warning_color: ColorStyle, width: usize, height: usize) -> Self {
        Self {
            block: Self::insert_new_block(Block::default(), width, height),
            colors: ColorGrid::new(
                background_color,
                warning_color,
                width,
                height,
            ),
        }
    }

    pub fn renew(&mut self) {
        self.colors.clear();
        self.insert(Block::default());
    }

    pub fn insert(&mut self, block: Block) {
        self.block = Board::insert_new_block(block, self.colors.width(), self.colors.height());
    }

    fn insert_new_block(block: Block, width: usize, height: usize) -> BlockWithPos {
        Board::fit(block, Vec2::new(width / 2, 0), width, height).unwrap()
    }

    fn fit(block: Block, _pos: Vec2, width: usize, height: usize) -> Option<BlockWithPos> {
        let mut pos = _pos;
        for _ in 0..6 {
            let mut possible = true;
            for cell in block.cells() {
                let x = pos.x as i32 + cell.0;
                let y = pos.y as i32 + cell.1;
                if x < 0 {
                    pos.x += 1;
                    possible = false;
                    break;
                } else if x >= width as i32 {
                    pos.x -= 1;
                    possible = false;
                    break;
                } else if y < 0 {
                    pos.y += 1;
                    possible = false;
                    break;
                } else if y >= height as i32{
                    pos.y -= 1;
                    possible = false;
                    break;
                }
            }
            if possible {
                return Some(BlockWithPos::new(block,pos));
            }
        }
        None
    }

    pub fn move_block_lrd(&self, block: &BlockWithPos, lrd: LRD) -> (Option<BlockWithPos>, bool) {
        let (can_move, stop) = self.can_move(block, &lrd);
        if !can_move {
            return (None, stop)
        }
        let delta = lrd.delta();
        let x = block.pos.x as i32 + delta.0;
        let y = block.pos.y as i32 + delta.1;
        let bwp = BlockWithPos::new(block.block.clone(), Vec2::new(x as usize, y as usize));
        (Some(bwp), stop)
    }

    fn can_move(&self, block: &BlockWithPos, lrd: &LRD) -> (bool, bool) {
        let delta = lrd.delta();
        let mut stop = false;
        let board_width = self.colors.width() as i32;
        let board_height = self.colors.height() as i32;
        for cell in block.cells() {
            let next_x = cell.x as i32 + delta.0;
            let next_y =  cell.y as i32 + delta.1;
            if next_x < 0 || next_x >= board_width || next_y < 0 || next_y >= board_height || self.colors.is_occupied(next_x as usize, next_y as usize)
            {
                return (false, false);
            }
            if next_y + 1 == board_height || self.colors.is_occupied(next_x as usize, next_y as usize + 1)
            {
                stop = true;
            }
        }
        (true, stop)
    }

    pub fn hint(&self) -> BlockWithPos {
        let mut hint = self.block.clone();
        let mut stopped = false;
        while !stopped {
            let (block, hit_bottom) = self.move_block_lrd(&hint, LRD::Down);
            stopped = hit_bottom || block.is_none();
            hint = block.unwrap_or(hint);
        }
        hint
    }

    pub fn on_down(&mut self, is_drop: bool) -> (bool, bool) {
        let mut stopped = false;
        let mut hit_bottom = is_drop;
        let mut current: Option<BlockWithPos>;
        while !stopped {
            (current, hit_bottom)= self.move_block_lrd(&self.block, LRD::Down);
            match current {
                Some(b) => self.block = b,
                None => return (true, true),
            }

            stopped = hit_bottom || !is_drop;
        }
        (false, hit_bottom && self.colors.merge_block(&self.block))
    }

    fn handle_lr(&mut self, lr: LR) -> EventResult {
        let (block, _) = self.move_block_lrd(&self.block, lr.to_lrd());
        if block.is_some() {
            self.block = block.unwrap();
        }
        EventResult::Consumed(None)
    }

    pub fn rotate(&mut self) -> EventResult {
        let next_block = self.block.block.rotate();
        Board::fit(next_block, self.block.pos, self.colors.width(), self.colors.height())
            .map(|b| self.block = b);
        EventResult::Consumed(None)
    }
}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
        self.draw_dangerous(printer);
        self.draw_hint(&self.hint(), printer);
        self.draw_block(&self.block, printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let width = self.colors.width();
        let height = self.colors.height();
        cursive::Vec2::new(2*width + 3, height + 10)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Left)  => self.handle_lr(LR::Left),
            Event::Key(Key::Right) => self.handle_lr(LR::Right),
            Event::Key(Key::Up) => self.rotate(),
            _ => EventResult::Ignored,
        }
    }
}

impl Board {
    fn draw_background(&self, printer: &Printer) {
        let width = self.colors.width();
        let height = self.colors.height();
        for j in 0..height {
            for i in 0..width {
                printer.with_color(self.colors[j][i], |printer| {
                    printer.print((2*i, j), "  ");
                });
            }
        }
    }

    fn draw_block(&self, block: &BlockWithPos, printer: &Printer) {
        for cell in block.cells() {
                printer.with_color(block.color(), |printer| {
                    printer.print((2*cell.x, cell.y), "  ");
                });
        }
    }

    fn draw_hint(&self, block: &BlockWithPos, printer: &Printer) {
        let color = ColorStyle::new(Color::Rgb(255,255,255), Color::Rgb(0,0,0));
        for cell in block.cells() {
                printer.with_color(color, |printer| {
                    printer.print((2*cell.x, cell.y), "░░");
                });
        }
    }

    fn draw_dangerous(&self, printer: &Printer) {
        let width = self.colors.width();
        let mut warning_stage = false;
        for j in 0..3 {
            for i in 0..width {
                if self.colors.is_occupied(i, j) {
                    warning_stage = true;
                    break;
                }
            }
            if warning_stage {
                break;
            }
        }
        if !warning_stage {
            return;
        }
        for j in 0..3 {
            for i in 0..width {
                printer.with_color(self.colors.warning_color, |printer| {
                    printer.print((2*i, j), "  ");
                });
            }
        }
    }
}