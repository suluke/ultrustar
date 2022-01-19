use crate::{Event, EventLoop, Signals, UserData};
use anyhow::anyhow;
use directories::ProjectDirs;
use glutin::{ContextWrapper, PossiblyCurrent};
use log::info;
use std::{fs::File, path::PathBuf};
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

pub use gl;

pub struct Settings;

pub struct Platform {
    window: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: EventLoop,
}
fn get_userdata_path(user_id: &str) -> Result<PathBuf, anyhow::Error> {
    let proj_dirs = ProjectDirs::from("io.github", "suluke", "ultrustar")
        .ok_or_else(|| anyhow!("Failed to retrieve application directories"))?;

    let mut dest = proj_dirs.config_dir().to_owned();
    dest.push(format!("{}_user.json", user_id));
    Ok(dest)
}
impl super::PlatformApi for Platform {
    type Settings = Settings;

    type Renderer = crate::gfx::gl::RendererES2;
    type InitError = ();

    fn load_userdata(id: &str) -> Result<crate::UserData, anyhow::Error> {
        match File::open(get_userdata_path(id)?) {
            Ok(src) => {
                serde_json::from_reader::<_, crate::UserData>(src).map_err(anyhow::Error::from)
            }
            Err(err) => {
                if id == "default" && matches!(err.kind(), std::io::ErrorKind::NotFound) {
                    Ok(UserData::default())
                } else {
                    Err(anyhow::Error::from(err))
                }
            }
        }
    }

    fn persist_userdata(data: &crate::UserData) -> Result<(), anyhow::Error> {
        let dest = get_userdata_path(&data.user.id)?;
        let dest = std::fs::File::create(dest)?;

        #[cfg(not(debug_assertions))]
        serde_json::to_writer_pretty(dest, data);
        #[cfg(debug_assertions)]
        serde_json::to_writer(dest, data)?;
        Ok(())
    }

    fn init(_settings: Self::Settings) -> Result<Self, Self::InitError> {
        let event_loop = EventLoop::with_user_event();
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
        F: 'static + FnMut(&Event<'_>, &EventLoopWindowTarget<Signals>),
    {
        self.event_loop.run(move |mut ev, tgt, control_flow| {
            *control_flow = ControlFlow::Wait;
            if matches!(
                &ev,
                Event::UserEvent(Signals::Exit)
                    | Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    }
            ) {
                info!("Received exit event");
                *control_flow = ControlFlow::Exit;
                // Clients should be able to react to shutdown, but it makes no sense
                // to have more than one event to signal that.
                ev = Event::UserEvent(Signals::Exit);
            }
            main_loop(&ev, tgt);
            if matches!(&ev, Event::RedrawRequested(_)) {
                self.window.swap_buffers().unwrap();
            }
        });
    }

    fn create_renderer(&self) -> Self::Renderer {
        Self::Renderer {}
    }
}
