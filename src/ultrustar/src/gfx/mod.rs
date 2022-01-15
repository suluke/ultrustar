/// Documentation trait
pub trait Renderer {
    /// Refresh the graphical representation
    fn render();
}

pub mod gl;
