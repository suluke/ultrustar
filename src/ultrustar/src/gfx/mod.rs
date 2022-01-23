use egui::epaint::ClippedShape;

use crate::platform::Platform;

/// Documentation trait
pub trait Renderer: Sized {
    /// Settings
    type InitSettings: crate::SettingsTrait;

    /// Error which may occur during initialization
    type InitError: Into<anyhow::Error>;

    /// Create the Renderer
    ///
    /// # Errors
    ///
    /// Whenever the instantiation of the renderer fails
    fn new(settings: &Self::InitSettings, platform: &Platform) -> Result<Self, Self::InitError>;

    fn get_window(&self) -> &crate::Window;

    /// Refresh the graphical representation
    fn render(&self, shapes: Vec<ClippedShape>);
}

pub mod gl;
