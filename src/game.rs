use crate::tetris::Tetris;

use cursive::{
    Cursive, CursiveExt,
    event::Event,
    view::Nameable,
    views::{Dialog}, View,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();
    let view = Dialog::around(
        Tetris::new()
            .with_name("tetris")
    ).title("Tetris");

    siv.add_layer(view);

    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Refresh)
        });
    });
    siv.set_fps(1);

    siv.run();
}