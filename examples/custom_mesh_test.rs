use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

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

fn update_mesh(keys: Res<Input<KeyCode>>,
meshes: Res<Assets<Mesh>>,
mut q: Query<(&mut MeshData, &MeshResource)>) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut data, mesh_res) in q.iter_mut() {

            let origin = Vec3::new(1.0, 1.0, 0.0);
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

            let ii = 6;
            let indices = vec![ii + 0, ii + 1, ii + 2, ii + 3, ii + 2, ii + 1];
            
            
            let mut mesh = meshes.get(mesh_res.mesh.clone()).unwrap();
            
            data.update_mesh(&mut mesh);

        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let origin = Vec3::new(0.0, 0.0, 0.0);
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
    let indices = vec![0, 1, 2, 3, 2, 1];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let handle = meshes.add(mesh);

    let data = MeshData {
        verts: positions,
        uvs: uvs,
        indices: indices,
        normals: normals
    };

    data.update_mesh(&mut mesh);

    let handle = meshes.add(mesh);

    commands.spawn().insert((
        data, 
        MeshResource { mesh: handle}
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
