#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform TerminalMaterial_color {
    vec4 Base_Color;
};
layout(set = 1, binding = 1) uniform TerminalMaterial_clip_color {
    vec4 Clip_Color;
};

layout(set = 1, binding = 2) uniform texture2D TerminalMaterial_texture;
layout(set = 1, binding = 3) uniform sampler TerminalMaterial_texture_sampler;

layout(location = 1) in vec3 Frag_FG_Color;
layout(location = 2) in vec3 Frag_BG_Color;

void main() {
    vec4 color = Base_Color;
    
    vec4 texColor = texture(
        sampler2D(TerminalMaterial_texture, TerminalMaterial_texture_sampler),
        v_Uv);

    if(texColor.rgb == Clip_Color.rgb) {
        color.rgb = Frag_BG_Color;
    } else {
        color.rgb *= texColor.rgb * Frag_FG_Color;
    }

    //color = vec4(1.0);
    o_Target = color;
}