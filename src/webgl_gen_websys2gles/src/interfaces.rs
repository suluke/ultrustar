use super::types;
use webgl_generator::{Argument, Interface, Member, NamedType, Operation, Registry, VisitOptions};

pub fn write<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    for (name, interface) in registry.iter_types(NamedType::as_interface) {
        write_interface(name, interface, registry, dest)?;
    }
    Ok(())
}

fn write_interface<W>(
    name: &str,
    interface: &Interface,
    registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    if interface.is_hidden {
        return Ok(());
    }
    writeln!(dest, "// {}", name)?;
    if name.starts_with("WebGL") && name.ends_with("RenderingContext") {
        write_rendering_context(name, interface, registry, dest)?;
    } else if name == "GLContext" {
        // No idea what this is
    } else if interface.has_class {
        write_class(name, dest)?;
    } else {
        writeln!(dest, "// {}", name)?;
    }
    Ok(())
}

fn write_class<W>(name: &str, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(
        dest,
        "pub use web_sys::{} as {};",
        name.replace("WebGL", "WebGl"),
        name
    )?;
    Ok(())
}

fn unoverload(name: &str, op: &Operation) -> Option<&'static str> {
    let mut fixups = std::collections::BTreeMap::new();
    fixups.insert("bufferData", |op: &Operation| {
        if op.args[1].name == "data" {
            None
        } else {
            Some("bufferData")
        }
    });
    fixups.get(name).and_then(|fixerupper| fixerupper(op))
}

fn write_rendering_context<W>(
    _name: &str,
    interface: &Interface,
    registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    const OPS_NOT_FOUND: [&str; 25] = [
        "buffer_data",
        "buffer_sub_data",
        "compressed_tex_image2_d",
        "compressed_tex_sub_image2_d",
        "copy_tex_image2_d",
        "copy_tex_sub_image2_d",
        "draw_elements",
        "framebuffer_texture2_d",
        "read_pixels",
        "uniform1fv",
        "uniform1iv",
        "uniform2fv",
        "uniform2iv",
        "uniform3fv",
        "uniform3iv",
        "uniform4fv",
        "uniform4iv",
        "uniform_matrix2fv",
        "uniform_matrix3fv",
        "uniform_matrix4fv",
        "vertex_attrib1fv",
        "vertex_attrib2fv",
        "vertex_attrib3fv",
        "vertex_attrib4fv",
        "vertex_attrib_pointer",
    ];
    const OPS_WRONG_TYPES: [&str; 7] = [
        "GetAttachedShaders",
        "GetExtension",
        "GetFramebufferAttachmentParameter",
        "GetParameter",
        "GetSupportedExtensions",
        "GetVertexAttrib",
        "GetVertexAttribOffset",
    ];
    for (name, members) in interface.collect_members(registry, &VisitOptions::default()) {
        for &member in &members {
            if let Member::Operation(op) = member {
                let name = if members.len() > 1 {
                    unoverload(name, op)
                } else {
                    Some(name)
                }
                .map(heck::ToUpperCamelCase::to_upper_camel_case);
                if let Some(name) = name {
                    use heck::ToSnakeCase;
                    let disabled = OPS_WRONG_TYPES.contains(&name.as_str())
                        || OPS_NOT_FOUND.contains(&name.to_snake_case().as_str());
                    if disabled {
                        write!(
                            dest,
                            "#[allow(unused, non_snake_case)] pub fn {name}",
                            name = name
                        )?;
                    } else {
                        write!(dest, "#[allow(non_snake_case)] pub fn {name}", name = name)?;
                    }
                    write_args(&op.args, registry, dest)?;
                    if let Some(retty) = &op.return_type {
                        write!(dest, " -> {} ", types::stringify_return(retty, registry))?;
                    }
                    writeln!(dest, " {{")?;
                    if disabled {
                        write!(dest, "    todo!()")?;
                    } else {
                        write!(
                            dest,
                            "    withctx!(CONTEXT, ctx, {{ctx.{}",
                            name.to_snake_case()
                        )?;
                        write_params(&op.args, dest)?;
                        writeln!(dest, "}})")?;
                    }
                    writeln!(dest, "}}")?;
                }
            }
        }
    }

    Ok(())
}

fn write_args<W>(args: &[Argument], registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    write!(dest, "(")?;
    for (idx, arg) in args.iter().enumerate() {
        write_ident(&arg.name, dest)?;
        write!(dest, ": {}", types::stringify_arg(&arg.type_, registry))?;
        if idx < args.len() - 1 {
            write!(dest, ", ")?;
        }
    }
    write!(dest, ")")?;
    Ok(())
}
fn write_params<W>(args: &[Argument], dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    write!(dest, "(")?;
    for (idx, arg) in args.iter().enumerate() {
        write_ident(&arg.name, dest)?;
        if idx < args.len() - 1 {
            write!(dest, ", ")?;
        }
    }
    write!(dest, ")")?;
    Ok(())
}

fn is_keyword(s: &str) -> bool {
    ["ref", "type"].contains(&s)
}
fn write_ident<W>(ident: &str, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    write!(dest, "{}", ident)?;
    if is_keyword(ident) {
        write!(dest, "_")?;
    }
    Ok(())
}
