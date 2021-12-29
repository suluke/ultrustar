/// Re-export webgl_generator apis so that users don't need to explicitly name it
/// as a dependency. This avoids version mismatches.
pub use webgl_generator::*;

pub struct WebSysGen;
impl Generator for WebSysGen {
    fn write<W>(&self, _registry: &Registry, _dest: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        // todo!()
        Ok(())
    }
}
