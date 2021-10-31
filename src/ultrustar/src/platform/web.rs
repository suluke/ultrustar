use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_into() {
    use web_sys::console;
    console::log_1(&"Hello wasm".into());
}
