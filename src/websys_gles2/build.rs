use std::{fs::File, path::PathBuf};

use gl_generator_websys::{Api, Exts, Registry, WebSysGen};

fn main() {
    let mut dest_dir = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap());
    dest_dir.push("src");
    let mut dest = File::create(&dest_dir.join("webgl_bindings.rs")).unwrap();

    let registry = Registry::new(Api::WebGl, Exts::ALL);
    registry.write_bindings(WebSysGen, &mut dest).unwrap();
}
