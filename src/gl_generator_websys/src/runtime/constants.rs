// FIXME use a const function to determine unused error enum values by examining
// the ones that are defined here: https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getError
pub const STACK_OVERFLOW: types::GLenum = 1337;
pub const STACK_UNDERFLOW: types::GLenum = 1338;
pub const FALSE: GLboolean = false;
pub const TRUE: GLboolean = true;
pub const INFO_LOG_LENGTH: types::GLenum = 0x1;
pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x1;
