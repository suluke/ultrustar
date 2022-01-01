use std::{fs::File, path::PathBuf};

fn platform_web() {
    use webgl_generator_websys::{Api, Exts, Registry, WebSysGen};

    let mut dest_dir = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap());
    dest_dir.push("src/platform/web");
    let mut dest = File::create(&dest_dir.join("webgl_bindings.rs")).unwrap();

    let registry = Registry::new(Api::WebGl, Exts::ALL);
    registry.write_bindings(WebSysGen, &mut dest).unwrap();
}

fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        platform_web();
    }
}
