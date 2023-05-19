use cursive::{
    View,
    Printer,
};
use crate::numbers;

pub struct Score {
    score: usize,
    perfect: usize,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            perfect: 40,
        }
    }
}

impl Score {
    pub fn new() -> Score{
        Self::default()
    }

    pub fn add(&mut self, s: usize) {
        self.score += s
    }

    pub fn is_gameover(&self) -> bool {
        self.score >= self.perfect
    }

    fn num2str(&self) -> String {
        format!("Lines: {} / {}", numbers::padding(self.score, 2), self.perfect)
    }
}

impl View for Score {
    fn draw(&self, printer: &Printer) {
        printer.print((0, 0), &self.num2str());
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let line = self.num2str();
        cursive::Vec2::new(line.len() + 3, 2)
    }
}