pub mod backend;
pub mod block;
pub mod board;
pub mod game;
pub mod color_grid;
pub mod lrd;
pub mod manual;
pub mod numbers;
pub mod pause;
pub mod score;
pub mod tetris;
pub mod timer;
pub mod queue;

#[cfg(feature = "wasm-backend")]
pub mod wasm;

