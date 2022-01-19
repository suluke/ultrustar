#[path = "./platform/mod.rs"]
pub mod platform;

#[path = "./gfx/mod.rs"]
pub mod gfx;

use platform::{Platform, PlatformApi};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait SettingsTrait: Default + Serialize + DeserializeOwned {}

type Renderer = <Platform as PlatformApi>::Renderer;
use gfx::Renderer as RendererApi;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: "default".into(),
            name: whoami::username(),
        }
    }
}

#[allow(unused)]
pub enum Signals {
    Exit,
}
pub type EventLoop = winit::event_loop::EventLoop<Signals>;
pub type Event<'a> = winit::event::Event<'a, Signals>;

#[derive(Default, Serialize, Deserialize)]
pub struct UserData {
    user: User,
    gfx: <Renderer as RendererApi>::InitSettings,
}
impl SettingsTrait for UserData {}

// FIXME
#[allow(clippy::missing_panics_doc)]
/// Cross-platform `main` function
pub fn run(platform: Platform) {
    // wrap code in IIFE to write any errors to log before panicing
    (|| -> anyhow::Result<()> {
        let userdata = Platform::load_userdata("default")?;
        let renderer = Renderer::new(&userdata.gfx)?;
        platform.run(move |event, _| match event {
            Event::RedrawRequested(_) => {
                renderer.render();
            }
            Event::UserEvent(Signals::Exit) => {
                Platform::persist_userdata(&userdata).expect("Persisting settings failed");
            }
            _ => (),
        });
        Ok(())
    })()
    .map_err(|err| {
        use log::error;
        error!("{}", &err);
        err
    })
    .unwrap();
}
