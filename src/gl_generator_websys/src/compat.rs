use crate::runtime::{patches, CONSTANTS};
use webgl_generator::Registry;

fn write_constants<W>(_registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(dest, "{}", CONSTANTS)?;
    Ok(())
}

#[derive(Clone)]
pub struct FunctionAlternative {
    name: &'static str,
    code: &'static str,
}
impl FunctionAlternative {
    pub fn get(name: &str) -> Option<Self> {
        Self::get_all().iter().cloned().find(|alt| alt.name == name)
    }
    pub fn get_all() -> [Self; 12] {
        [
            Self {
                name: "BufferData",
                code: patches::BUFFER_DATA,
            },
            Self {
                name: "BufferSubData",
                code: patches::BUFFER_SUB_DATA,
            },
            Self {
                name: "ShaderSource",
                code: patches::SHADER_SOURCE,
            },
            Self {
                name: "GetShaderInfoLog",
                code: patches::GET_SHADER_INFO_LOG,
            },
            Self {
                name: "GetProgramInfoLog",
                code: patches::GET_PROGRAM_INFO_LOG,
            },
            Self {
                name: "GetAttachedShaders",
                code: patches::GET_ATTACHED_SHADERS,
            },
            Self {
                name: "GetExtension",
                code: patches::GET_EXTENSION,
            },
            Self {
                name: "GetFramebufferAttachmentParameter",
                code: patches::GET_FRAMEBUFFER_ATTACHMENT_PARAMETER,
            },
            Self {
                name: "GetParameter",
                code: patches::GET_PARAMETER,
            },
            Self {
                name: "GetShaderSource",
                code: patches::GET_SHADER_SOURCE,
            },
            Self {
                name: "GetSupportedExtensions",
                code: patches::GET_SUPPORTED_EXTENSIONS,
            },
            Self {
                name: "GetVertexAttrib",
                code: patches::GET_VERTEX_ATTRIB,
            },
        ]
    }
}

pub fn write_typdefs<W>(_registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(dest, "pub type GLchar = i8;")?;
    writeln!(dest, "pub type GLdouble = f64;")?;
    writeln!(dest, "pub type GLint64 = i64;")?;
    Ok(())
}

fn write_functions<W>(_registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(dest, "{}", crate::runtime::POLYFILLS)?;
    for alt in FunctionAlternative::get_all() {
        writeln!(dest, "{}", alt.code)?;
    }
    Ok(())
}

pub fn write<W>(registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    write_constants(registry, dest)?;
    write_functions(registry, dest)?;
    Ok(())
}
