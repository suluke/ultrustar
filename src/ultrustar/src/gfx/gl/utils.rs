use crate::platform::gl::{
    self,
    types::{GLchar, GLenum, GLint, GLuint},
};
use log::error;
use std::mem;
use std::ptr;

/// Check for errors in `gl`
///
/// # Panics
///
/// This function will panic if a `gl` error is encountered
pub fn check_error() {
    #[allow(unsafe_code)]
    unsafe {
        let error = gl::GetError();
        match error {
            gl::NO_ERROR => (),
            gl::INVALID_ENUM => panic!("Invalid enum"),
            gl::INVALID_VALUE => panic!("Invalid value"),
            gl::INVALID_OPERATION => panic!("Invalid operation"),
            gl::STACK_OVERFLOW => panic!("Stack overflow"),
            gl::STACK_UNDERFLOW => panic!("Stack underflow"),
            gl::OUT_OF_MEMORY => panic!("Out of memory"),
            _ => panic!("Unknown error"),
        };
    }
}

pub struct Buffer(GLenum, GLuint);
impl Buffer {
    pub fn new(binding_point: GLenum, size: usize) -> Self {
        #[allow(unsafe_code)]
        unsafe {
            let mut buffer: GLuint = 0;
            gl::GenBuffers(1, &mut buffer as *mut GLuint);
            gl::BindBuffer(binding_point, buffer);
            gl::BufferData(
                binding_point,
                size.try_into().unwrap(),
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(binding_point, 0);
            Self(binding_point, buffer)
        }
    }

    pub fn bind(&self) {
        #[allow(unsafe_code)]
        unsafe {
            gl::BindBuffer(self.0, self.1);
        }
    }

    pub fn _unbind(&self) {
        #[allow(unsafe_code)]
        unsafe {
            gl::BindBuffer(self.0, 0);
        }
    }

    pub fn set_data<T>(&mut self, offset: isize, data: &[T]) {
        self.bind();

        #[allow(unsafe_code, clippy::cast_possible_wrap)]
        unsafe {
            gl::BufferSubData(
                self.0,
                offset as gl::types::GLintptr,
                (data.len() * mem::size_of::<T>())
                    .try_into()
                    .expect("Buffer data exceeds half address space size"),
                data.as_ptr().cast::<std::ffi::c_void>(),
            );
        }
    }
}

pub struct Texture(GLuint);
impl Texture {
    pub fn new_uninitialized() -> Self {
        #[allow(unsafe_code)]
        unsafe {
            let mut texture: GLuint = 0;
            gl::GenTextures(1, &mut texture as *mut GLuint);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            Self(texture)
        }
    }

    pub fn initialize(&mut self, width: usize, height: usize, pixels: &[u8]) {
        #[allow(unsafe_code)]
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr().cast::<std::ffi::c_void>());
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind(&self) {
        #[allow(unsafe_code)]
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
    }
}

pub struct Program(GLuint);
impl Program {
    pub fn new(vert_src: &str, frag_src: &str) -> Self {
        Self(create_program(vert_src, frag_src))
    }
    pub fn get_uniform_location(&self, name: &str) -> Option<GLint> {
        #[allow(unsafe_code)]
        unsafe {
            let name_c = std::ffi::CString::new(name).unwrap();
            let pos = gl::GetUniformLocation(self.0, name_c.as_ptr());
            if pos == -1 {
                None
            } else {
                Some(pos)
            }
        }
    }
    pub fn get_attrib_location(&self, name: &str) -> Option<GLint> {
        #[allow(unsafe_code)]
        unsafe {
            let name_c = std::ffi::CString::new(name).unwrap();
            let pos = gl::GetAttribLocation(self.0, name_c.as_ptr());
            if pos == -1 {
                None
            } else {
                Some(pos)
            }
        }
    }
    pub fn activate(&self) {
        #[allow(unsafe_code)]
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}

fn create_shader(type_: GLenum, src: &str) -> GLuint {
    #[allow(unsafe_code)]
    unsafe {
        let shader = gl::CreateShader(type_);
        let src_ptr = src.as_bytes().as_ptr().cast::<i8>();
        let src_len: GLint = src
            .len()
            .try_into()
            .expect("Shader source size exceeds address space");
        gl::ShaderSource(
            shader,
            1,
            &src_ptr as *const *const i8,
            &src_len as *const GLint,
        );
        gl::CompileShader(shader);
        let mut is_compiled: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled as *mut GLint);
        if is_compiled == GLint::from(gl::FALSE) {
            let mut max_length: GLint = 0;
            #[allow(clippy::cast_sign_loss)]
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut max_length as *mut GLint);

            // The max_length includes the NULL character
            #[allow(clippy::cast_sign_loss)]
            let mut error_log: Vec<GLchar> = vec![0; max_length as usize];
            gl::GetShaderInfoLog(
                shader,
                max_length,
                &mut max_length as *mut GLint,
                error_log.as_mut_ptr(),
            );
            error!("{:?}", std::ffi::CStr::from_ptr(error_log.as_ptr()));

            gl::DeleteShader(shader); // Don't leak the shader.
            panic!("Failed to compile shader");
        }
        shader
    }
}

fn create_program(vert_src: &str, frag_src: &str) -> u32 {
    #[allow(unsafe_code)]
    unsafe {
        let vs = create_shader(gl::VERTEX_SHADER, vert_src);
        let fs = create_shader(gl::FRAGMENT_SHADER, frag_src);
        let program = gl::CreateProgram();

        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        gl::DeleteShader(fs);
        gl::DeleteShader(vs);

        let mut is_linked: GLint = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut is_linked as *mut GLint);
        if is_linked == GLint::from(gl::FALSE) {
            let mut max_length: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut max_length as *mut GLint);

            // The max_length includes the NULL character
            #[allow(clippy::cast_sign_loss)]
            let mut error_log: Vec<GLchar> = vec![0; max_length as usize];
            gl::GetProgramInfoLog(
                program,
                max_length,
                &mut max_length as *mut GLint,
                error_log.as_mut_ptr(),
            );
            error!("{:?}", std::ffi::CStr::from_ptr(error_log.as_ptr()));

            gl::DeleteProgram(program); // Don't leak the program.
            panic!("Failed to link program");
        }

        program
    }
}
