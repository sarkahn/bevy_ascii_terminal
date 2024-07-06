#import bevy_sprite::{
    mesh2d_functions::{mesh2d_position_local_to_clip, get_world_from_local},
}

struct TerminalMaterial {
    clip_color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: TerminalMaterial;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) bg_color: vec4<f32>,
    @location(3) fg_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) bg_color: vec4<f32>,
    @location(3) fg_color: vec4<f32>,
};

@vertex
fn vertex(v_in: Vertex) -> VertexOutput {
    var v_out: VertexOutput;
    var model = get_world_from_local(v_in.instance_index);
    v_out.clip_position = mesh2d_position_local_to_clip(model, vec4<f32>(v_in.position, 1.0));
    v_out.uv = v_in.uv;
    v_out.fg_color = v_in.fg_color;
    v_out.bg_color = v_in.bg_color;
    return v_out;
}

struct FragmentInput {
    @location(1) uv: vec2<f32>,
    @location(2) bg_color: vec4<f32>,
    @location(3) fg_color: vec4<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {   
    var clip_col: vec4<f32> = material.clip_color;
    var fg_col = in.fg_color;
    var bg_col = in.bg_color;
    var tex_col = textureSample(texture, texture_sampler, in.uv);
    
    if( all(tex_col.rgb - clip_col.rgb < vec3<f32>(0.001, 0.001, 0.001)) ) {
        return bg_col;
    } else {
        return vec4<f32>(tex_col.rgb * fg_col.rgb, fg_col.a);
    }
}
