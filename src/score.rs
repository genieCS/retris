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

    fn num2str(&self) -> Vec<String> {
        let mut score = self.score.to_string();
        if score.len() < 2 {
             score = format!("0{}", score);
        }
        numbers::display(&format!("{} {}", score, self.perfect), "/")
    }
}

impl View for Score {
    fn draw(&self, printer: &Printer) {
        for (y, line) in self.num2str().iter().enumerate() {
            printer.print((0, y), line);
        }
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let lines = self.num2str();
        cursive::Vec2::new(lines[0].len() + 3, lines.len())
    }
}