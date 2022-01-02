/// Re-export of webgl_generator apis so that users don't need to explicitly name it
/// as a dependency. This avoids version mismatches.
pub use webgl_generator::*;

fn write_header<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(
        dest,
        r#"
/// DO NOT CHANGE - THIS FILE IS GENERATED AUTOMATICALLY

// {registry:?}
use std::cell::RefCell;
use web_sys::WebGlRenderingContext;
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

pub struct WebSysGen;
impl Generator for WebSysGen {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write_header(registry, dest)?;
        Ok(())
    }
}
