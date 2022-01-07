use webgl_generator::{Enum, NamedType, Registry};

pub fn write<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    for (name, enum_) in registry.iter_types(NamedType::as_enum) {
        write_enum(name, enum_, registry, dest)?;
    }
    Ok(())
}

fn write_enum<W>(
    name: &str,
    _enum_: &Enum,
    _registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    let mut fixups = std::collections::BTreeMap::new();
    fixups.insert("WebGLPowerPreference", "WebGlPowerPreference");
    if let Some(&websys_alias) = fixups.get(name) {
        writeln!(dest, "pub use web_sys::{} as {};", websys_alias, name)?;
    } else {
        writeln!(dest, "pub use web_sys::{};", name)?;
    }
    Ok(())
}
