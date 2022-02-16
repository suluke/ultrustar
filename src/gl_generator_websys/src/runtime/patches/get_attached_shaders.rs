#[allow(unused, non_snake_case)]
#[with_gl_context(CONTEXT as ctx)]
pub unsafe fn GetAttachedShaders(
    program: GLuint,
    maxCount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
) {
    todo!()
}
