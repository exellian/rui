#version 450

// Instance buffer input
layout(location = 0) in vec4 rect;
layout(location = 1) in vec4 color;
layout(location = 2) in int texture_index;

// Fragment shader output
layout(location = 0) out vec4 frag_color;
layout(location = 1) out vec2 texture_coordinates;
layout(location = 2) flat out int frag_texture_index;

void main() {

    frag_color = color;
    frag_texture_index = texture_index;

    if (gl_VertexID == 0) {
        gl_Position = vec4(rect.xy, 0.0, 1.0);
        texture_coordinates = vec2(0.0);
    } else if (gl_VertexID == 1) {
        gl_Position = vec4(rect.x + rect.z, rect.y, 0.0, 1.0);
        texture_coordinates = vec2(0.0);
    } else if (gl_VertexID == 2) {
        gl_Position = vec4(rect.x, rect.y + rect.w, 0.0, 1.0);
    } else {
        gl_Position = vec4(rect.x + rect.z, rect.y + rect.w, 0.0, 1.0);
    }

}