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

    pub fn renew(&mut self) {
        let now = Instant::now();
        self.timer = now;
        self.pause_start = now;
        self.is_paused = false;
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

    pub fn time2str(&self) -> String {
        let (mins, secs, mills) = self.elapsed();
        let mins = numbers::padding(mins as usize, 2);
        let secs = numbers::padding(secs as usize, 2);
        let mills = numbers::padding(mills as usize, 3);
        format!("Time {}:{}:{}", mins, secs, mills)
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
}

impl View for Timer {
    fn draw(&self, printer: &Printer) {
        printer.print((0, 0), &self.time2str());
    }

    fn required_size(&mut self, _: cursive::Vec2) -> cursive::Vec2 {
        let line = self.time2str();
        cursive::Vec2::new(line.len() + 3, 1)
    }
}