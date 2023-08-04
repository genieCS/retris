#[cfg(not(feature = "wasm-backend"))]
fn main() {
    retris::game::run();
}

#[cfg(feature = "wasm-backend")]
fn main() {}