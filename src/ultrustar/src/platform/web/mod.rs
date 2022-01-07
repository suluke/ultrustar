use js_sys::{Boolean, JsString, Map};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, HtmlCanvasElement, Node, WebGlRenderingContext};

#[path = "webgl_bindings.rs"]
pub mod gl;

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
    let ctx_opts = Map::new();
    ctx_opts.set(&JsString::from("depth"), &Boolean::from(false));
    let ctx: WebGlRenderingContext = canvas
        .get_context_with_context_options("webgl", &ctx_opts)?
        .unwrap()
        .dyn_into()?;
    gl::set_context(ctx);
    parent.append_child(&canvas.into())?;

    gl::ClearColor(0.0, 0.0, 0.0, 0.0);
    gl::Clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}
