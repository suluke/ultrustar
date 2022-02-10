#[allow(non_snake_case)]
#[with_gl_context(CONTEXT as ctx)]
pub unsafe fn BufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const std::ffi::c_void,
) {
    let data = std::slice::from_raw_parts(data as *const u8, size as usize);
    let offset = offset as i32;
    ctx.buffer_sub_data_with_i32_and_u8_array(target, offset, data)
}
