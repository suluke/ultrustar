use crate::platform::gl;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InitSettings;
impl crate::SettingsTrait for InitSettings {}

#[derive(Debug)]
pub struct InitError;
pub struct Renderer;

impl crate::gfx::Renderer for Renderer {
    fn cfg() -> Self::InitSettings {
        InitSettings
    }

    type InitSettings = InitSettings;

    type InitError = InitError;

    fn new(_settings: Self::InitSettings) -> Result<Self, Self::InitError> {
        Ok(Self)
    }

    fn render(&self) {
        #[allow(unsafe_code)]
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
