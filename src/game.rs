use crate::tetris::Tetris;

use crate::manual::Manual;
use cursive::{
    Cursive, CursiveExt,
    event::{Event, Key},
    view::Nameable,
    views::{Dialog, LinearLayout, DummyView}, View,
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

    // let events = [
    //     Event::Key(Key::Up),
    //     Event::Key(Key::Down),
    //     Event::Key(Key::Left),
    //     Event::Key(Key::Right),
    //     Event::Char(' '),
    //     Event::Refresh,
    // ];

    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Refresh);
        });
    });
    siv.add_global_callback(Event::Key(Key::Up), |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Key(Key::Up));
        });
    });
    siv.add_global_callback(Event::Key(Key::Down), |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Key(Key::Down));
        });
    });
    siv.add_global_callback(Event::Key(Key::Left), |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Key(Key::Left));
        });
    });
    siv.add_global_callback(Event::Key(Key::Right), |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Key(Key::Right));
        });
    });
    siv.add_global_callback(Event::Char(' '), |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Char(' '));
        });
    });
    siv.add_global_callback(Event::Char('n'), |s| {
        s.call_on_name("tetris", |tetris: &mut Tetris| {
            tetris.on_event(Event::Char('n'));
        });
    });

    siv.add_layer(view);

    siv.set_fps(1);

    siv.run();
}