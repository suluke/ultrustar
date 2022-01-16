#[path = "./platform/mod.rs"]
pub mod platform;

#[path = "./gfx/mod.rs"]
pub mod gfx;

use platform::{Platform, PlatformApi};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait SettingsTrait: Serialize + DeserializeOwned {}

type Renderer = <Platform as PlatformApi>::Renderer;
use gfx::Renderer as RendererApi;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    gfx: <Renderer as RendererApi>::InitSettings,
}
impl SettingsTrait for UserData {}

// FIXME
#[allow(clippy::missing_panics_doc)]
/// Cross-platform `main` function
pub fn run(platform: Platform) {
    let renderer = Renderer::new(Renderer::cfg()).unwrap();
    platform.run(move |_, _| {
        renderer.render();
    });
}
