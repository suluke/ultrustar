use js_sys::{Boolean, JsString, Map};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, Node, WebGlRenderingContext};

pub use websys_gles2 as gl;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}
fn cancel_animation_frame(raf_handle: i32) {
    window()
        .cancel_animation_frame(raf_handle)
        .expect("should register `cancel_animation_frame` OK")
}

fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let document = document();
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    Ok(canvas)
}

/// Creates and starts a self-repeating
//
fn create_main_loop(game: &Rc<RefCell<Instance>>) -> Closure<dyn FnMut()> {
    let game = game.clone();
    Closure::wrap(Box::new(move || {
        #[allow(unsafe_code)]
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        }
        let mut game = game.borrow_mut();
        if let Some(next) = game.main_loop.as_ref() {
            game.raf_handle = request_animation_frame(next);
        }
    }) as Box<dyn FnMut()>)
}

struct Instance {
    main_loop: Option<Closure<dyn FnMut()>>,
    raf_handle: i32,
    canvas: HtmlCanvasElement,
}
impl Instance {
    fn new(canvas: HtmlCanvasElement) -> Result<Rc<RefCell<Self>>, JsValue> {
        let ctx_opts = Map::new();
        ctx_opts.set(&JsString::from("depth"), &Boolean::from(false));
        let ctx: WebGlRenderingContext = canvas
            .get_context_with_context_options("webgl", &ctx_opts)?
            .unwrap()
            .dyn_into()?;
        gl::set_context(ctx);
        let instance = Rc::new(RefCell::new(Self {
            canvas,
            raf_handle: 0,
            main_loop: None,
        }));
        let main_loop = create_main_loop(&instance);
        {
            let mut instance = instance.borrow_mut();
            instance.raf_handle = request_animation_frame(&main_loop);
            instance.main_loop = Some(main_loop);
        }
        Ok(instance)
    }
}

#[wasm_bindgen]
pub struct Api {
    instance: Rc<RefCell<Instance>>,
}
#[wasm_bindgen]
impl Api {
    /// Changes the application canvas size
    pub fn resize(&self, width: u32, height: u32) {
        self.instance.borrow().canvas.set_width(width);
        self.instance.borrow().canvas.set_height(height);
    }
    /// Ends the application
    pub fn stop(&mut self) {
        cancel_animation_frame(self.instance.borrow().raf_handle);
        (*self.instance.borrow_mut()).main_loop = None;
    }
}

#[wasm_bindgen]
pub fn render_into(parent: &Node) -> Result<Api, JsValue> {
    let canvas = create_canvas()?;
    parent.append_child(canvas.unchecked_ref())?;
    Instance::new(canvas).map(|instance| Api { instance })
}
