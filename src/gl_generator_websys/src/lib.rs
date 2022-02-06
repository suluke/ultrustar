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

mod compat;
mod dicts;
mod features;
mod interfaces;
mod runtime;
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
    writeln!(dest, "{}", *runtime::PRELUDE)?;
    Ok(())
}

pub struct WebSysGen;
impl Generator for WebSysGen {
    fn write<W>(&self, registry: &Registry, dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        write_header(registry, dest)?;
        types::write_defs(registry, dest)?;
        dicts::write(registry, dest)?;
        interfaces::write(registry, dest)?;
        compat::write(registry, dest)?;
        Ok(())
    }
}
