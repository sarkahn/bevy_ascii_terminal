use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;

#[derive(Default)]
struct MeshData {
    verts: Vec<Vec3>,
    uvs: Vec<Vec2>,
    indices: Vec<u32>,
    normals: Vec<Vec3>,
}

struct MeshResource {
    mesh: Handle<Mesh>,
}

impl MeshData {
    fn add_tile(&mut self, origin: Vec3) {
        let right = Vec3::new(1.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

    
        #[rustfmt::skip]
        let positions = vec![
            origin + up, 
            origin + up + right, 
            origin, 
            origin + right
            ];
    
        let origin = Vec2::new(0.0, 0.0);
        let right = Vec2::new(1.0, 0.0);
        let up = Vec2::new(0.0, 1.0);
    
        #[rustfmt::skip]
        let uvs = vec![
            origin + up, 
            origin + up + right, 
            origin, 
            origin + right
            ];
    
        let normals = vec![Vec3::Z; 4];
        let vi = self.verts.len() as u32;
        let indices = vec![vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1];


        self.verts.extend(positions);
        self.uvs.extend(uvs);
        self.indices.extend(indices);
        self.normals.extend(normals);
    }

    fn update_mesh(&self, mesh: &mut Mesh) {
        let positions: Vec<[f32;3]> = self.verts.iter().map(|&p| p.into()).collect();
        let uvs: Vec<[f32;2]> = self.uvs.iter().map(|&u| u.into()).collect();
        let normals: Vec<[f32;3]> = self.normals.iter().map(|&n| n.into()).collect();

        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }
}

fn update_mesh(
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&mut MeshData, &MeshResource)>) 
{
    if keys.just_pressed(KeyCode::Q) {
        info!("Keypress detected");
        for (mut data, mesh_res) in q.iter_mut() {
            info!("Updating mesh");
            let mesh = meshes.get_mut(mesh_res.mesh.clone()).unwrap();
            
            data.add_tile(Vec3::X);
            data.update_mesh(mesh);

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

    let mut data = MeshData::default();

    data.add_tile(Vec3::ZERO);
    data.update_mesh(&mut mesh);
    //data.add_tile(Vec3::X);


    let handle = meshes.add(mesh);

    commands.spawn().insert((
        data, 
        MeshResource { mesh: handle.clone()}
    ));

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: handle.clone(),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
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
        .add_system(update_mesh.system())
        .run();
}