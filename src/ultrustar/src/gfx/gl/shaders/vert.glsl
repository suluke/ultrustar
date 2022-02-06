#version 100
#ifdef GL_ES
    precision mediump float;
#endif

uniform vec4 transform;

attribute vec2 v_position;
attribute vec2 v_uv;
attribute vec4 v_color;

varying vec2 uv;
varying vec4 color;

void main() {
    gl_Position = vec4(v_position * transform.xy + transform.zw, 0.0, 1.0);
    uv = v_uv;
    color = v_color;
}
