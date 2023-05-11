use cursive::{
    event::{Event, EventResult, Key},
    View,
    Printer, 
    theme::{Color, ColorStyle},
};
use crate::block::{ Block, BlockWithPos };
use crate::color_grid::ColorGrid;
use crate::lrd::{ LR, LRD };

pub struct Board {
    colors: ColorGrid,
}

impl Board {
    pub fn new(background_color: (ColorStyle, ColorStyle), warning_color: ColorStyle, width: usize, height: usize) -> Self {
        Self {
            colors: ColorGrid::new(
                background_color,
                warning_color,
                width,
                height,
            ),
        }
    }

    pub fn renew(&mut self) {
        self.colors.renew();
    }

    pub fn move_block_lrd(&self, block: &BlockWithPos, lrd: LRD) -> (Option<BlockWithPos>, bool) {
        self.colors.move_block_lrd(block, lrd)
    }

    pub fn on_down(&mut self, is_drop: bool) -> (bool, bool, usize) {
        self.colors.on_down(is_drop)
    }

    pub fn insert(&mut self, block: Block) {
        self.colors.insert(block);
    }

    fn rotate(&mut self) -> EventResult {
        self.colors.rotate();
        EventResult::Consumed(None)
 
    }

    fn handle_lr(&mut self, lr: LR) -> EventResult {
        self.colors.handle_lr(lr);
        EventResult::Consumed(None)
    }

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

    fn draw_block(&self, printer: &Printer) {
        for cell in self.colors.block.cells() {
                printer.with_color(self.colors.block.color(), |printer| {
                    printer.print((2*cell.x, cell.y), "  ");
                });
        }
    }

    fn draw_hint(&self, printer: &Printer) {
        let hint = self.colors.hint();
        let color = ColorStyle::new(Color::Rgb(255,255,255), Color::Rgb(0,0,0));
        for cell in hint.cells() {
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

impl View for Board {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
        self.draw_dangerous(printer);
        self.draw_hint(printer);
        self.draw_block(printer)
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