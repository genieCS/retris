use cursive::{
    event::{Event, EventResult, Key},
    View, Vec2,
    Printer, 
    theme::{ColorStyle},
};
use crate::block::Block;
use crate::lrd::{ LR, LRD };
use crate::current::{Current, BOARD_WIDTH, BOARD_HEIGHT, BACKGROUND_FRONT, BACKGROUND_BACK};

pub struct Board {
    current: Current,
    colors: [[ColorStyle; BOARD_WIDTH]; BOARD_HEIGHT],
    x_padding: usize,
    y_padding: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            current: Current {
                block: Block::new(),
                pos: Vec2::new(4, 0),
            },
            colors: [[ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK); BOARD_WIDTH]; BOARD_HEIGHT],
            x_padding: 1,
            y_padding: 5,
        }
    }

    pub fn move_lrd(&mut self, lrd: LRD) -> bool {
        self.current.move_lrd(lrd, &self.colors)
    }

    pub fn on_down(&mut self, is_drop: bool) -> bool {
        let mut stopped = false;
        let mut hit_bottom = is_drop;
        while !stopped {
            hit_bottom = self.move_lrd(LRD::Down);
            stopped = hit_bottom || !is_drop;
        }
        hit_bottom && self.merge_block()
    }

    pub fn insert_new_block(&mut self, block: Block) {
        self.current = Current {
            block,
            pos: Vec2::new(4, 0),
        };
    }

    fn merge_block(&mut self) -> bool {
        self.fill_board_with_current_block();
        self.remove_rows_if_possible();
        true
    }

    fn remove_rows_if_possible(&mut self) {
        let background = ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK);
        let mut rows_to_remove = Vec::new();
        for (i, row) in self.colors.iter_mut().enumerate().rev() {
            let mut remove = true;
            for cell in row.iter() {
                if cell == &background {
                    remove = false;
                    break;
                }
            }
            if remove {
                rows_to_remove.push(i);
            }
        }
        self.remove_rows(rows_to_remove);
    }

    fn remove_rows(&mut self, rows_to_remove: Vec<usize>) {
        if rows_to_remove.is_empty() {
            return;
        }
        let mut fill_y = BOARD_HEIGHT - 1;
        let mut check_y = BOARD_HEIGHT - 1;
        for row in rows_to_remove {
            while check_y > row {
                if fill_y != check_y {
                    self.colors[fill_y] = self.colors[check_y];
                }
                fill_y -= 1;
                check_y -= 1;
            }
            check_y = row - 1;
        }
        while check_y > 0 {
            self.colors[fill_y] = self.colors[check_y];
            fill_y -= 1;
            check_y -= 1;
        }
        while fill_y > 0 {
            self.colors[fill_y] = [ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK); BOARD_WIDTH];
            fill_y -= 1;
        }
    }

    fn fill_board_with_current_block(&mut self) {
        for block in &self.current.block.shape.vectors() {
            let x = self.current.pos.x + block.x;
            let y = self.current.pos.y + block.y;
            self.colors[y][x] = self.current.block.color;
        }
    }

    fn handle_lr(&mut self, lr: LR) -> EventResult {
        self.move_lrd(lr.to_lrd());
        EventResult::Consumed(None)
    }

    pub fn rotate(&mut self) -> EventResult {
        self.current.rotate(&self.colors);
        EventResult::Consumed(None)
    }

}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
        self.draw_current_block(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(2*BOARD_WIDTH + 3, BOARD_HEIGHT + 10)
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
        for i in 0..BOARD_WIDTH {
            printer.print((2*i + 1 + self.x_padding, self.y_padding), "_");
        }
        for j in 0..BOARD_HEIGHT {
            for i in 0..BOARD_WIDTH {
                printer.with_color(self.colors[j][i], |printer| {
                    printer.print((2*i + self.x_padding + 1, j + self.y_padding + 1), "_|");
                });
            }
            printer.with_color(ColorStyle::new(BACKGROUND_FRONT, BACKGROUND_BACK), |printer| {
                printer.print((self.x_padding, j + self.y_padding + 1), "|");
            });
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