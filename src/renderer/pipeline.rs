use bevy::{
    core::FloatOrd,
    core_pipeline::Transparent2d,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices,
        render_asset::RenderAssets,
        render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace,
            MultisampleState, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipelineCache,
            RenderPipelineDescriptor, SpecializedPipeline, SpecializedPipelines, TextureFormat,
            VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
        },
        texture::BevyDefault,
        view::VisibleEntities,
        RenderApp, RenderStage,
    },
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
};

pub const TERMINAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 1142086872234562509);

pub struct TerminalPipelinePlugin;

/// Tag component for extract/queue steps.
#[derive(Component,Default)]
pub struct ColoredMesh2d;

impl Plugin for TerminalPipelinePlugin {
    fn build(&self, app: &mut App) {
        let shader_text = include_str!("terminal.wgsl");
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            TERMINAL_SHADER_HANDLE,
            Shader::from_wgsl(shader_text)
        );

        // Register custom draw function and the pipeline,
        // and add our render systems
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app
            .add_render_command::<Transparent2d, DrawTerminal>()
            .init_resource::<TerminalMeshPipeline>()
            .init_resource::<SpecializedPipelines<TerminalMeshPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_terminal_renderer)
            .add_system_to_stage(RenderStage::Queue, queue_terminal_mesh);
    }
}

pub struct TerminalMeshPipeline {
    pipeline: Mesh2dPipeline,
}

impl FromWorld for TerminalMeshPipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            pipeline: Mesh2dPipeline::from_world(world),
        }
    }
}

impl SpecializedPipeline for TerminalMeshPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let vertex_attributes = vec![
            // Position
            VertexAttribute {
                format: VertexFormat::Float32x3,
                // Vertex_position isn't first in the buffer due to how mesh sorts attributes
                // Fixed when https://github.com/bevyengine/bevy/pull/3120 is merged
                offset: 16,
                shader_location: 0,
            },
            // Color
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset:0,
                shader_location: 1
            },
        ];

        // Size of Pos + color (12 + 16)
        let vertex_array_stride = 28;

        RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: TERMINAL_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                // Use our custom vertex buffer
                buffers: vec![VertexBufferLayout {
                    array_stride: vertex_array_stride,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vertex_attributes,
                }],
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: TERMINAL_SHADER_HANDLE.typed::<Shader>(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                }],
            }),
            // Use the two standard uniforms for 2d meshes
            layout: Some(vec![
                // Bind group 0 is the view uniform
                self.pipeline.view_layout.clone(),
                // Bind group 1 is the mesh uniform
                self.pipeline.mesh_layout.clone(),
            ]),
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: key.primitive_topology(),
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("terminal_mesh_pipeline".into()),
        }
    }
}

// This specifies how to render a terminal mesh
type DrawTerminal = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMesh2dViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMesh2dBindGroup<1>,
    // Draw the mesh
    DrawMesh2d,
);

fn extract_terminal_renderer(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    q_computed_vis: Query<(Entity, &ComputedVisibility), With<ColoredMesh2d>>,
) {
    let mut values = Vec::with_capacity(*previous_len);
    for (entity, computed_visibility) in q_computed_vis.iter() {
        if !computed_visibility.is_visible {
            continue;
        }
        values.push((entity, (ColoredMesh2d,)));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

/// Queue our meshes marked with `TerminalRenderer`
#[allow(clippy::too_many_arguments)]
fn queue_terminal_mesh(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    terminal_pipeline: Res<TerminalMeshPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<TerminalMeshPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    q_mesh: Query<(&Mesh2dHandle, &Mesh2dUniform), With<ColoredMesh2d>>,
    mut views: Query<(&VisibleEntities, &mut RenderPhase<Transparent2d>)>,
) {
    if q_mesh.is_empty() {
        return;
    }
    // Iterate each view (a camera is a view)
    for (visible_entities, mut transparent_phase) in views.iter_mut() {
        let draw_terminal = transparent_draw_functions
            .read()
            .get_id::<DrawTerminal>()
            .unwrap();

        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples);

        // Queue all entities visible to that view
        for visible_entity in &visible_entities.entities {
            if let Ok((mesh_handle, terminal_mesh_uniform)) = q_mesh.get(*visible_entity) {
                // Get our specialized pipeline
                let mut mesh2d_key = mesh_key;
                if let Some(mesh) = render_meshes.get(&mesh_handle.0) {
                    mesh2d_key |=
                        Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
                }

                let pipeline_id =
                    pipelines.specialize(&mut pipeline_cache, &terminal_pipeline, mesh2d_key);

                let mesh_z = terminal_mesh_uniform.transform.w_axis.z;
                transparent_phase.add(Transparent2d {
                    entity: *visible_entity,
                    draw_function: draw_terminal,
                    pipeline: pipeline_id,
                    // The 2d render items are sorted according to their z value before rendering,
                    // in order to get correct transparency
                    sort_key: FloatOrd(mesh_z),
                    // This material is not batched
                    batch_range: None,
                });
            }
        }
    }
}