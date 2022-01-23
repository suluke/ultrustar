use crate::{Event, Signals};
use winit::event_loop::EventLoopWindowTarget;

/// Documentation trait for what APIs a platform implementation needs to expose
#[allow(clippy::module_name_repetitions)]
pub trait PlatformApi: Sized {
    /// Platform-specific initialization settings
    type Settings;

    /// The (possibly polymorphic) type of renderer to be used
    type Renderer: crate::gfx::Renderer;

    /// Representation of errors that may occur during initialization
    type InitError;

    /// Implementation of a window offering `OpenGl` functionality on the current platform.
    type GlWindow: Sized;

    /// Create a new window which offers `OpenGl` graphics capabilities
    ///
    /// # Errors
    ///
    /// Return an error if the creation of the gl-capable window failed.
    fn create_gl_window(&self) -> Result<Self::GlWindow, anyhow::Error>;

    /// Load user data from persistent storage
    ///
    /// # Errors
    ///
    /// In case user data cannot be loaded and requested user is not "default", either.
    fn load_userdata(id: &str) -> Result<crate::UserData, anyhow::Error>;

    /// Store user data to persistent storage
    ///
    /// # Errors
    ///
    /// In case I/O failed
    fn persist_userdata(data: &crate::UserData) -> Result<(), anyhow::Error>;

    /// Initializes (instantiates) the platform
    ///
    /// # Errors
    ///
    /// Platform-specific initialization errors
    fn init(settings: Self::Settings) -> Result<Self, Self::InitError>;

    /// Allow hooking into the event loop
    fn run<F>(self, main_loop: F)
    where
        F: 'static + FnMut(&Event<'_>, &EventLoopWindowTarget<Signals>);

    /// Create a renderer instance
    ///
    /// # Errors
    ///
    /// Forwards any errors of the underlying graphics subsystem which prevent
    /// successful instantiation.
    fn create_renderer(
        &self,
        settings: &<Self::Renderer as crate::gfx::Renderer>::InitSettings,
    ) -> Result<Self::Renderer, anyhow::Error>;
}

/// Other platform implementation requirements:
/// *
///
/// optional:
/// * module `gl` for [`gfx::gl::RendererES2`|`gfx::gl::RendererES2`]

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
mod generic;
#[cfg(not(target_arch = "wasm32"))]
pub use generic::*;
