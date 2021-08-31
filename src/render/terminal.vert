#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 2) in vec3 FG_Color;
layout(location = 3) in vec3 BG_Color;

layout(location = 0) out vec2 v_Uv;
layout(location = 1) out vec3 Frag_FG_Color;
layout(location = 2) out vec3 Frag_BG_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    vec2 uv = Vertex_Uv;

    v_Uv = uv;

    vec3 position = Vertex_Position;
    gl_Position = ViewProj * Model * vec4(position, 1.0);
    Frag_FG_Color = FG_Color;
    Frag_BG_Color = BG_Color;
}