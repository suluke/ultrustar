#[cfg(not(feature = "no-unsafe"))]
pub(crate) const TOKEN_UNSAFE: &str = "unsafe";
#[cfg(feature = "no-unsafe")]
pub(crate) const TOKEN_UNSAFE: &str = "unsafe"; // TODO finalize no-unsafe feature
