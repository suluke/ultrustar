#[allow(unused, non_snake_case)]
#[with_gl_context(CONTEXT as ctx)]
pub unsafe fn GetShaderSource(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
) {
    todo!();
}
