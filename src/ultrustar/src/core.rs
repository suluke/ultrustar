#[path = "./platform/mod.rs"]
pub mod platform;

use platform::gl;

#[path = "./gfx/mod.rs"]
pub mod gfx;

use platform::{Platform, PlatformApi};

/// Cross-platform `main` function
pub fn run(platform: Platform) {
    platform.run(move |_, _| {
        println!("Clear");
        #[allow(unsafe_code)]
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    });
}
