use bevy::{prelude::{Assets, Added, Commands, Entity, Query, Changed, Handle, Res, Or, GlobalTransform}};
use sark_grids::Size2d;

use crate::{TerminalLayout, Terminal};

use super::{mesh_data::{VertData, VertMesher, TileData, UvMesher}, uv_mapping::UvMapping};


pub(crate) fn init_terminal(
    mut q: Query<Entity, Added<Terminal>>,
    _commands: Commands,
) {
    for _term_entity in q.iter_mut() {
        // let border = commands.spawn((
        //     TerminalRenderBundle::default(),
        //     BorderMesh::default(),
        // )).id();

        // commands.entity(term_entity).push_children(&[border]);
    }
}


pub(crate) fn update_layout(
    mut q_term: Query<(&Terminal, &mut TerminalLayout, &GlobalTransform), Changed<Terminal>>,
) {
    for (term, mut layout, transform) in &mut q_term {
        if layout.term_size() != term.size() 
        || layout.border.as_ref() != term.border()
        {
            // println!("Updating layout");
            let pos = transform.translation().truncate().as_ivec2();
            layout.update_state(term, pos);
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_vert_data(
    mut q_term: Query<
        (&TerminalLayout, &mut VertData, &Handle<UvMapping>),
        Changed<TerminalLayout>,
    >,
    mappings: Res<Assets<UvMapping>>,
) {
    for (layout, mut verts, mapping) in &mut q_term {
        if mappings.get(mapping).is_none() {
            continue;
        }

        verts.clear();
        verts.reserve(layout.term_size().len());
        
        let mut mesher = VertMesher::new(
            layout.origin(), 
            layout.tile_size, 
            &mut verts
        );
        
        // Note the order verts are added - uvs must be added in the same order!
        for i in 0..layout.term_size().len() {
            let x = i % layout.width();
            let y = i / layout.width();
            mesher.tile_verts_at([x,y]);
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_tile_data(
    mut q_term: Query<
        (&Terminal, &mut TileData, &Handle<UvMapping>),
        Or<(Changed<Terminal>, Changed<TerminalLayout>)>,
    >,
    mappings: Res<Assets<UvMapping>>,
) {
    for (term, mut tiles, mapping) in &mut q_term {
        if mappings.get(mapping).is_none() {
            continue;
        }
        tiles.clear();
        tiles.reserve(term.size().len());
        let mapping = mappings.get(mapping).unwrap();
        let mut mesher = UvMesher::new(mapping, &mut tiles);

        //println!("Updating tile data");
        for tile in term.iter() {
            mesher.tile_uvs(tile.glyph, tile.fg_color, tile.bg_color);
        }
    }
}