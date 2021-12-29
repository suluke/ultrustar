use std::{fs::File, path::Path};

fn platform_web() {
    use webgl_generator_websys::{Api, Exts, Registry, WebSysGen};

    let dest_dir = std::env::var("OUT_DIR").unwrap();
    let mut dest = File::create(&Path::new(&dest_dir).join("webgl_bindings.rs")).unwrap();

    let registry = Registry::new(Api::WebGl, Exts::ALL);
    registry.write_bindings(WebSysGen, &mut dest).unwrap();
}

fn main() {
    println!("{}", std::env::var("CARGO_CFG_TARGET_ARCH").unwrap());
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        platform_web();
    }
}
