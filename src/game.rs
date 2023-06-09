use crate::backend::backend;
use crate::tetris::Tetris;

use cursive::{
    Cursive,
    view::{Nameable, Selector},
};

pub fn run() {
    let mut siv: Cursive = Cursive::new();

    let tetris = Tetris::new().with_name("tetris");

    siv.add_layer(tetris);
    siv.focus(&Selector::Name("tetris")).unwrap();

    siv.set_fps(1000);
    
    let siv: std::sync::Mutex<Cursive> = std::sync::Mutex::new(siv);
    siv.lock().unwrap().run_with(|| backend());
}