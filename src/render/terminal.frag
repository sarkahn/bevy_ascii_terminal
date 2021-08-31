#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(location = 1) in vec3 Frag_FG_Color;
layout(location = 2) in vec3 Frag_BG_Color;

# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

void main() {
    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    vec4 texColor = texture(
        sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),
        v_Uv);
    if(texColor.rgb == vec3(0.0)) {
        color.rgb = Frag_BG_Color;
    } else {
        color.rgb *= texColor.rgb * Frag_FG_Color;
    }
# endif
    //color.a = 1.0;
    o_Target = color;
}