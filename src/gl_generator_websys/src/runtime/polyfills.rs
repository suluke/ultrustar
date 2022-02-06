#[allow(non_snake_case)]
pub unsafe fn GetShaderiv(shader: types::GLuint, param: types::GLenum, buf: *mut types::GLint) {
    withctx!(CONTEXT, ctx, {
        let shader: web_sys::WebGlShader = std::mem::transmute(shader);
        match param {
            DELETE_STATUS | COMPILE_STATUS => {
                *(buf as *mut GLboolean) =
                    ctx.get_shader_parameter(&shader, param).as_bool().unwrap();
            }
            SHADER_TYPE => todo!(),
            _ => panic!("Unknown shader property {}", param),
        }
        // Don't allow wasm_bindgen runtime to forget the shader object
        std::mem::forget(shader);
    });
}

#[allow(non_snake_case)]
pub unsafe fn GetProgramiv(program: types::GLuint, param: types::GLenum, buf: *mut types::GLint) {
    withctx!(CONTEXT, ctx, {
        let program: web_sys::WebGlProgram = std::mem::transmute(program);
        match param {
            DELETE_STATUS | LINK_STATUS | VALIDATE_STATUS => {
                *(buf as *mut GLboolean) =
                    ctx.get_program_parameter(&program, param).as_bool().unwrap();
            }
            ATTACHED_SHADERS | ACTIVE_ATTRIBUTES | ACTIVE_UNIFORMS => todo!(),
            _ => panic!("Unknown shader property {}", param),
        }
        // Don't allow wasm_bindgen runtime to forget the shader object
        std::mem::forget(program);
    });
}

#[allow(non_snake_case)]
pub unsafe fn GenBuffers(n: types::GLsizei, buffers: *mut types::GLuint) {
    withctx!(CONTEXT, ctx, {
        for i in 0..n {
            *buffers.offset(i as isize) = std::mem::transmute(ctx.create_buffer().unwrap());
        }
    });
}
