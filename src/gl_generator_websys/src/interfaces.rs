use crate::features::TOKEN_UNSAFE;

use super::{compat::FunctionAlternative, types};
use webgl_generator::{
    Argument, Interface, Member, NamedType, Operation, Registry, Type, TypeKind, VisitOptions,
};

pub fn write<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(dest, "use static_assertions::assert_eq_size;")?;
    for (name, interface) in registry.iter_types(NamedType::as_interface) {
        write_interface(name, interface, registry, dest)?;
    }
    Ok(())
}

struct IndexType {
    ty: &'static str,
    none_val: &'static str,
}
impl IndexType {
    pub fn gl_int() -> Self {
        Self {
            ty: "GLint",
            none_val: "-1",
        }
    }
    pub fn gl_uint() -> Self {
        Self {
            ty: "GLuint",
            none_val: "GLuint::MAX",
        }
    }
}

#[derive(Default)]
struct ClassInfo {
    indexed_as: Option<IndexType>,
}
impl ClassInfo {
    pub fn get(name: &str) -> Self {
        if name == "WebGLUniformLocation" {
            Self {
                indexed_as: Some(IndexType::gl_int()),
            }
        } else {
            Self {
                indexed_as: Some(IndexType::gl_uint()),
            }
        }
    }
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
    if let Some(idxty) = ClassInfo::get(name).indexed_as {
        let websys_name = name.replace("WebGL", "WebGl");
        writeln!(
            dest,
            "assert_eq_size!({}, web_sys::{});",
            idxty.ty, websys_name
        )?;
        writeln!(dest, "pub type {} = types::{};", name, idxty.ty)?;
    }
    Ok(())
}

/// There is at least one function that is only overloaded because it takes a pointer as argument
/// and web_sys wants to give the option to use either u32 or f64 to represent the pointer (because
/// js numbers and stuff). Since we typedef GL*ptr to u32 only one of those overloads is releveant
/// for us and we only create one binding for it.
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
        } else if matches!(
            &arg.type_.kind,
            TypeKind::ArrayBufferView | TypeKind::BufferSource
        ) {
            if arg.type_.optional {
                Some("opt_u8")
            } else {
                Some("u8")
            }
        } else {
            None
        }
    });
    let mut result = name.to_snake_case().replace("2_d", "_2d");
    // a generator for the conjunctions, used for generating e.g. "with_x_and_y_and_z"
    let mut conj_gen = Some("_with").iter().chain(Some("_and").iter().cycle());
    if has_pointer_arg {
        result += conj_gen.next().unwrap();
        result += "_i32";
    }
    if let Some(buff_ty) = has_buffer_arg {
        result += conj_gen.next().unwrap();
        result += &format!("_{}_array", buff_ty);
    }
    result
}

fn write_rendering_context<W>(
    ctx_ty_name: &str,
    interface: &Interface,
    registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
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
                    write_member_op(name, op, registry, dest)?;
                }
            } else if let Member::Const(constant) = member {
                writeln!(
                    dest,
                    "pub const {constant}: {ty} = web_sys::{name}::{constant};",
                    name = ctx_ty_name.replace("WebGL", "WebGl"),
                    constant = name,
                    ty = types::stringify_return(&constant.type_, registry)
                )?;
            }
        }
    }

    Ok(())
}

