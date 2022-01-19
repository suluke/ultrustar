/// Documentation trait
pub trait Renderer: Sized {
    /// Settings
    type InitSettings: crate::SettingsTrait;

    /// Error which may occur during initialization
    type InitError: std::error::Error;

    /// Create the Renderer
    ///
    /// # Errors
    ///
    /// Whenever the instantiation of the renderer fails
    fn new(settings: &Self::InitSettings) -> Result<Self, Self::InitError>;

    /// Refresh the graphical representation
    fn render(&self);
}

pub mod gl;
