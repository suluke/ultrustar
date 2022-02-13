#![deny(
    unsafe_code,
    unused_imports,
    clippy::all,
    clippy::complexity,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]
#![allow(clippy::module_name_repetitions)]

#[path = "./platform/mod.rs"]
pub mod platform;

#[path = "./gfx/mod.rs"]
pub mod gfx;

#[path = "./ui/mod.rs"]
pub mod ui;

#[path = "./model/mod.rs"]
pub mod model;

use log::info;
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

pub type Window = winit::window::Window;
pub type EventLoop = winit::event_loop::EventLoop<Signals>;
pub type Event<'a> = winit::event::Event<'a, Signals>;

#[derive(Default, Serialize, Deserialize)]
pub struct UserData {
    #[serde(default)]
    user: User,
    #[serde(default)]
    gfx: <Renderer as RendererApi>::InitSettings,
    #[serde(default)]
    ui: ui::MainUISettings,
    #[serde(default)]
    library: model::library::Settings,
}
impl SettingsTrait for UserData {}

/// Cross-platform `main` function
///
/// # Panics
///
/// If an error bubbles up it will be written to log and converted into a panic
pub fn run(platform: Platform) {
    // wrap code in IIFE to write any errors to log before panicing
    (|| -> anyhow::Result<()> {
        let userdata = Platform::load_userdata("default")?;
        let renderer = platform.create_renderer(&userdata.gfx)?;
        let library = model::Library::init(&userdata.library);
        let mut main_ui = ui::MainUI::new(&userdata.ui, renderer.get_window());
        info!("Library with {} songs", library.len());
        platform.run(move |event, _| match event {
            Event::RedrawRequested(_) => main_ui.render(&renderer),
            Event::UserEvent(Signals::Exit) => {
                Platform::persist_userdata(&userdata).expect("Persisting settings failed");
            }
            Event::WindowEvent { event, .. } => main_ui.push_event(event),
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
