use webgl_generator::Registry;

fn write_constants<W>(_registry: &Registry, dest: &mut W) -> std::io::Result<()>
where
    W: std::io::Write,
{
    // FIXME use a const function to determine unused error enum values by examining
    // the ones that are defined here: https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getError
    writeln!(dest, "pub const STACK_OVERFLOW: GLenum = 1337;")?;
    writeln!(dest, "pub const STACK_UNDERFLOW: GLenum = 1338;")?;
    writeln!(dest, "pub const FALSE: GLboolean = false;")?;
    writeln!(dest, "pub const TRUE: GLboolean = true;")?;
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
    pub fn get_all() -> [Self; 5] {
        [
            Self {
                name: "BufferData",
                code: "#[allow(non_snake_case)]
                pub unsafe fn BufferData(
                    target: GLenum,
                    size: GLsizeiptr,
                    data: *const core::ffi::c_void,
                    usage: GLenum) {
                    withctx!(CONTEXT, ctx, {
                        let data = std::slice::from_raw_parts(data as *const u8, *(size as *const usize));
                        ctx.buffer_data_with_u8_array(target, data, usage);
                    })
                }",
            },
            Self {
                name: "BufferSubData",
                code: "#[allow(non_snake_case)]
                pub unsafe fn BufferSubData(
                    target: GLenum,
                    offset: GLintptr,
                    size: GLsizeiptr,
                    data: *const std::ffi::c_void) {
                    let data = std::slice::from_raw_parts(data as *const u8, *(size as *const usize));
                    let offset = offset as i32;
                    withctx!(CONTEXT, ctx, {ctx.buffer_sub_data_with_i32_and_u8_array(target, offset, data)})
                }",
            },
            Self {
                name: "ShaderSource",
                code: "#[allow(non_snake_case)]
                pub unsafe fn ShaderSource(shader: WebGLShader, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint) {
                    if count == 1 {
                        let len: types::GLint = *length;
                        let src: &str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                            *(string as *const *const u8),
                            len as usize,
                        ));
                        let shdr: web_sys::WebGlShader = std::mem::transmute::<_, _>(shader);
                        withctx!(CONTEXT, ctx, {
                            ctx.shader_source(&shdr, src)
                        });
                        // Don't allow the wasm_bindgen glue to forget the shader object
                        std::mem::forget(shdr);
                    } else {
                        todo!();
                    }
                }",
            },
            Self {
                name: "GetShaderInfoLog",
                code: "#[allow(non_snake_case)]
                pub unsafe fn GetShaderInfoLog(_shader: types::GLuint, _bufSize: types::GLsizei, _length: *mut types::GLsizei, _infoLog: *mut types::GLchar) {
                    todo!()
                }",
            },
            Self {
                name: "GetProgramInfoLog",
                code: "#[allow(non_snake_case)]
                pub unsafe fn GetProgramInfoLog(_program: types::GLuint, _bufSize: types::GLsizei, _length: *mut types::GLsizei, _infoLog: *mut types::GLchar) {
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
        pub unsafe fn GetShaderiv(shader: WebGLShader, param: GLenum, buf: *mut GLint) {{
            withctx!(CONTEXT, ctx, {{
                let shader: web_sys::WebGlShader = std::mem::transmute(shader);
                match param {{
                    DELETE_STATUS | COMPILE_STATUS => {{
                        *(buf as *mut GLboolean) =
                            ctx.get_shader_parameter(&shader, param).as_bool().unwrap();
                    }}
                    SHADER_TYPE => todo!(),
                    _ => panic!(\"Unknown shader property {{}}\", param),
                }}
                // Don't allow wasm_bindgen runtime to forget the shader object
                std::mem::forget(shader);
            }});
        }}

        #[allow(non_snake_case)]
        pub unsafe fn GetProgramiv(program: WebGLProgram, param: GLenum, buf: *mut GLint) {{
            withctx!(CONTEXT, ctx, {{
                let program: web_sys::WebGlProgram = std::mem::transmute(program);
                match param {{
                    DELETE_STATUS | LINK_STATUS | VALIDATE_STATUS => {{
                        *(buf as *mut GLboolean) =
                            ctx.get_program_parameter(&program, param).as_bool().unwrap();
                    }}
                    ATTACHED_SHADERS | ACTIVE_ATTRIBUTES | ACTIVE_UNIFORMS => todo!(),
                    _ => panic!(\"Unknown shader property {{}}\", param),
                }}
                // Don't allow wasm_bindgen runtime to forget the shader object
                std::mem::forget(program);
            }});
        }}

        #[allow(non_snake_case)]
        pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) {{
            withctx!(CONTEXT, ctx, {{
                for i in 0..n {{
                    *buffers.offset(i as isize) = std::mem::transmute(ctx.create_buffer().unwrap());
                }}
            }});
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
