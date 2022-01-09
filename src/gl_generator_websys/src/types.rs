use webgl_generator::{NamedType, Primitive, Registry, Type, TypeKind};

enum PassBy {
    Val,
    Ref,
}

struct RustType {
    name: String,
    pass_by: PassBy,
}
impl RustType {
    fn from_idl(ty: &Type, registry: &Registry) -> Self {
        let (name, pass_by) = match &ty.kind {
            TypeKind::Primitive(p) => (p.name().into(), PassBy::Val),
            TypeKind::String => ("String".into(), PassBy::Ref),
            TypeKind::ArrayBuffer | TypeKind::ArrayBufferView | TypeKind::BufferSource => {
                ("ArrayBuffer".into(), PassBy::Ref)
            }
            TypeKind::CanvasElement => ("HtmlCanvasElement".into(), PassBy::Ref),
            TypeKind::TypedArray(p) => (
                match p {
                    Primitive::Bool | Primitive::I64 | Primitive::U64 => {
                        unimplemented!("Unsupported return type: TypedArray")
                    }
                    Primitive::I8 => "Int8Array",
                    Primitive::U8 => "Uint8Array",
                    Primitive::I16 => "Int16Array",
                    Primitive::U16 => "Uint16Array",
                    Primitive::I32 => "Int32Array",
                    Primitive::U32 => "Uint32Array",
                    Primitive::F32 => "Float32Array",
                    Primitive::F64 => "Float64Array",
                }
                .into(),
                PassBy::Ref,
            ),
            TypeKind::Sequence(ty) => (
                format!("Vec<{}>", Self::from_idl(ty, registry).name),
                PassBy::Ref,
            ),
            TypeKind::Union(tys) => (
                tys.iter()
                    .find(|&ty| matches!(&ty.kind, TypeKind::TypedArray(_)))
                    .map(|ty| Self::from_idl(ty, registry).name)
                    .expect("Unsupported return type: Union without TypedArray"),
                PassBy::Ref,
            ),
            TypeKind::Named(name) => {
                let resolved = registry.resolve_type(name);
                match resolved {
                    NamedType::Mixin(_) => unimplemented!("Unsupported return type: Mixin"),
                    NamedType::Interface(_) | NamedType::Dictionary(_) => {
                        (name.clone(), PassBy::Ref)
                    }
                    NamedType::Typedef(ty) => (name.clone(), Self::from_idl(ty, registry).pass_by),
                    NamedType::Enum(_) => (name.clone(), PassBy::Val),
                    NamedType::Callback(_) => unimplemented!("Unsupported return type: Callback"),
                }
            }
            TypeKind::Any | TypeKind::Object => ("JsValue".into(), PassBy::Ref),
        };
        Self { name, pass_by }
    }
}

pub fn stringify_return(ty: &Type, registry: &Registry) -> String {
    let mapped = RustType::from_idl(ty, registry);
    if ty.optional {
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
    if ty.optional {
        format!("Option<{}{}>", pass_spec, mapped.name)
    } else if pass_spec.is_empty() {
        mapped.name
    } else {
        format!("{}{}", pass_spec, mapped.name)
    }
}
