use crate::tetris::Tetris;

use crate::manual::Manual;
use cursive::{
    Cursive, CursiveExt,
    view::{Nameable, Selector},
    views::{Dialog, LinearLayout, DummyView},
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    let manual = Manual::new();
    let tetris = Tetris::new()
    .with_name("tetris");

    let view = Dialog::around(
        LinearLayout::horizontal()
                .child(manual)
                .child(DummyView)
                .child(tetris)
    ).title("Tetris");

    siv.add_layer(view);
    siv.focus(&Selector::Name("tetris")).unwrap();

    siv.set_fps(1);

    siv.run();
}