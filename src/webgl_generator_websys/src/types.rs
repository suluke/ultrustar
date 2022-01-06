use webgl_generator::{NamedType, Primitive, Registry, Type, TypeKind};

pub fn stringify_return(ty: &Type, registry: &Registry) -> String {
    let base_ty = match &ty.kind {
        TypeKind::Primitive(p) => p.name().into(),
        TypeKind::String => "JsString".into(),
        TypeKind::ArrayBuffer | TypeKind::ArrayBufferView => "ArrayBuffer".into(),
        TypeKind::BufferSource => unimplemented!("Unsupported return type: BufferSource"),
        TypeKind::CanvasElement => "HtmlCanvasElement".into(),
        TypeKind::TypedArray(p) => match p {
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
        TypeKind::Sequence(ty) => format!("Vec<{}>", stringify_return(ty, registry)),
        TypeKind::Union(tys) => tys
            .iter()
            .find(|&ty| matches!(&ty.kind, TypeKind::TypedArray(_)))
            .map(|ty| stringify_return(ty, registry))
            .expect("Unsupported return type: Union without TypedArray"),
        TypeKind::Named(name) => match registry.resolve_type(name) {
            NamedType::Mixin(_) => unimplemented!("Unsupported return type: Mixin"),
            NamedType::Interface(_)
            | NamedType::Dictionary(_)
            | NamedType::Enum(_)
            | NamedType::Typedef(_) => name.clone(),
            NamedType::Callback(_) => unimplemented!("Unsupported return type: Callback"),
        },
        TypeKind::Any | TypeKind::Object => "JsValue".into(),
    };
    if ty.optional {
        format!("Option<{}>", base_ty)
    } else {
        base_ty
    }
}
