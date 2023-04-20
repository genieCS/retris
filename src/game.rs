use crate::container::Container;
use cursive::{
    Cursive, CursiveExt,
    event::{Event}, view::Nameable,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();
    let container = Container::new().with_name("container");
    siv.add_layer(container);

    // Schedule a tick function to be called every second
    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_name("container", |v: &mut Container| v.handle_tick());
    });
    siv.set_fps(1);

    // Start the Cursive event loop
    siv.run();
}