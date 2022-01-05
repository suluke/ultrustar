/// Re-export of `webgl_generator` apis so that users don't need to explicitly name it
/// as a dependency. This avoids version mismatches.
pub use webgl_generator::*;

mod types;

fn write_header<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(
        dest,
        r#"
// DO NOT CHANGE - THIS FILE IS GENERATED AUTOMATICALLY

// {registry:?}

#![allow(unused_imports)] // FIXME shouldn't be necessary
use std::cell::RefCell;
use wasm_bindgen::{{JsValue}};
use web_sys::{{HtmlCanvasElement, WebGlRenderingContext}};
use js_sys::{{
    ArrayBuffer,
    JsString,
    Int8Array,
    Uint8Array,
    Int16Array,
    Uint16Array,
    Int32Array,
    Uint32Array,
    Float32Array,
    Float64Array
}};

thread_local!(static CONTEXT: RefCell<Option<WebGlRenderingContext>> = RefCell::new(None));

pub fn set_context(ctx: WebGlRenderingContext) {{
    CONTEXT.with(|tls_ctx| {{
        *tls_ctx.borrow_mut() = Some(ctx);
    }});
}}
"#,
        registry = registry
    )?;
    Ok(())
}

fn write_typedefs<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    for (name, ty) in registry.iter_types(NamedType::as_typedef) {
        writeln!(
            dest,
            r#"#[allow(dead_code)] pub type {name} = {ty};"#,
            name = name,
            ty = types::stringify_return(ty, registry)
        )?;
    }
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
        Ok(())
    }
}
