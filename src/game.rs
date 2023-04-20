use crate::tetris::Tetris;

use cursive::{
    Cursive, CursiveExt,
    view::Nameable,
    views::{Dialog},
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();
    let view = Dialog::around(
        Tetris::new()
            .with_name("tetris")
    ).title("Tetris");

    siv.add_layer(view);

    siv.set_fps(1);

    siv.run();
}