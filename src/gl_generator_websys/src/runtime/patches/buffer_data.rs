#[allow(non_snake_case)]
#[with_gl_context(CONTEXT as ctx)]
pub unsafe fn BufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const core::ffi::c_void,
    usage: GLenum,
) {
    let data = std::slice::from_raw_parts(data as *const u8, size as usize);
    ctx.buffer_data_with_u8_array(target, data, usage);
}
