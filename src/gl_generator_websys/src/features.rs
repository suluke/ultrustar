#[cfg(not(feature = "no-unsafe"))]
pub(crate) const TOKEN_UNSAFE: &'static str = "unsafe";
#[cfg(feature = "no-unsafe")]
pub(crate) const TOKEN_UNSAFE: &'static str = "unsafe"; // TODO finalize no-unsafe feature
