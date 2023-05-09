use crate::block::Block;
use cursive:: {
    Printer,
    View,
    theme::{Color, ColorStyle},
};
use std::collections::VecDeque;

pub struct Queue {
    pub blocks: VecDeque<Block>,
}

impl Queue {
    pub fn new() -> Self {
        let mut blocks = VecDeque::new();
        for _ in 0..3 {
            blocks.push_back(Block::default());
        }
        Self {
            blocks,
        }
    }

    pub fn renew(&mut self) {
        *self = Self::new();
    }

    pub fn pop_and_spawn_new_block(&mut self) -> Block {
        let block = self.blocks.pop_front().unwrap();
        self.blocks.push_back(Block::default());
        block
    }

    fn draw_blocks(&self, printer: &Printer) {
        let x_padding = 10;
        let mut y_padding = 7;
        for block in &self.blocks {
            for vector in &block.cells() {
                printer.with_color(block.color(), |printer| {
                    printer.print((2*vector.0 + x_padding , y_padding + vector.1), "_|");
                });
            }
            y_padding += 5;
        }
    }

    fn draw_container(&self, printer: &Printer) {
        let x_padding = 4;
        let y_padding = 6;
        let color_style = ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(255,255,255));
        for j in 0..15 {
                printer.with_color(color_style, |printer| {
                    printer.print((x_padding, y_padding + j), "|          |");
                });
        }
        printer.with_color(color_style, |printer| {
            printer.print((x_padding, y_padding + 15), "|__________|");
        });
    }
}

impl View for Queue {
    fn draw(&self, printer: &Printer) {
        self.draw_container(printer);
        self.draw_blocks(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(15, 20)
    }
}

