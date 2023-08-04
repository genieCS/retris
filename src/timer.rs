use crate::numbers;
use cursive::{
    View,
    Printer,
};

#[cfg(not(feature = "wasm-backend"))]
use std::time::Instant;
#[cfg(feature = "wasm-backend")]
use js_sys::Date;

pub struct Timer {
    #[cfg(not(feature = "wasm-backend"))]
    start: Instant,
    #[cfg(feature = "wasm-backend")]
    start: f64,
    is_paused: bool,
    #[cfg(not(feature = "wasm-backend"))]
    pause_start: Instant,
    #[cfg(feature = "wasm-backend")]
    pause_start: f64,

}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        let now = Self::now();
        Self {
            start: now,
            is_paused: false,
            pause_start: now,
        }
    }
    
    #[cfg(not(feature = "wasm-backend"))]
    fn now() -> Instant {
        Instant::now()
    }

    #[cfg(feature = "wasm-backend")]
    fn now() -> f64 {
        Date::now()
    }

    pub fn renew(&mut self) {
        let now = Self::now();
        self.start = now;
        self.pause_start = now;
        self.is_paused = false;
    }

    pub fn toggle_pause(&mut self) {
        let now = Self::now();
        if self.is_paused {
            self.start += now - self.pause_start;
        } else {
            self.pause_start = now;
        }
        self.is_paused = !self.is_paused;
    }

    pub fn reset(&mut self) {
        let now = Self::now();
        self.start = now;
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
        let mills = self.elapsed_mills();
        let mins = mills / 60000;
        let secs = (mills % 60000) / 1000;
        let mills = mills % 1000;
        (mins, secs, mills)
    }

    #[cfg(not(feature = "wasm-backend"))]
    fn elapsed_mills(&self) -> u128 {
        if self.is_paused {
            (self.pause_start - self.start).as_millis()
        } else {
            (Self::now() - self.pause_start).as_millis()
        }
    }

    #[cfg(feature = "wasm-backend")]
    fn elapsed_mills(&self) -> u128 {
        if self.is_paused {
            (self.pause_start - self.start) as u128
        } else {
            (Self::now() - self.pause_start) as u128
        }
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