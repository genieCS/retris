use cursive::{
    theme::{ColorStyle, Color},
};
use rand::Rng;
use crate::shape::{Shape};



pub struct Block {
    pub shape: Shape,
    pub color: ColorStyle,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            shape: Shape::default(),
            color: Self::color(),
        }
    }

    fn color() -> ColorStyle {
        let colors = Self::colors();
        colors[rand::thread_rng().gen_range(0..colors.len())]
    }

    fn colors() -> [ColorStyle; 6] { [
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(255, 0, 0)),     // Red
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(0, 255, 0)),     // Green
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(255, 255, 0)),   // Yellow
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(0, 0, 255)),     // Blue
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(255, 0, 255)),   // Magenta
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(0, 255, 255)),   // Cyan
        ]
    }
}