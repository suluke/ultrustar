# Runtime code for generated bindings
Hand-written code to be included in bindings.

Binding generation requires manual intervention, i.e. hand-written code, in the following cases:
1. API: Interacting with the bindings themselves
    * `prelude.rs`
2. Util: Support code to keep both generator and generated code simple
    * `prelude.rs`
3. Compat: Code smoothing over the differences between WebGL and GL ES
    * `constants.rs` - GL ES constants not present in WebGL
    * `polyfills.rs` - GL ES functions not present in WebGL
    * `patches.rs` - Functions where parameters/return type differs between GL ES and WebGL
