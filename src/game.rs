use crate::container::Container;
use crate::queue::Queue;
use cursive::{
    Cursive, CursiveExt,
    event::{Event}, view::Nameable,
    views::{Dialog, LinearLayout},
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();
    let container = Container::new().with_name("container");
    let queue = Queue::new().with_name("queue");
    let view = Dialog::around(
        LinearLayout::horizontal()
        .child(container)
        .child(queue)
    ).title("Tetris");

    siv.add_layer(view);

    // Schedule a tick function to be called every second
    siv.add_global_callback(Event::Refresh, |s| {
        let stopped = s.call_on_name("container", |v: &mut Container| v.handle_tick());
        if stopped.unwrap() {
            let mut queue = s.find_name::<Queue>("queue").unwrap();
            let block = queue.pop_and_spawn_new_block();
            s.call_on_name("container", |v: &mut Container| v.insert_new_block(block));
        }
    });
    siv.set_fps(1);

    // Start the Cursive event loop
    siv.run();
}