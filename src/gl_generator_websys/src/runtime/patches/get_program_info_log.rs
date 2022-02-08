#[allow(non_snake_case)]
pub unsafe fn GetProgramInfoLog(
    program: types::GLuint,
    bufSize: types::GLsizei,
    length: *mut types::GLsizei,
    infoLog: *mut types::GLchar) {{
        withctx!(CONTEXT, ctx, {
            let program_ =
                std::mem::MaybeUninit::new(std::mem::transmute::<_, web_sys::WebGlProgram>(program));
            let program = program_.assume_init_ref();
            if let Some(string) = ctx.get_program_info_log(program) {
                let len: usize = (bufSize as usize).min(string.len());
                let written = string[0..len].as_bytes();
                let target = std::slice::from_raw_parts_mut(infoLog as *mut u8, len);
                target.copy_from_slice(written);
                *length = len as types::GLsizei;
            }
        });
}}
