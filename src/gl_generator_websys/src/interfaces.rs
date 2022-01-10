use super::types;
use webgl_generator::{
    Argument, Interface, Member, NamedType, Operation, Registry, TypeKind, VisitOptions,
};

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

fn get_websys_function_name(name: &str, op: &Operation, registry: &Registry) -> String {
    use heck::ToSnakeCase;
    let has_pointer_arg = op.args.iter().any(|arg| {
        if let TypeKind::Named(name) = &arg.type_.kind {
            name.ends_with("ptr")
        } else {
            false
        }
    });
    let has_buffer_arg = op.args.iter().find_map(|arg| {
        if let TypeKind::Named(name) = &arg.type_.kind {
            let resolved = registry.resolve_type(name);
            if let NamedType::Typedef(aliased) = resolved {
                let aliased = if let TypeKind::Union(options) = &aliased.kind {
                    options
                        .iter()
                        .find(|ty| matches!(ty.kind, TypeKind::TypedArray(_)))
                        .unwrap()
                } else {
                    aliased
                };
                if let TypeKind::TypedArray(p) = &aliased.kind {
                    Some(p.name())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    });
    let mut result = name.to_snake_case().replace("2_d", "_2d");
    if has_pointer_arg {
        result += "_with_f64";
    }
    if let Some(buff_ty) = has_buffer_arg {
        result += &format!("_with_{}_array", buff_ty);
    }
    result
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
    const OPS_NOT_FOUND: [&str; 4] = [
        "buffer_sub_data_with_f64",
        "compressed_tex_image_2d",
        "compressed_tex_sub_image_2d",
        "read_pixels",
    ];
    const OPS_WRONG_TYPES: [&str; 6] = [
        "GetAttachedShaders",
        "GetExtension",
        "GetFramebufferAttachmentParameter",
        "GetParameter",
        "GetSupportedExtensions",
        "GetVertexAttrib",
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
                if let Some(name) = &name {
                    let websys_name = get_websys_function_name(name, op, registry);
                    let disabled = OPS_WRONG_TYPES.contains(&name.as_str())
                        || OPS_NOT_FOUND.contains(&websys_name.as_str());
                    if disabled {
                        writeln!(dest, "#[allow(unused, non_snake_case)]")?;
                    } else {
                        writeln!(dest, "#[allow(non_snake_case)]")?;
                    }
                    write!(dest, "pub fn {name}", name = name)?;
                    write_args(&op.args, registry, dest)?;
                    if let Some(retty) = &op.return_type {
                        write!(dest, " -> {}", types::stringify_return(retty, registry))?;
                    }
                    writeln!(dest, " {{")?;
                    if disabled {
                        #[cfg(debug_assertions)]
                        writeln!(dest, "    // {:?}", op)?;
                        writeln!(dest, "    todo!()")?;
                    } else {
                        write!(dest, "    withctx!(CONTEXT, ctx, {{ctx.{}", websys_name)?;
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
