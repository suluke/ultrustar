use crate::platform::{gl, Platform, PlatformApi};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InitSettings;
impl crate::SettingsTrait for InitSettings {}

pub struct Renderer {
    window: <Platform as PlatformApi>::GlWindow,
}

impl crate::gfx::Renderer for Renderer {
    type InitSettings = InitSettings;

    type InitError = anyhow::Error;

    fn new(_settings: &Self::InitSettings, platform: &Platform) -> Result<Self, Self::InitError> {
        let window = platform.create_gl_window()?;
        Ok(Self { window })
    }

    fn get_window(&self) -> &crate::Window {
        self.window.window()
    }

    fn render(&self) {
        #[allow(unsafe_code)]
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.window.swap_buffers().unwrap();
    }
}
