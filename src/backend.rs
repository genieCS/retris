use cursive::backends;
use cursive::backend::Backend;

#[cfg(feature = "blt-backend")]
pub fn backend() -> Box<dyn Backend> {
    backends::blt::Backend::init()
}

#[cfg(feature = "termion-backend")]
pub fn backend() -> Box<dyn Backend> {
    backends::termion::Backend::init().unwrap()
}

#[cfg(feature = "crossterm-backend")]
pub fn backend() -> Box<dyn Backend> {
    backends::crossterm::Backend::init().unwrap()
}

#[cfg(feature = "pancurses-backend")]
pub fn backend() -> Box<dyn Backend> {
    backends::curses::pan::Backend::init().unwrap()
}

#[cfg(feature = "ncurses-backend")]
pub fn backend() -> Box<dyn Backend> {
    backends::curses::n::Backend::init().unwrap()
}

#[cfg(feature = "wasm-backend")]
pub fn backend() -> Box<dyn Backend> {
    backends::wasm::Backend::init().unwrap()
}
