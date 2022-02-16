pub const BUFFER_DATA: &str = include_str!("buffer_data.rs");
pub const BUFFER_SUB_DATA: &str = include_str!("buffer_sub_data.rs");
pub const GET_ATTACHED_SHADERS: &str = include_str!("get_attached_shaders.rs");
pub const GET_EXTENSION: &str = include_str!("get_extension.rs");
pub const GET_FRAMEBUFFER_ATTACHMENT_PARAMETER: &str =
    include_str!("get_framebuffer_attachment_parameter.rs");
pub const GET_PARAMETER: &str = include_str!("get_parameter.rs");
pub const GET_PROGRAM_INFO_LOG: &str = include_str!("get_program_info_log.rs");
pub const GET_SHADER_INFO_LOG: &str = include_str!("get_shader_info_log.rs");
pub const GET_SHADER_SOURCE: &str = include_str!("get_shader_source.rs");
pub const GET_SUPPORTED_EXTENSIONS: &str = include_str!("get_supported_extensions.rs");
pub const GET_VERTEX_ATTRIB: &str = include_str!("get_vertex_attrib.rs");
pub const SHADER_SOURCE: &str = include_str!("shader_source.rs");

#[cfg(test)] // compile test
mod test {
    #![allow(unsafe_code, unused_imports)]
    use gl::types::{self, *};
    include!("../prelude.rs");

    include!("buffer_data.rs");
    include!("buffer_sub_data.rs");
    include!("get_attached_shaders.rs");
    include!("get_extension.rs");
    include!("get_framebuffer_attachment_parameter.rs");
    include!("get_parameter.rs");
    include!("get_program_info_log.rs");
    include!("get_shader_info_log.rs");
    include!("get_shader_source.rs");
    include!("get_supported_extensions.rs");
    include!("get_vertex_attrib.rs");
    include!("shader_source.rs");
}
