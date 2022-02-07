pub const BUFFER_DATA: &str = include_str!("buffer_data.rs");
pub const BUFFER_SUB_DATA: &str = include_str!("buffer_sub_data.rs");
pub const SHADER_SOURCE: &str = include_str!("shader_source.rs");
pub const GET_SHADER_INFO_LOG: &str = include_str!("get_shader_info_log.rs");
pub const GET_PROGRAM_INFO_LOG: &str = include_str!("get_program_info_log.rs");

#[cfg(test)] // compile test
mod test {
    #![allow(unsafe_code)]
    use gl::types::{self, *};
    include!("../prelude.rs");

    include!("buffer_data.rs");
    include!("buffer_sub_data.rs");
    include!("shader_source.rs");
    include!("get_shader_info_log.rs");
    include!("get_program_info_log.rs");
}
