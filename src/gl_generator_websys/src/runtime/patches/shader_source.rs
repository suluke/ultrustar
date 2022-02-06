#[allow(non_snake_case)]
pub unsafe fn ShaderSource(
    shader: GLuint,
    count: types::GLsizei,
    string: *const *const types::GLchar,
    length: *const types::GLint) {
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
}
