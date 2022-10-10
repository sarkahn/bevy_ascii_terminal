use std::collections::BTreeMap;

use bevy::{utils::HashMap, prelude::{Changed, Query, Component, Children, Handle, Mesh, ResMut, Mut, Or, IVec2, Vec2, Assets, Res, Plugin, CoreStage, IntoSystemDescriptor, DetectChanges}};
use sark_grids::GridPoint;

use crate::{Edge, Tile, Terminal, Border, TerminalLayout};

use super::{mesh_data::{TileData, VertData, VertMesher, UvMesher}, uv_mapping::UvMapping, TERMINAL_LAYOUT_CHANGE, TERMINAL_UPDATE_TILES, TERMINAL_RENDER};

#[derive(Debug, Default, PartialEq)]
pub struct BorderTile {
    pub pos: IVec2,
    pub tile: Tile,
}

#[derive(Default, Component)]
pub struct BorderMesh {
    tiles: HashMap<IVec2, BorderTile>,
    size: IVec2,
    tile_size: Vec2,
}

impl BorderMesh {
    pub fn clear(&mut self) {
        self.tiles.clear();
    }

    pub fn put_tile(&mut self, xy: impl GridPoint, tile: Tile) {
        let tile = BorderTile { pos: xy.as_ivec2(), tile };
        self.tiles.insert(xy.as_ivec2(), tile);
    }
}

fn update_from_terminal(
    mut q_border: Query<&mut BorderMesh>,
    q_term: Query<(&Terminal, &TerminalLayout, &Children), Changed<TerminalLayout>>,
) {
    for (term, layout, children) in &q_term {
        println!("Update border from terminal");
        for child in children {
            if let Ok(mut border) = q_border.get_mut(*child) { 
                println!("FOUND BORDER?");
                border.bypass_change_detection().clear();
                if !term.has_border() {
                        return;
                }
                println!("Inserting border tiles");

                border.size = term.size().as_ivec2();
                border.tile_size = layout.tile_size;
                let w = border.size.x;
                let h = border.size.y;
                let hw = w / 2;
                let hh = h / 2;

                let tile = get_tile(Edge::TopLeft, term);
                border.put_tile([-1,1], tile);
                let tile = get_tile(Edge::TopRight, term);
                border.put_tile([1,1], tile);
                let tile = get_tile(Edge::BottomLeft, term);
                border.put_tile([-1,-1], tile);
                let tile = get_tile(Edge::BottomRight, term);
                border.put_tile([1,-1], tile);

                // let top = get_tile(Edge::Top, term);
                // let bot  = get_tile(Edge::Bottom, term);
                // for x in 0..term.width() {
                //     border.put_tile([x as i32 + 1, hh], top);
                //     border.put_tile([x as i32 + 1, -hh], bot);
                // }
                // let left = get_tile(Edge::Left, term);
                // let right  = get_tile(Edge::Right, term);
                // for y in 0..term.height() {
                //     border.put_tile([-hw, y as i32 + 1], left);
                //     border.put_tile([hw, y as i32 + 1], right);
                // }
            }
        }
    } 
}

fn get_tile(edge: Edge, term: &Terminal) -> Tile {
    let mut tile = term.clear_tile;
    tile.glyph = term.border().unwrap().edge_glyph(edge);
    tile
}

fn update_tile_data(
    mut q_mesh: Query<(&BorderMesh, &mut TileData, &mut VertData, &Handle<UvMapping>), 
    Changed<BorderMesh>>,
    mappings: Res<Assets<UvMapping>>,
) {
    for (bmesh, mut td, mut vd, mapping) in &mut q_mesh {
        if bmesh.tiles.len() == 0 {
            continue;
        }
        println!("Update border tile data");
        let mapping = mappings.get(mapping).unwrap();
        let mut vmesher = VertMesher::new([0.0,-0.5], bmesh.tile_size, &mut vd);
        let mut tmesher = UvMesher::new(mapping, &mut td);

        println!("border tiles {}", bmesh.tiles.len());
        for (p,t) in bmesh.tiles.iter() {
            let t = t.tile;
            vmesher.tile_verts_at(*p);
            tmesher.tile_uvs(t.glyph, t.fg_color, t.bg_color);
        }

        println!("Vertcount {}, uvcount {}", vd.verts.len() / 4, td.uvs.len() / 4);
    }
}

pub struct BorderMeshPlugin;

impl Plugin for BorderMeshPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_to_stage(
            CoreStage::Last,
            update_from_terminal
                .after(TERMINAL_UPDATE_TILES)
                .before(TERMINAL_RENDER)
        )
        .add_system_to_stage(CoreStage::Last, 
            update_tile_data
                .after(TERMINAL_UPDATE_TILES)
                .before(TERMINAL_RENDER)
        );
    }
}