#version 100
#ifdef GL_ES
    precision mediump float;
#endif

uniform sampler2D texture;

varying vec2 uv;
varying vec4 color;

void main() {
    //gl_FragColor = vec4(color);
    gl_FragColor = vec4(1.0, 1.0, 1.0, 0.25);
}
