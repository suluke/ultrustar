///
mod core;
#[allow(unused)]
use self::core::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        use platform::{Platform, PlatformApi};
        type Settings = <Platform as PlatformApi>::Settings;
        let settings = Settings {};
        let pf = Platform::init(settings).unwrap();
        core::run(pf);
    }
}