fn write_member_op<W>(
    name: &str,
    op: &Operation,
    registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    if FunctionAlternative::get(name).is_some() {
        // will be written by compat
        return Ok(());
    }
    const FALLIBLE_RESULTS: [&str; 1] = ["ReadPixels"];
    const OPS_WRONG_TYPES: [&str; 9] = [
        // Not in regular GLES
        "GetExtension",
        "GetParameter",
        // Return types should be out params
        "GetAttachedShaders",
        "GetSupportedExtensions",
        "GetProgramInfoLog",
        "GetShaderInfoLog",
        "GetShaderSource",
        // Need output parameter and multiple type overloads
        "GetFramebufferAttachmentParameter",
        "GetVertexAttrib",
    ];

    let websys_name = get_websys_function_name(name, op, registry);
    let disabled = OPS_WRONG_TYPES.contains(&name);
    if disabled {
        writeln!(dest, "#[allow(unused, non_snake_case)]")?;
    } else {
        writeln!(dest, "#[allow(non_snake_case)]")?;
    }
    write!(
        dest,
        "pub {unsafe_} fn {name}",
        name = name,
        unsafe_ = TOKEN_UNSAFE
    )?;
    write_args(&op.args, registry, dest)?;
    write_return_signature(op.return_type.as_ref(), registry, dest)?;
    writeln!(dest, " {{")?;
    if disabled {
        #[cfg(debug_assertions)]
        writeln!(dest, "    // {:?}", op)?;
        writeln!(dest, "    todo!()")?;
    } else {
        write_param_casts(&op.args, registry, dest)?;
        write!(dest, "    withctx!(CONTEXT, ctx, {{ctx.{}", websys_name)?;
        write_params(&op.args, dest)?;
        writeln!(dest, "}})")?;
        if FALLIBLE_RESULTS.contains(&name) {
            writeln!(dest, "    .handle_js_error()")?;
        }
        if let Some(retty) = &op.return_type {
            write_result_conversion(retty, registry, dest)?;
        }
    }
    writeln!(dest, "}}")?;
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
fn write_param_casts<W>(args: &[Argument], registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    for arg in args.iter() {
        let argname = escape_ident(&arg.name);
        if let TypeKind::String = &arg.type_.kind {
            writeln!(
                dest,
                "    let {argname} = std::ffi::CStr::from_ptr({argname}).to_str().unwrap();",
                argname = &argname
            )?;
        } else if let TypeKind::Named(name) = &arg.type_.kind {
            if name == "GLintptr" {
                writeln!(
                    dest,
                    "    let {argname} = {argname} as i32;",
                    argname = &argname
                )?;
            } else {
                let resolved = registry.resolve_type(name);
                if let NamedType::Interface(_) = resolved {
                    let argname = escape_ident(&arg.name);
                    writeln!(
                    dest,
                    "    let {argname}_ = std::mem::MaybeUninit::new(std::mem::transmute::<_, web_sys::{tyname}>({argname}));",
                    argname = &argname,
                    tyname = name.replace("WebGL", "WebGl")
                )?;
                    writeln!(
                        dest,
                        "    let {argname} = {argname}_.assume_init_ref();",
                        argname = &argname,
                    )?;
                    if arg.type_.optional {
                        writeln!(
                            dest,
                            "    let {argname} = Some({argname});",
                            argname = argname
                        )?;
                    }
                }
            }
        }
    }
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
fn escape_ident(ident: &str) -> String {
    if is_keyword(ident) {
        format!("{}_", ident)
    } else {
        ident.to_owned()
    }
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

fn write_return_signature<W>(
    return_type: Option<&Type>,
    registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    if let Some(retty) = return_type {
        write!(dest, " -> {}", types::stringify_return(retty, registry))?;
    }
    Ok(())
}

/// Adds conversion code if necessary in case `web_sys` returns web-native objects instead of rust-native ones.
/// E.g. `JsString -> String` or `Array -> Vec`
fn write_result_conversion<W>(
    retty: &Type,
    registry: &Registry,
    dest: &mut W,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    if let TypeKind::Sequence(ty) = &retty.kind {
        let ty_name = types::stringify_return(ty, registry);
        let convert = if let TypeKind::String = ty.kind {
            "unchecked_into::<js_sys::JsString>().as_string().unwrap()".to_owned()
        } else {
            format!("unchecked_into::<{}>()", ty_name)
        };
        if retty.optional {
            writeln!(
                dest,
                "    .map(|some| some.iter().map(|val| val.{}).collect::<Vec<_>>())",
                convert
            )?;
        } else {
            writeln!(
                dest,
                "    .iter().map(|val| val.{}).collect::<Vec<_>>()",
                convert
            )?;
        }
    }
    if let TypeKind::Named(name) = &retty.kind {
        let resolved = registry.resolve_type(name);
        if name.ends_with("ptr") {
            write!(dest, " as i32 as *const std::ffi::c_void")?;
        }
        if retty.optional {
            if let NamedType::Interface(_) = resolved {
                if let Some(idxty) = ClassInfo::get(name).indexed_as {
                    writeln!(
                        dest,
                        "    .map_or({}, |val| std::mem::transmute::<_, {}>(val))",
                        idxty.none_val, name
                    )?;
                }
            }
        }
    }
    Ok(())
}
