use winit::{event::Event, event_loop::EventLoopWindowTarget};

/// Documentation trait for what APIs a platform implementation needs to expose
#[allow(clippy::module_name_repetitions)]
pub trait PlatformApi: Sized {
    /// Platform-specific initialization settings
    type Settings;

    /// The (possibly polymorphic) type of renderer to be used
    type Renderer: crate::gfx::Renderer;

    /// Representation of errors that may occur during initialization
    type InitError;

    /// Load user data from persistent storage
    fn load_userdata() -> crate::UserData;

    /// Store user data to persistent storage
    fn persist_userdata(data: &crate::UserData);

    /// Initializes (instantiates) the platform
    ///
    /// # Errors
    ///
    /// Platform-specific initialization errors
    fn init(settings: Self::Settings) -> Result<Self, Self::InitError>;

    /// Allow hooking into the event loop
    fn run<F>(self, main_loop: F)
    where
        F: 'static + FnMut(&Event<'_, ()>, &EventLoopWindowTarget<()>);

    /// Create a renderer instance
    fn create_renderer(&self) -> Self::Renderer;
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
