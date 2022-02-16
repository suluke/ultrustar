#![allow(dead_code, unused_macros, unsafe_code)]

use lazy_static::lazy_static;

pub mod patches;

fn prelude() -> String {
    include_str!("prelude.rs").to_owned()
}

lazy_static! {
    pub static ref PRELUDE: String = prelude();
}
pub const POLYFILLS: &str = include_str!("polyfills.rs");
pub const CONSTANTS: &str = include_str!("constants.rs");

#[cfg(test)]
mod test {
    #![allow(unsafe_code, unused_imports)]
    use gl::{types, *};
    pub type GLboolean = bool;

    include!("prelude.rs");
    include!("constants.rs");
    include!("polyfills.rs");
}
