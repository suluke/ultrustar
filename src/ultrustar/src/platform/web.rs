use super::PlatformApi;
use js_sys::{Boolean, JsString, Map};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, Node, WebGlRenderingContext};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

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
#[allow(unused)]
fn create_main_loop(game: &Platform) -> Closure<dyn FnMut()> {
    let game = game.clone();
    Closure::wrap(Box::new(move || {
        let mut game = game.borrow_mut();
        if let Some(next) = game.main_loop.as_ref() {
            game.raf_handle = request_animation_frame(next);
        }
    }) as Box<dyn FnMut()>)
}

pub struct PlatformImpl {
    event_loop: Option<EventLoop<()>>,
    _window: Window,
    main_loop: Option<Closure<dyn FnMut()>>,
    raf_handle: i32,
    canvas: HtmlCanvasElement,
}
pub type Platform = Rc<RefCell<PlatformImpl>>;

pub struct Settings {
    mount_point: Node,
}

impl PlatformApi for Platform {
    type Settings = Settings;
    type Renderer = crate::core::gfx::gl::RendererES2;
    type InitError = JsValue;

    fn init(settings: Self::Settings) -> Result<Self, Self::InitError> {
        let canvas = create_canvas()?;
        settings.mount_point.append_child(canvas.unchecked_ref())?;
        let ctx_opts = Map::new();
        ctx_opts.set(&JsString::from("depth"), &Boolean::from(false));
        let ctx: WebGlRenderingContext = canvas
            .get_context_with_context_options("webgl", &ctx_opts)?
            .unwrap()
            .dyn_into()?;
        gl::set_context(ctx);

        let event_loop = EventLoop::new();
        use winit::platform::web::WindowBuilderExtWebSys;
        let window = WindowBuilder::new()
            .with_canvas(Some(canvas.clone()))
            .with_title("A fantastic window!")
            .build(&event_loop)
            .unwrap();

        let instance = Rc::new(RefCell::new(PlatformImpl {
            event_loop: Some(event_loop),
            _window: window,
            canvas,
            raf_handle: 0,
            main_loop: None,
        }));
        // TODO haven't figured out yet how to make winit play with requestAnimationFrame
        // let main_loop = create_main_loop(&instance);
        // {
        //     let mut instance = instance.borrow_mut();
        //     instance.raf_handle = request_animation_frame(&main_loop);
        //     instance.main_loop = Some(main_loop);
        // }

        Ok(instance)
    }
    fn run<F>(self, main_loop: F)
    where
        F: 'static + FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow),
    {
        self.borrow_mut().event_loop.take().unwrap().run(main_loop);
    }

    fn create_renderer(&self) -> Self::Renderer {
        todo!();
    }
}

#[wasm_bindgen]
pub struct Api {
    instance: Platform,
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
pub fn render_into(parent: Node) -> Result<Api, JsValue> {
    let settings = Settings {
        mount_point: parent,
    };
    let pf = Platform::init(settings).unwrap();
    crate::core::run(pf.clone());
    Ok(Api { instance: pf })
}
