use super::PlatformApi;
use crate::{Event, EventLoop, Signals};
use js_sys::{Boolean, JsString, Map};
use log::info;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, Node, WebGlRenderingContext};
use winit::{
    event_loop::{ControlFlow, EventLoopClosed, EventLoopProxy, EventLoopWindowTarget},
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

fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let document = document();
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    Ok(canvas)
}

fn create_event_loop_takeover(platform: &Platform) -> Closure<dyn FnMut()> {
    let platform = platform.clone();
    Closure::once(Box::new(move || {
        crate::core::run(platform);
    }) as Box<dyn FnOnce()>)
}

enum EventLoopHandle {
    Owned(EventLoop),
    Proxy(EventLoopProxy<Signals>),
}
impl EventLoopHandle {
    fn take(&mut self) -> EventLoop {
        use EventLoopHandle::*;
        if let Owned(el) = match self {
            Owned(el) => {
                let mut proxy = Proxy(el.create_proxy());
                std::mem::swap(self, &mut proxy);
                proxy
            }
            Proxy(_) => panic!("Taking EventLoop more than once"),
        } {
            el
        } else {
            unreachable!("Will panic in earlier match");
        }
    }
    fn send_event(&self, signal: Signals) -> Result<(), EventLoopClosed<crate::Signals>> {
        use EventLoopHandle::*;
        match self {
            Owned(el) => el.create_proxy().send_event(signal),
            Proxy(pr) => pr.send_event(signal),
        }
    }
}

pub struct PlatformImpl {
    event_loop: EventLoopHandle,
    _window: Window,
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

        let event_loop = EventLoop::with_user_event();
        use winit::platform::web::WindowBuilderExtWebSys;
        let window = WindowBuilder::new()
            .with_canvas(Some(canvas.clone()))
            .with_title("A fantastic window!")
            .build(&event_loop)
            .unwrap();

        let instance = Rc::new(RefCell::new(PlatformImpl {
            event_loop: EventLoopHandle::Owned(event_loop),
            _window: window,
            canvas,
        }));

        Ok(instance)
    }
    fn run<F>(self, mut main_loop: F)
    where
        F: 'static + FnMut(&Event<'_>, &EventLoopWindowTarget<Signals>),
    {
        let el = self.borrow_mut().event_loop.take();
        el.run(move |ev, tgt, cf| {
            if matches!(&ev, Event::UserEvent(Signals::Exit)) {
                *cf = ControlFlow::Exit;
                info!("Received exit event");
            }
            main_loop(&ev, tgt);
        });
    }
    fn load_userdata(_id: &str) -> Result<crate::UserData, anyhow::Error> {
        Ok(crate::UserData::default())
    }
    fn persist_userdata(_data: &crate::UserData) -> Result<(), anyhow::Error> {
        todo!();
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
        info!("Send stop event");
        let _ = self.instance.borrow().event_loop.send_event(Signals::Exit);
    }
}

#[wasm_bindgen]
pub fn render_into(parent: Node) -> Result<Api, JsValue> {
    use log::Level;
    #[cfg(debug_assertions)]
    console_log::init_with_level(Level::Debug).expect("Failed to initialize logging framework");
    #[cfg(not(debug_assertions))]
    console_log::init_with_level(Level::Warn).expect("Failed to initialize logging framework");

    let settings = Settings {
        mount_point: parent,
    };
    let pf = Platform::init(settings).unwrap();
    let deferred_takeover = create_event_loop_takeover(&pf);
    window().set_timeout_with_callback(deferred_takeover.as_ref().unchecked_ref())?;
    deferred_takeover.forget();

    Ok(Api { instance: pf })
}
