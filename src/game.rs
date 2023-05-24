use crate::tetris::Tetris;

use cursive::{
    Cursive, CursiveExt,
    view::{Nameable, Selector},
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    let tetris = Tetris::new().with_name("tetris");

    siv.add_layer(tetris);
    siv.focus(&Selector::Name("tetris")).unwrap();

    siv.set_fps(1000);

    siv.run();
}