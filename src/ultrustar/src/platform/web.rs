use super::PlatformApi;
use crate::{gfx::Renderer, Event, EventLoop, Signals};
use anyhow::anyhow;
use js_sys::{Boolean, JsString, Map, Object as JsObject};
use log::info;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlCanvasElement, Node, Storage, WebGlRenderingContext};
use winit::{
    event_loop::{ControlFlow, EventLoopClosed, EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

pub use websys_gles2 as gl;

#[derive(Debug)]
pub struct ErrorFromJs(String);
impl std::fmt::Display for ErrorFromJs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
impl std::error::Error for ErrorFromJs {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.0
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
trait AsErrorFromJs {
    fn as_js_err(self) -> ErrorFromJs;
}
impl AsErrorFromJs for JsValue {
    fn as_js_err(self) -> ErrorFromJs {
        ErrorFromJs(JsString::from(self).as_string().unwrap())
    }
}
impl AsErrorFromJs for JsObject {
    fn as_js_err(self) -> ErrorFromJs {
        ErrorFromJs(self.to_string().as_string().unwrap())
    }
}
trait MapJsError<T> {
    fn map_js_error(self) -> Result<T, ErrorFromJs>;
}
impl<T> MapJsError<T> for Result<T, JsValue> {
    fn map_js_error(self) -> Result<T, ErrorFromJs> {
        self.map_err(JsValue::as_js_err)
    }
}
impl<T> MapJsError<T> for Result<T, JsObject> {
    fn map_js_error(self) -> Result<T, ErrorFromJs> {
        self.map_err(JsObject::as_js_err)
    }
}

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

fn local_storage() -> Result<Storage, ErrorFromJs> {
    Ok(window()
        .local_storage()
        .map_js_error()?
        .expect("should have localstorage"))
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
impl std::ops::Deref for EventLoopHandle {
    type Target = EventLoopWindowTarget<Signals>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(el) => el,
            Self::Proxy(_) =>
                todo!("Don't know how to get EventLoopWindowTarget from Proxy but maybe never becomes needed anyhow"),
        }
    }
}

pub struct GlWindow(Window);
impl GlWindow {
    pub fn swap_buffers(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
    pub fn window(&self) -> &Window {
        &self.0
    }
}

pub struct PlatformImpl {
    event_loop: EventLoopHandle,
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
    type GlWindow = GlWindow;

    fn init(settings: Self::Settings) -> Result<Self, Self::InitError> {
        let canvas = create_canvas()?;
        settings.mount_point.append_child(canvas.unchecked_ref())?;
        let event_loop = EventLoop::with_user_event();
        let instance = Rc::new(RefCell::new(PlatformImpl {
            event_loop: EventLoopHandle::Owned(event_loop),
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
    fn load_userdata(id: &str) -> Result<crate::UserData, anyhow::Error> {
        let storage = local_storage()?;
        if let Some(data) = storage.get_item(&format!("{}_user", id)).map_js_error()? {
            info!("Found existing data for user id {}", id);
            Ok(serde_json::from_str(&data)?)
        } else if id == "default" {
            info!("No persisted user data found for id {}. Using default.", id);
            Ok(crate::UserData::default())
        } else {
            Err(anyhow!("Failed to find data for user id {}", id))
        }
    }
    fn persist_userdata(data: &crate::UserData) -> Result<(), anyhow::Error> {
        let storage = local_storage()?;
        let json = serde_json::to_string(data)?;
        storage
            .set_item(&format!("{}_user", data.user.id), &json)
            .map_js_error()?;
        Ok(())
    }

    fn create_gl_window(&self) -> Result<Self::GlWindow, anyhow::Error> {
        let leme = self.borrow();
        let ctx_opts = Map::new();
        ctx_opts.set(&JsString::from("depth"), &Boolean::from(false));
        let ctx: WebGlRenderingContext = leme
            .canvas
            .get_context_with_context_options("webgl", &ctx_opts)
            .map_js_error()?
            .unwrap()
            .dyn_into()
            .map_js_error()?;
        ctx.get_extension("OES_element_index_uint").unwrap();
        gl::set_context(ctx);

        use winit::platform::web::WindowBuilderExtWebSys;
        let window = WindowBuilder::new()
            .with_canvas(Some(leme.canvas.clone()))
            .with_title("A fantastic window!")
            .build(&leme.event_loop)?;
        Ok(GlWindow(window))
    }

    fn create_renderer(
        &self,
        settings: &<Self::Renderer as Renderer>::InitSettings,
    ) -> Result<Self::Renderer, anyhow::Error> {
        Self::Renderer::new(settings, self)
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

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let settings = Settings {
        mount_point: parent,
    };
    let pf = Platform::init(settings).unwrap();
    let deferred_takeover = create_event_loop_takeover(&pf);
    window().set_timeout_with_callback(deferred_takeover.as_ref().unchecked_ref())?;
    deferred_takeover.forget();

    Ok(Api { instance: pf })
}
