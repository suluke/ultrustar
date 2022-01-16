use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub use gl;

pub struct Settings;

pub struct Platform {
    window: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: EventLoop<()>,
}
impl super::PlatformApi for Platform {
    type Settings = Settings;

    type Renderer = crate::gfx::gl::RendererES2;
    type InitError = ();

    fn load_userdata() -> crate::UserData {
        todo!()
    }

    fn persist_userdata(_data: &crate::UserData) {
        todo!()
    }

    fn init(_settings: Self::Settings) -> Result<Self, Self::InitError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("Ultrustar");
        let gl_window = glutin::ContextBuilder::new()
            .build_windowed(window, &event_loop)
            .unwrap();

        #[allow(unsafe_code)]
        let gl_window = unsafe { gl_window.make_current().unwrap() };
        gl::load_with(|symbol| gl_window.get_proc_address(symbol));

        Ok(Self {
            window: gl_window,
            event_loop,
        })
    }

    fn run<F>(self, mut main_loop: F)
    where
        F: 'static
            + FnMut(&winit::event::Event<'_, ()>, &winit::event_loop::EventLoopWindowTarget<()>),
    {
        self.event_loop.run(move |ev, tgt, control_flow| {
            *control_flow = ControlFlow::Wait;
            main_loop(&ev, tgt);
            if matches!(ev, Event::RedrawRequested(_)) {
                self.window.swap_buffers().unwrap();
            }
        });
    }

    fn create_renderer(&self) -> Self::Renderer {
        Self::Renderer {}
    }
}
