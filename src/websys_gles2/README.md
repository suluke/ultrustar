# websys_gles2

OpenGL ES2 API on top of [`web_sys`](https://docs.rs/web-sys/latest/web_sys/index.html)' `WebGlRenderingContext`.
This crate basically just calls to `gl_generator_websys` for generating the API and pulls in `web_sys` with the proper activated features.
