#[allow(unused, non_snake_case)]
#[with_gl_context(CONTEXT as ctx)]
pub unsafe fn GetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    todo!()
}
