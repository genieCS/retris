use cursive::{
    event::{Event, EventResult, Key},
    View, Vec2,
    Printer, 
    theme::{Color, ColorStyle},
};
use crate::block::Block;
use crate::current::Current;
use crate::color_grid::ColorGrid;
use crate::lrd::{ LR, LRD };

pub struct Board {
    current: Current,
    colors: ColorGrid,
}



impl Default for Board {
    fn default() -> Self {
        Self::new(
            ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(255,255,255)),
            ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(200,200,0)),
            10,
            20)
    }
}

impl Board {
    pub fn new(background_color: ColorStyle, warning_color: ColorStyle, width: usize, height: usize) -> Self {
        Self {
            current: Current::new(
                Block::default(),
                Vec2::new(width / 2, 0)
            ),
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
        self.current = Current::new(
            Block::default(),
            Vec2::new(self.colors.width() / 2, 0)
        );
    }

    pub fn move_lrd(&mut self, lrd: LRD) -> (bool, bool) {
        self.current.move_lrd(lrd, &self.colors)
    }

    pub fn on_down(&mut self, is_drop: bool) -> (bool, bool) {
        let mut stopped = false;
        let mut hit_bottom = is_drop;
        let mut moved;
        while !stopped {
            (moved, hit_bottom)= self.move_lrd(LRD::Down);
            if !moved {
                return (true, true)
            }
            stopped = hit_bottom || !is_drop;
        }
        (false, hit_bottom && self.colors.merge_block(&self.current))
    }

    pub fn insert_new_block(&mut self, block: Block) {
        self.current = Current::new(
            block,
             Vec2::new(self.colors.width() / 2, 0)
            );
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
        self.draw_dangerous(printer);
        let hint_color = ColorStyle::new(Color::Rgb(50,50,50), Color::Rgb(200,200,200));
        self.draw_block(&self.current.hint(&self.colors),hint_color, printer);
        self.draw_block(&self.current, self.current.color(), printer);
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
        for i in 0..width {
            printer.print((2*i + 1, 0), "_");
        }
        for j in 0..height {
            for i in 0..width {
                printer.with_color(self.colors[j][i], |printer| {
                    printer.print((2*i + 1, j + 1), "_|");
                });
            }
            printer.with_color(self.colors.background_color, |printer| {
                printer.print((0, j + 1), "|");
            });
        }
    }

    fn draw_block(&self, block: &Current, color: ColorStyle, printer: &Printer) {
        for cell in block {
                printer.with_color(color, |printer| {
                    printer.print((2*cell.x as usize + 1, cell.y as usize + 1), "_|");
                });
        }
    }

    fn draw_dangerous(&self, printer: &Printer) {
        let width = self.colors.width();
        let mut warning_stage = false;
        for j in 0..3 {
            for i in 0..width {
                if self.colors[j][i] != self.colors.background_color {
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
                    printer.print((2*i + 1, j + 1), "_|");
                });
            }
        }
    }
}