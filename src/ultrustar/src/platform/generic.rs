use anyhow::anyhow;
use std::{fs::File, path::PathBuf};

use directories::ProjectDirs;
use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub use gl;

use crate::UserData;

pub struct Settings;

pub struct Platform {
    window: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: EventLoop<()>,
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
