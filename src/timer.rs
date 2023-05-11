use crate::numbers;
use std::time::Instant;
use cursive::{
    View,
    Printer,
};

pub struct Timer {
    timer: Instant,
    is_paused: bool,
    pause_start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            timer: now,
            is_paused: false,
            pause_start: now,
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.is_paused {
            self.timer += Instant::now() - self.pause_start;
        } else {
            self.pause_start = Instant::now();
        }
        self.is_paused = !self.is_paused;
    }

    pub fn reset(&mut self) {
        let now = Instant::now();
        self.timer = now;
        self.pause_start = now;
    }

    pub fn time2str(&self) -> Vec<String> {
        let (mins, secs, mills) = self.elapsed();
        self.str(mins, secs, mills)

    }

    fn elapsed(&self) -> (u128, u128, u128) {
        let mills = if self.is_paused {
            (self.pause_start - self.timer).as_millis()
        } else { self.timer.elapsed().as_millis() };
        let mins = mills / 60000;
        let secs = (mills % 60000) / 1000;
        let mills = mills % 1000;
        (mins, secs, mills)
    }

    fn str(&self, mins: u128, secs: u128, mills: u128) -> Vec<String> {
        let mins = numbers::padding(mins as usize, 2);
        let secs = numbers::padding(secs as usize, 2);
        let mills = numbers::padding(mills as usize, 3);
        numbers::display(&format!("{} {} {}", mins, secs, mills), ":", false)
    }
}

impl View for Timer {
    fn draw(&self, printer: &Printer) {
        for (y, line) in self.time2str().iter().enumerate() {
            printer.print((0, y), line);
        }    
    }

    fn required_size(&mut self, _: cursive::Vec2) -> cursive::Vec2 {
        let lines = self.time2str();
        cursive::Vec2::new(lines[0].len() + 3, lines.len())    
    }
}