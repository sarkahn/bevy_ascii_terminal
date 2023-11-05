#import bevy_render::view::View;
#import bevy_sprite::{mesh2d_functions as mesh_functions, mesh2d_types::Mesh2d};

struct TerminalMaterial {
    clip_color: vec4<f32>,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
};
const TERMINAL_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

@group(0) @binding(0)
var<uniform> view: View;

@group(1) @binding(0)
var<uniform> material: TerminalMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) bg_color: vec4<f32>,
    @location(3) fg_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) bg_color: vec4<f32>,
    @location(3) fg_color: vec4<f32>,
};

/// Entry point for the vertex shader
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var model = mesh_functions::get_model_matrix(vertex.instance_index);
    var world_position = mesh_functions::mesh2d_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.world_position = world_position;
    // Project the world position of the mesh into screen position
    out.clip_position = mesh_functions::mesh2d_position_world_to_clip(world_position);
    out.uv = vertex.uv;
    out.fg_color = vertex.fg_color;
    out.bg_color = vertex.bg_color;
    return out;
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @location(0) world_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) bg_color: vec4<f32>,
    @location(3) fg_color: vec4<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    
    var clip_color: vec4<f32> = material.clip_color;
    var fg_color = in.fg_color;
    var bg_color = in.bg_color;

    var out_color = fg_color;

    if ((material.flags & TERMINAL_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
        var tex_color = textureSample(texture, texture_sampler, in.uv);

        let tex_rgb = vec3<f32>(tex_color.rgb);
        let clip_rgb = vec3<f32>(clip_color.rgb);
        
        if( all(tex_rgb - clip_rgb < vec3<f32>(0.001, 0.001, 0.001)) ) {
            out_color = bg_color;
        } else {
            out_color = vec4<f32>(tex_color.rgb * fg_color.rgb, fg_color.a);
        }
    }
    //return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    return out_color;
}
