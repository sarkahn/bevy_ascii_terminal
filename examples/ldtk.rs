use bevy::prelude::*;

use bevy_ascii_terminal::{Terminal, ldtk::{LdtkPlugin, LdtkAsset}, TerminalBundle, TerminalPlugin, code_page_437, TerminalMaterial, renderer::uv_mapping::UvMapping, TileWriter};
use bevy_tiled_camera::{TiledCameraBundle, TiledCameraPlugin, TiledProjection};

fn main () {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(LdtkPlugin)
    .add_plugin(TerminalPlugin)
    .add_plugin(TiledCameraPlugin)
    .add_startup_system(setup)
    .add_system(build_from_ldtk)
    .run();
}

#[derive(Component)]
pub struct BuildLdtk(Handle<LdtkAsset>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let handle: Handle<LdtkAsset> = asset_server.load("tiles.ldtk");

    let size = [32,32];
    commands.spawn_bundle(TerminalBundle::new().with_size(size))
    .insert(handle)
    ;
    commands.spawn_bundle(TiledCameraBundle::new().with_tile_count(size));

}

pub struct BuildFromLdtk;

fn build_from_ldtk(
    mut ev_ldtk: EventReader<AssetEvent<LdtkAsset>>,
    mut q_term: Query<(&mut Terminal, &Handle<TerminalMaterial>, &mut UvMapping, &Handle<LdtkAsset>)>,
    maps: Res<Assets<LdtkAsset>>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    mut q_cam: Query<&mut TiledProjection>,
) {
    for ev in ev_ldtk.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                println!("Asset created");
                for (mut term, mat, mut mapping, old_handle) in q_term.iter_mut() {
                    if old_handle != handle {
                        continue;
                    }
                    if let Some(new_map) = maps.get(handle) {
                        if !new_map.tilesets.is_empty() {
                            let mat = materials.get_mut(mat).unwrap();
                            let tex_handle = new_map.tilesets.iter().next().unwrap().1.clone();
                            mat.texture = Some(tex_handle);

                            let tileset = &new_map.project.defs.tilesets[0];
                            let w = tileset.c_wid as u32;
                            let h = tileset.c_hei as u32;

                            *mapping = UvMapping::from_grid([w,h]);
                            //*mat.texture = new_map.tilesets[0].1.clone();
                        }
                        let p = &new_map.project;
                        for level in p.levels.iter() {
                            if let Some(layers) = &level.layer_instances {
                                let w = layers.iter().map(|l|l.c_wid).max().unwrap() as u32;
                                let h = layers.iter().map(|l|l.c_hei).max().unwrap() as u32;
                                term.resize([w,h]);
                                let mut proj = q_cam.single_mut();
                                proj.set_tile_count([w,h]);
                                println!("Resizing to {},{}", w,h);
                                term.fill(0.transparent());
                                for layer in layers.iter().rev() {
                                    let height_offset = layer.c_hei as i32 - 1;
                                    for tile in layer.grid_tiles.iter() {
                                        let xy = IVec2::new(tile.px[0] as i32, tile.px[1] as i32);
                                        let xy = xy / layer.grid_size as i32;
                                        let xy = IVec2::new(xy.x, height_offset - xy.y);

                                        let id = tile.t as u16;
                                        term.put_tile(xy, id.fg(Color::WHITE));
                                    }
                                    for tile in layer.auto_layer_tiles.iter() {
                                        let xy = IVec2::new(tile.px[0] as i32, tile.px[1] as i32);
                                        let xy = xy / layer.grid_size as i32;
                                        let xy = IVec2::new(xy.x, height_offset - xy.y);
                                        
                                        let id = tile.t as u16;
                                        term.put_tile(xy, id.fg(Color::WHITE));
                                    }
                                }
                            }
                        }
                    }
                } 
            },
            _ => {}
        }
    }
}
