#version 450

// Vertex shader input
layout(location = 0) in vec4 frag_color;
layout(location = 1) in vec2 texture_coordinates;
layout(location = 2) flat in int frag_texture_index;

// Render target output
layout(location = 0) out vec4 color;

// Group 0
layout(set = 0, binding = 0) uniform texture2D textures[32];
layout(set = 0, binding = 1) uniform sampler texture_sampler;

void main() {
    color = vec4(texture(sampler2D(textures[frag_texture_index], texture_sampler), texture_coordinates).rgb, 1.0);
}