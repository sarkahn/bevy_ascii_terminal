use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;


#[derive(Default)]
struct MeshVertData {
    verts: Vec<[f32;3]>,
    normals: Vec<[f32;3]>,
    indices: Vec<u32>,
}

#[derive(Default)]
struct MeshUVData {
    uvs: Vec<[f32;2]>,
}

struct MeshResource {
    mesh: Handle<Mesh>,
}

impl MeshVertData {

    fn add_tile(&mut self, origin: Vec3) {
        let right = Vec3::X;
        let up = Vec3::Y;

        #[rustfmt::skip]
        let verts = vec![
            origin + up, 
            origin + up + right, 
            origin, 
            origin + right
            ];

    
        let normals = vec![[0.0,0.0,1.0]; 4];

        let vi = self.verts.len() as u32;
        let indices = vec![vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1];

        let verts: Vec<[f32;3]> = verts.iter().map(|&p| p.into()).collect();

        self.verts.extend(verts);
        self.indices.extend(indices);
        self.normals.extend(normals);
    }

    fn update_mesh(&self, mesh: &mut Mesh) {
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.verts.clone());
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
    }
}

impl MeshUVData {
    fn add_tile(&mut self, glyph_index: (usize,usize)) {
        let uv_size = Vec2::new(1.0 / 16.0, 1.0 / 16.0);
        let right = Vec2::new(uv_size.x, 0.0);
        let up = Vec2::new(0.0, uv_size.y);
        let origin = Vec2::new(glyph_index.0 as f32 * uv_size.x, glyph_index.1 as f32 * uv_size.y);
    
        #[rustfmt::skip]
        let uvs = vec![
            origin + up, 
            origin + up + right, 
            origin, 
            origin + right
            ];
        let uvs: Vec<[f32;2]> = uvs.iter().map(|&u| u.into()).collect();
        self.uvs.extend(uvs);
    }

    fn update_mesh(&self, mesh: &mut Mesh) {
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
    }
}

fn update_mesh(verts: &MeshVertData, uvs: &MeshUVData, mesh: &mut Mesh) {
    verts.update_mesh(mesh);
    uvs.update_mesh(mesh);
}

fn add_tile_system(
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&mut MeshVertData, &mut MeshUVData, &MeshResource)>) 
{
    if keys.just_pressed(KeyCode::Q) {
        info!("Keypress detected");
        for (mut verts, mut uvs, mesh_res) in q.iter_mut() {
            info!("Updating mesh");
            let mesh = meshes.get_mut(mesh_res.mesh.clone()).unwrap();
            
            verts.add_tile(Vec3::X);
            uvs.add_tile((1,1));

            update_mesh(&verts,&uvs,mesh);
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut verts = MeshVertData::default();
    let mut uvs = MeshUVData::default();

    verts.add_tile(Vec3::ZERO);
    uvs.add_tile((1,1));
    update_mesh(&verts, &uvs, &mut mesh);

    let handle = meshes.add(mesh);

    commands.spawn().insert_bundle((
        verts, 
        uvs,
        MeshResource { mesh: handle.clone()}));


    commands.spawn_bundle(PbrBundle {
        mesh: handle.clone(),
        material: materials.add(Color::BLUE.into()),
        ..Default::default()
    });

    //camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(add_tile_system.system())
        .run();
}