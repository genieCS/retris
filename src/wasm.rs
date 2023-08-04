#![cfg(feature = "wasm-backend")]

use cursive::{
    Cursive,
    view::{Nameable, Selector},
};
use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use crate::backend;


#[wasm_bindgen]
pub struct CursiveWrapper {
    backend: Mutex<Cursive>,
}

#[wasm_bindgen]
impl CursiveWrapper {
    #[wasm_bindgen(js_name = "retris")]
    pub async fn retris() -> CursiveWrapper {
        let mut siv: Cursive = Cursive::new();
        let tetris = crate::tetris::Tetris::new().with_name("retris");
        siv.add_layer(tetris);
        siv.focus(&Selector::Name("retris")).unwrap();
        siv.set_fps(1000);
        let siv: Mutex<Cursive> = std::sync::Mutex::new(siv);
        siv.lock().unwrap().run_with(|| backend::backend()).await;
        CursiveWrapper { backend: siv }
    }
}