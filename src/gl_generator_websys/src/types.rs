use webgl_generator::{NamedType, Registry, Type, TypeKind};

enum PassBy {
    Val,
    Ref,
}

struct RustType {
    name: String,
    pass_by: PassBy,
    optional: bool,
}
impl RustType {
    fn from_idl(ty: &Type, registry: &Registry) -> Self {
        let (name, pass_by, optional) = match &ty.kind {
            TypeKind::Primitive(p) => (p.name().into(), PassBy::Val, ty.optional),
            TypeKind::String => ("*const i8".into(), PassBy::Val, ty.optional),
            TypeKind::ArrayBuffer | TypeKind::ArrayBufferView | TypeKind::BufferSource => {
                ("mut [u8]".into(), PassBy::Ref, ty.optional)
            }
            TypeKind::CanvasElement => ("HtmlCanvasElement".into(), PassBy::Ref, ty.optional),
            TypeKind::TypedArray(p) => (format!("[{}]", p.name()), PassBy::Ref, ty.optional),
            TypeKind::Sequence(ty) => (
                format!("Vec<{}>", Self::from_idl(ty, registry).name),
                PassBy::Ref,
                ty.optional,
            ),
            // We mostly have Union(TypedArray | Sequence) which would map to &[PRIMITIVE] vs JsValue respectively in web_sys.
            // The latter is obviously not used in OpenGL so we flatten the Union to TypedArray only.
            TypeKind::Union(tys) => (
                tys.iter()
                    .find(|&ty| matches!(&ty.kind, TypeKind::TypedArray(_)))
                    .map(|ty| Self::from_idl(ty, registry).name)
                    .expect("Unsupported return type: Union without TypedArray"),
                PassBy::Ref,
                ty.optional,
            ),
            TypeKind::Named(name) => {
                let resolved = registry.resolve_type(name);
                match resolved {
                    NamedType::Mixin(_) => unimplemented!("Unsupported return type: Mixin"),
                    NamedType::Interface(_) => (name.clone(), PassBy::Val, false), // interfaces are all GLuint in GLES, None is -1/MAX
                    NamedType::Dictionary(_) => (name.clone(), PassBy::Ref, ty.optional),
                    NamedType::Typedef(ty) => {
                        let alias = Self::from_idl(ty, registry);
                        (name.clone(), alias.pass_by, alias.optional)
                    }
                    NamedType::Enum(_) => (name.clone(), PassBy::Val, ty.optional),
                    NamedType::Callback(_) => unimplemented!("Unsupported return type: Callback"),
                }
            }
            TypeKind::Any | TypeKind::Object => ("JsValue".into(), PassBy::Ref, ty.optional),
        };
        Self {
            name,
            pass_by,
            optional,
        }
    }
}

pub fn stringify_return(ty: &Type, registry: &Registry) -> String {
    let mapped = RustType::from_idl(ty, registry);
    if mapped.optional {
        format!("Option<{}>", mapped.name)
    } else {
        mapped.name
    }
}

pub fn stringify_arg(ty: &Type, registry: &Registry) -> String {
    let mapped = RustType::from_idl(ty, registry);
    let pass_spec = if let PassBy::Ref = mapped.pass_by {
        "&"
    } else {
        ""
    };
    if mapped.optional {
        format!("Option<{}{}>", pass_spec, mapped.name)
    } else if pass_spec.is_empty() {
        mapped.name
    } else {
        format!("{}{}", pass_spec, mapped.name)
    }
}

pub fn write_defs<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    let mut fixups = std::collections::BTreeMap::new();
    fixups.insert("GLintptr", "*const std::ffi::c_void");
    fixups.insert("GLsizeiptr", "i32");

    writeln!(dest, "pub mod types {{")?;
    writeln!(dest, "    use wasm_bindgen::JsValue;")?;
    for (name, ty) in registry.iter_types(NamedType::as_typedef) {
        let ty = if let Some(&fixup) = fixups.get(name.as_str()) {
            fixup.to_owned()
        } else {
            stringify_return(ty, registry)
        };
        writeln!(dest, "    pub type {name} = {ty};", name = name, ty = ty)?;
    }
    super::compat::write_typdefs(registry, dest)?;
    writeln!(dest, "}}")?;
    writeln!(dest, "use types::*;")?;
    Ok(())
}
