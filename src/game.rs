use crate::container::Container;
use cursive::{
    Cursive, CursiveExt,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let mut container = Container::new();
    container.drop_down();
    siv.add_layer(container);
    siv.run();
}