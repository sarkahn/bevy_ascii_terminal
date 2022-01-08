//! Default plugin for rendering the terminal to a bevy mesh.

pub const TERMINAL_INIT: &str = "terminal_init_mesh";
pub const TERMINAL_UPDATE_MATERIAL: &str = "terminal_update_material";
pub const TERMINAL_UPDATE_SIZE: &str = "terminal_update_size";
pub const TERMINAL_UPDATE_TILE_DATA: &str = "terminal_update_tile_data";
pub const TERMINAL_UPDATE_MESH: &str = "terminal_update_mesh";

use bevy::{
    prelude::*,
    render::{render_resource::PrimitiveTopology, mesh::Indices
    }, sprite::Mesh2dHandle, reflect::TypeUuid,
};

use super::{font::*, *, material::TerminalMaterialPlugin,};

pub struct TerminalRendererPlugin;



impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugin(TerminalFontPlugin);

        app.add_plugin(TerminalMaterialPlugin);

        // app .add_system_set(
        //         SystemSet::on_enter(TerminalAssetLoadState::AssetsDoneLoading)
        //             .with_system(terminal_renderer_init.system()),
        //     )
        //     .add_system_set(
        //         SystemSet::on_update(TerminalAssetLoadState::AssetsDoneLoading)
        //             // .with_system(
        //             //     terminal_renderer_update_material
        //             //         .label(TERMINAL_UPDATE_MATERIAL),
        //             // )
        //             .with_system(
        //                 terminal_renderer_update_size
        //                     //.after(TERMINAL_UPDATE_MATERIAL)
        //                     .label(TERMINAL_UPDATE_SIZE),
        //             )
        //             .with_system(
        //                 terminal_renderer_update_tile_data
        //                     .after(TERMINAL_UPDATE_SIZE)
        //                     .label(TERMINAL_UPDATE_TILE_DATA),
        //             )
        //             .with_system(
        //                 terminal_renderer_update_mesh
        //                     .after(TERMINAL_UPDATE_TILE_DATA)
        //                     .label(TERMINAL_UPDATE_MESH),
        //             ),
        //     );

        app.add_system(terminal_renderer_init
                .label(TERMINAL_INIT))
            .add_system(terminal_renderer_update_size
                .after(TERMINAL_INIT)
                .label(TERMINAL_UPDATE_SIZE))
            .add_system(terminal_renderer_update_tile_data
                .after(TERMINAL_UPDATE_SIZE)
                .label(TERMINAL_UPDATE_TILE_DATA))
            .add_system(terminal_renderer_update_mesh
                .after(TERMINAL_UPDATE_TILE_DATA)
                .label(TERMINAL_UPDATE_MESH)
            );
            


        // // Set up material/pipline for default terminal construction
        // let cell = app.world_mut().cell();

        // materials.set_untracked(
        //     Handle::<TerminalMaterial>::default(),
        //     TerminalMaterial::default(),
        // );
    }
}

#[allow(clippy::type_complexity)]
pub fn terminal_renderer_init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<&mut Mesh2dHandle, (Added<Mesh2dHandle>, With<TerminalRendererVertexData>)>,
) {
    for mut mesh in q.iter_mut() {
        //info!("Initializing ascii terminal mesh");
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = Mesh2dHandle(meshes.add(new_mesh));
    }
}

// fn terminal_renderer_update_material(
//     fonts: Res<TerminalFonts>,
//     mut materials: ResMut<Assets<TerminalMaterial>>,
//     mut q: Query<
//             //(&TerminalFont, 
//             &mut Handle<TerminalMaterial>
//             //)
//             , Changed<Handle<TerminalMaterial>>>,
// ) {
//     for mut mat in q.iter_mut() {
//         //info!("Updating terminal renderer material");
//         let existing_mat = materials.get(mat.clone_weak());

//         if existing_mat.is_some() {
//             materials.remove(mat.clone_weak());
//         }

//         let handle = fonts.get(font.name()).texture_handle();

//         *mat = materials.add(
//             TerminalMaterial { 
//                 clip_color: font.clip_color(), 
//                 texture: Some(handle.clone()), 
//             }
//         );
//     }
// }

#[allow(clippy::type_complexity)]
fn terminal_renderer_update_size(
    mut meshes: ResMut<Assets<Mesh>>,
    //fonts: Res<TerminalFonts>,
    materials: Res<Assets<TerminalMaterial>>,
    mut q: Query<
        (
            &Terminal,
            //&TerminalFont,
            &Handle<TerminalMaterial>,
            &TileScaling,
            &TerminalPivot,
            &TilePivot,
            &mut Mesh2dHandle,
            &mut TerminalRendererVertexData,
            &mut TerminalRendererTileData,
        ),
        Or<(
            //Changed<Terminal>,
            Changed<Handle<Mesh>>,
            Changed<TileScaling>,
            Changed<Handle<TerminalMaterial>>,
            //Changed<TerminalFont>,
        )>,
    >,
) {
    for (terminal, material, scaling, term_pivot, tile_pivot, mesh, mut vert_data, mut tile_data) in
        q.iter_mut()
    {
        let mut tile_size = UVec2::ONE;
        // if let TileScaling::Pixels = scaling {
        //     tile_size *= fonts.get(font.name()).pixels_per_unit();
        // }

        let size = terminal.size();
        vert_data.resize(size, term_pivot.0, tile_pivot.0, tile_size);
        tile_data.resize(size);

        let mesh = meshes
            .get_mut(mesh.0.clone())
            .expect("Error retrieving mesh from terminal renderer");

         info!("Changing mesh size size: {}, Length: {}", size, vert_data.indices.len());
        // info!("First 4 verts: {:?}", &vert_data.verts[0..4]);
        // info!("First 6 indices: {:?}", &vert_data.indices[0..6]);
        mesh.set_indices(Some(Indices::U32(vert_data.indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vert_data.verts.clone());
    }
}

pub fn terminal_renderer_update_tile_data(
    mut q: Query<(&Terminal, &mut TerminalRendererTileData), Changed<Terminal>>,
) {
    for (term, mut data) in q.iter_mut() {
        info!("Renderer update tile data (colors)!");
        //info!("First tiles: {:?}", &term.tiles[0..4]);
        data.update_from_tiles(&term.tiles.slice(..));
    }
}

pub fn terminal_renderer_update_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&TerminalRendererTileData, &Mesh2dHandle), Changed<TerminalRendererTileData>>,
) {
    for (tile_data, mesh) in q.iter_mut() {
        let mesh = meshes.get_mut(&mesh.0).expect("Error accessing terminal mesh");
        info!("writing colors and uvs to mesh");
        //info!("First fg Colors: {:?}", &tile_data.fg_colors[0..4]);
        //info!("First bg Colors: {:?}", &tile_data.bg_colors[0..4]);
        //info!("First uvs: {:?}", &tile_data.uvs[0..4]);

        //mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, tile_data.fg_colors.clone());
        
        mesh.set_attribute("bg_color", tile_data.bg_colors.clone());
        mesh.set_attribute("fg_color", tile_data.fg_colors.clone());
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, tile_data.uvs.clone());
    }
}

// #[cfg(test)]
// pub mod tests {
//     use bevy::prelude::*;

//     #[test]
//     fn mesh_test() {
//         let _world = World::default();

//         let _update_stage = SystemStage::parallel();
//     }
// }
