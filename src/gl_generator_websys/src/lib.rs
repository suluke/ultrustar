#![deny(
    unsafe_code,
    unused_imports,
    clippy::all,
    clippy::complexity,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

/// Re-export of `webgl_generator` apis so that users don't need to explicitly name it
/// as a dependency. This avoids version mismatches.
pub use webgl_generator::*;

mod dicts;
mod interfaces;
mod types;

#[cfg(not(debug_assertions))]
fn write_dbginfo<W>(_registry: &Registry, _dest: &mut W) -> std::io::Result<()> {
    Ok(())
}
#[cfg(debug_assertions)]
fn write_dbginfo<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(dest, "// {:?}", registry)?;
    Ok(())
}

fn write_header<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(
        dest,
        "// DO NOT CHANGE - THIS FILE IS GENERATED AUTOMATICALLY"
    )?;
    write_dbginfo(registry, dest)?;
    writeln!(
        dest,
        r#"
use std::cell::RefCell;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

thread_local!(static CONTEXT: RefCell<Option<WebGlRenderingContext>> = RefCell::new(None));

pub fn set_context(ctx: WebGlRenderingContext) {{
    CONTEXT.with(|tls_ctx| {{
        *tls_ctx.borrow_mut() = Some(ctx);
    }});
}}
macro_rules! withctx {{
    ($global:ident, $local:ident, $code:block) => {{
        $global.with(|ctx| {{
            let scope = ctx.borrow();
            let $local = scope.as_ref().expect("WebGlRenderingContext not set for current thread");
            $code
        }})
    }};
}}
"#
    )?;
    Ok(())
}

fn write_typedefs<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    let mut fixups = std::collections::BTreeMap::new();
    fixups.insert("GLintptr", "f64");
    fixups.insert("GLsizeiptr", "f64");

    writeln!(dest, "pub mod types {{")?;
    writeln!(dest, "    use wasm_bindgen::JsValue;")?;
    for (name, ty) in registry.iter_types(NamedType::as_typedef) {
        let ty = if let Some(&fixup) = fixups.get(name.as_str()) {
            fixup.to_owned()
        } else {
            types::stringify_return(ty, registry)
        };
        writeln!(dest, "    pub type {name} = {ty};", name = name, ty = ty)?;
    }
    writeln!(dest, "}}")?;
    writeln!(dest, "use types::*;")?;
    Ok(())
}

pub struct WebSysGen;
impl Generator for WebSysGen {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write_header(registry, dest)?;
        write_typedefs(registry, dest)?;
        dicts::write(registry, dest)?;
        interfaces::write(registry, dest)?;
        Ok(())
    }
}
