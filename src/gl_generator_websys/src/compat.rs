use webgl_generator::Registry;

fn write_constants<W>(_registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    // FIXME use a const function to determine unused error enum values by examining
    // the ones that are defined here: https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getError
    writeln!(dest, "pub const STACK_OVERFLOW: GLenum = 1337;")?;
    writeln!(dest, "pub const STACK_UNDERFLOW: GLenum = 1338;")?;
    writeln!(dest, "pub const FALSE: GLint = 0;")?;
    writeln!(dest, "pub const TRUE: GLint = 1;")?;
    writeln!(dest, "pub const INFO_LOG_LENGTH: GLenum = 0x1;")?;
    writeln!(dest, "pub const SHADER_SOURCE_LENGTH: GLenum = 0x1;")?;
    Ok(())
}

#[derive(Copy, Clone)]
pub struct FunctionAlternative {
    name: &'static str,
    code: &'static str,
}
impl FunctionAlternative {
    pub fn get(name: &str) -> Option<Self> {
        Self::get_all().iter().cloned().find(|alt| alt.name == name)
    }
    pub fn get_all() -> [Self; 2] {
        [
            Self {
                name: "ShaderSource",
                code: "#[allow(non_snake_case)]
                pub unsafe fn ShaderSource(_shader: types::GLuint, _count: types::GLsizei, _string: *const *const types::GLchar, _length: *const types::GLint) {
                    todo!()
                }",
            },
            Self {
                name: "GetShaderInfoLog",
                code: "#[allow(non_snake_case)]
                pub unsafe fn GetShaderInfoLog(_shader: types::GLuint, _bufSize: types::GLsizei, _length: *mut types::GLsizei, _infoLog: *mut types::GLchar) {
                    todo!()
                }",
            },
        ]
    }
}

pub fn write_typdefs<W>(_registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(dest, "pub type GLchar = i8;")?;
    Ok(())
}

fn write_functions<W>(_registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    writeln!(
        dest,
        "
        #[allow(non_snake_case)]
        pub unsafe fn GetShaderiv(_shader: GLuint, _name: GLenum, _buf: *mut GLint) {{
            todo!();
        }}
    "
    )?;
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
