#version 100
#ifdef GL_ES
    precision mediump float;
#endif

#ifdef NEW_SHADER_INTERFACE
    out vec4 f_color;
    // a dirty hack applied to support webGL2
    #define gl_FragColor f_color
#endif

void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
