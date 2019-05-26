#version 330 core

layout(location = 0) in vec4 Position;
layout(location = 1) in vec3 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main() {
    gl_Position = Position;
    OUT.Color = Color;
}