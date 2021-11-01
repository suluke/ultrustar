use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, HtmlCanvasElement, Node};

fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    Ok(canvas)
}

#[wasm_bindgen]
pub fn render_into(parent: &Node) -> Result<(), JsValue> {
    console::log_1(&"Hello wasm".into());
    let canvas = create_canvas()?;
    parent.append_child(&canvas.into())?;
    Ok(())
}
