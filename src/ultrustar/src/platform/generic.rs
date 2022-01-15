use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub use gl;

pub struct Settings;

pub struct Platform {
    _window: Window,
    event_loop: EventLoop<()>,
}
impl super::PlatformApi for Platform {
    type Settings = Settings;

    type Renderer = crate::gfx::gl::RendererES2;
    type InitError = ();

    fn init(_settings: Self::Settings) -> Result<Self, Self::InitError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Ultrustar")
            .build(&event_loop)
            .unwrap();
        Ok(Self {
            _window: window,
            event_loop,
        })
    }

    fn create_renderer(&self) -> Self::Renderer {
        Self::Renderer {}
    }

    fn run<F>(self, main_loop: F)
    where
        F: 'static
            + FnMut(
                winit::event::Event<'_, ()>,
                &winit::event_loop::EventLoopWindowTarget<()>,
                &mut winit::event_loop::ControlFlow,
            ),
    {
        self.event_loop.run(main_loop);
    }
}
