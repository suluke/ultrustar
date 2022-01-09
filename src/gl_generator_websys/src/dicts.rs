use webgl_generator::{Dictionary, NamedType, Registry};

pub fn write<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    for (name, dictionary) in registry.iter_types(NamedType::as_dictionary) {
        write_dictionary(name, dictionary, registry, dest)?;
    }
    Ok(())
}

fn write_dictionary<W>(
    name: &str,
    dictionary: &Dictionary,
    _registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    if dictionary.is_hidden {
        return Ok(());
    }

    let mut fixups = std::collections::BTreeMap::new();
    fixups.insert("WebGLContextAttributes", "WebGlContextAttributes");
    if let Some(&websys_alias) = fixups.get(name) {
        writeln!(dest, "pub use web_sys::{} as {};", websys_alias, name)?;
    } else {
        writeln!(dest, "pub use web_sys::{};", name)?;
    }
    Ok(())
}
