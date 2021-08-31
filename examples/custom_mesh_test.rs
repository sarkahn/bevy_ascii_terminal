use bevy::prelude::*;
use bevy::render::{mesh::Indices};
use bevy::render::pipeline::{PrimitiveTopology};

use bevy_terminal::render::pipeline::{TerminalRendererPipeline};

#[derive(Default)]
struct MeshVertData {
    verts: Vec<[f32;3]>,
    indices: Vec<u32>,
}

#[derive(Default)]
struct MeshUVData {
    uvs: Vec<[f32;2]>,
    fg_colors: Vec<[f32;3]>,
    bg_colors: Vec<[f32;3]>,
}

impl MeshVertData {

    fn add_tile(&mut self, origin: Vec3) {
        let right = Vec3::X;
        let up = Vec3::Y;

        #[rustfmt::skip]
        let verts = vec![
            origin + up, 
            origin, 
            origin + up + right, 
            origin + right,
            ];

        let vi = self.verts.len() as u32;
        let indices = vec![vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1];

        let verts: Vec<[f32;3]> = verts.iter().map(|&p| p.into()).collect();

        self.verts.extend(verts);
        self.indices.extend(indices);
    }

    fn update_mesh(&self, mesh: &mut Mesh) {
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.verts.clone());
    }
}

impl MeshUVData {
    fn add_tile(&mut self, glyph_index: (usize,usize), fg_color: Color, bg_color: Color) {
        let uv_size = Vec2::new(1.0 / 16.0, 1.0 / 16.0);
        let right = Vec2::new(uv_size.x, 0.0);
        let up = Vec2::new(0.0, uv_size.y);
        let origin = Vec2::new(glyph_index.0 as f32 * uv_size.x, glyph_index.1 as f32 * uv_size.y);
    
        #[rustfmt::skip]
        let uvs = vec![
            origin, 
            origin + up, 
            origin + right,
            origin + up + right, 
            ];

        let fg_colors: Vec<[f32;3]> = vec![Vec4::from(fg_color).truncate().into(); 4];
        let bg_colors: Vec<[f32;3]> = vec![Vec4::from(bg_color).truncate().into(); 4];

        let uvs: Vec<[f32;2]> = uvs.into_iter().map(|u| u.into()).collect();
        self.uvs.extend(uvs);
        self.fg_colors.extend(fg_colors);
        self.bg_colors.extend(bg_colors);
    }

    fn change_tile(&mut self, tile_index: usize, glyph_index: (usize, usize)) {

    }

    fn update_mesh(&self, mesh: &mut Mesh) {
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        mesh.set_attribute("FG_Color", self.fg_colors.clone());
        mesh.set_attribute("BG_Color", self.bg_colors.clone());
    }
}

fn update_mesh(verts: &MeshVertData, uvs: &MeshUVData, mesh: &mut Mesh) {
    verts.update_mesh(mesh);
    uvs.update_mesh(mesh);
}

fn add_tile_system(
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&mut MeshVertData, &mut MeshUVData, &Handle<Mesh>)>) 
{
    if keys.just_pressed(KeyCode::Q) {
        info!("Keypress detected");
        for (mut verts, mut uvs, mesh) in q.iter_mut() {
            info!("Updating mesh");
            let mesh = meshes.get_mut(mesh.clone()).unwrap();
            
            verts.add_tile(Vec3::X);
            uvs.add_tile((1,1), Color::PINK, Color::RED);

            update_mesh(&verts,&uvs,mesh);
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pipeline: Res<TerminalRendererPipeline>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tex = asset_server.load("zx_evolution_8x8.png");
    let mat = materials.add(ColorMaterial::texture(tex));

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut verts = MeshVertData::default();
    let mut uvs = MeshUVData::default();

    verts.add_tile(Vec3::ZERO);
    uvs.add_tile((1,0), Color::YELLOW, Color::GREEN);

    update_mesh(&verts, &uvs, &mut mesh);

    let pipeline = pipeline.get_pipelines();
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(mesh),
        render_pipelines: pipeline,
        ..Default::default()
    }).insert(mat).insert(verts).insert(uvs);

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .init_resource::<TerminalRendererPipeline>()
        .add_system(add_tile_system.system())
        .add_startup_system(setup.system())
        .run();
}