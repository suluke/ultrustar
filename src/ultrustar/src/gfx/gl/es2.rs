use std::fmt::Display;

use crate::platform::gl;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InitSettings;
impl crate::SettingsTrait for InitSettings {}

#[derive(Debug)]
pub struct InitError;
impl Display for InitError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for InitError {}
pub struct Renderer;

impl crate::gfx::Renderer for Renderer {
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
