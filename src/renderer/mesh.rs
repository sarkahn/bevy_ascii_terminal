use bevy::{
    ecs::prelude::*,
    math::vec3,
    prelude::{Assets, BuildChildren, Children, Handle, Image, Mesh, UVec2, Vec2, Vec3, Visibility},
    render::{
        mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
        render_resource::{PrimitiveTopology, VertexFormat}, view::VisibleEntities,
    },
    sprite::{Mesh2dHandle, MaterialMesh2dBundle},
};
use sark_grids::Size2d;

use crate::{Terminal, TerminalMaterial, Tile};

use super::{
    border::TerminalBorder, mesh_data::{VertexData, TileData, MeshData, ATTRIBUTE_COLOR_BG, ATTRIBUTE_COLOR_FG, ATTRIBUTE_UV}, TerminalLayout, TileScaling, AsciiRenderBundle, uv_mapping::UvMapping,
};

#[allow(clippy::type_complexity)]
pub(crate) fn init_terminal(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(Entity, &mut Mesh2dHandle, &mut TerminalLayout), Added<Terminal>>,
    mut commands: Commands,
) {
    for (term_entity, mut mesh, mut layout) in q.iter_mut() {
        //info!("Initializing terminal mesh");
        // Initialize terminal mesh
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = Mesh2dHandle(meshes.add(new_mesh));

        // Initialize border entity and mesh
        let border_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let border_bundle = AsciiRenderBundle {
            mesh_bundle: MaterialMesh2dBundle { 
                mesh: Mesh2dHandle(meshes.add(border_mesh)), 
                visibility: Visibility::INVISIBLE,
                ..Default::default()
            },
            ..Default::default()
        };

        let border_entity = commands.spawn_bundle(
            border_bundle
        ).id();

        layout.border_entity = Some(border_entity);

        commands.entity(term_entity).add_child(border_entity);
    }
}

pub(crate) fn material_change(
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    mut q_term: Query<
        (&Handle<TerminalMaterial>, &mut TerminalLayout),
        Changed<Handle<TerminalMaterial>>,
    >,
) {
    for (handle, mut layout) in &mut q_term {
        if let Some(material) = materials.get(handle) {
            if let Some(image) = material.texture.clone() {
                if let Some(image) = images.get(&image) {
                    // TODO: Should be derived from image size, can't assume 16x16 tilesheet for
                    // graphical terminals
                    let font_size = image.size() / 16.0;
                    layout.pixels_per_tile = font_size.as_uvec2();
                }
            }
        }
    }
}

pub(crate) fn update_layout(mut q_term: Query<(&Terminal, &mut TerminalLayout), Changed<Terminal>>) {
    for (term, mut layout) in &mut q_term {
        if layout.term_size() != term.size() || layout.has_border() != term.has_border() {
            layout.update_state(term);
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn layout_changed(
    mut q_term: Query<(
        &Terminal, 
        &TerminalLayout, 
        &mut VertexData, 
        &mut TileData, 
        &UvMapping, 
        &Mesh2dHandle
    ), 
        (Changed<TerminalLayout>, Without<TerminalBorder>)
        >,
    mut q_border: Query<(&mut VertexData, &mut TileData, &mut Visibility, &Mesh2dHandle), 
        With<TerminalBorder>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (term, layout, mut vert_data, mut tile_data, mapping, mesh) in &mut q_term {
        let tile_count = term.size().len();
        let width = term.width();
        let height = term.height();

        let mesh = meshes.get_mut(&mesh.0)
            .expect("Error retrieving terminal mesh");

        // Remove vert data so we can modify it with indices without
        // the borrow checker complaining. Note indices are mutably
        // borrowed, not removed since they are separate from the attribute map.
        let (mut verts, indices) = mesh.get_vert_data();
        verts.clear();
        indices.clear();
        // 4 verts per tile
        verts.reserve(tile_count * 4);
        // 6 indices per tile
        indices.reserve(tile_count * 6);

        let tile_size = layout.tile_size;
        let origin = layout.origin();
        let right = Vec3::X * tile_size.x;
        let up = Vec3::Y * tile_size.y;

        for i in 0..tile_count {
            let x = (i % width) as f32;
            let y = (i / width) as f32;

            let xy = vec3(x, y, 0.0) * tile_size.extend(0.0);
            let p = origin.extend(0.0) + xy;

            verts
                .extend(&[p + up, p, p + right + up, p + right].map(|p| p.to_array()));
            let vi = (i * 4) as u32;
            indices
                .extend(&[vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);
        }

        mesh.insert_vert_data(verts);

        // Update border only when layout changes 
        let (mut vert_data, mut tile_data, mut visibility, mesh) = 
            q_border.get_mut(layout.border_entity.unwrap()).unwrap();

        let mesh = meshes.get_mut(&mesh.0)
            .expect("Error retrieving the terminal border mesh");

        let (mut uvs, mut fg_colors, mut bg_colors) = mesh.get_tile_data();
        let (mut verts, indices) = mesh.get_vert_data();

        verts.clear();
        indices.clear();
        uvs.clear();
        fg_colors.clear();
        bg_colors.clear();

        if let Some(border) = &term.border {
            visibility.is_visible = true;
            let tile_count = ((width + 2) * 2) + (height * 2);

            verts.reserve(tile_count * 4);
            indices.reserve(tile_count * 6);
            uvs.reserve(tile_count * 4);
            fg_colors.reserve(tile_count * 4);
            bg_colors.reserve(tile_count * 4);
            
            let origin = origin.extend(0.0) - tile_size.extend(0.0);
            let right = Vec3::X * tile_size.x;
            let up = Vec3::Y * tile_size.y;

            let fg = border.get_fg().unwrap_or(term.clear_tile.fg_color);
            let bg = border.get_bg().unwrap_or(term.clear_tile.bg_color);

            let mut tile_at = |x: usize, y: usize, glyph: char| {
                let xy = vec3(x as f32, y as f32, 0.0);
                let p = origin + xy;

                let vi = vert_data.verts.len() as u32;
                verts
                    .extend(&[p + up, p, p + right + up, p + right].map(|p| p.to_array()));
                indices
                    .extend(&[vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);

                let glyph_uv = mapping.uvs_from_glyph(glyph);
                uvs.extend(glyph_uv);
                fg_colors
                    .extend(std::iter::repeat(fg.as_linear_rgba_f32()).take(4));
                bg_colors
                    .extend(std::iter::repeat(bg.as_linear_rgba_f32()).take(4));
            };

            let top = height - 1;
            let bottom = 0;
            let left = 0;
            let right = width - 1;

            tile_at(left, bottom, border.bottom_left);
            tile_at(left, top, border.top_left);
            tile_at(right, top, border.top_right);
            tile_at(right, bottom, border.bottom_right);

            for x in 1..width - 1 {
                tile_at(x, bottom, border.bottom);
                tile_at(x, top, border.top);
            }

            for y in 1..height - 1 {
                tile_at(left, y, border.left);
                tile_at(right, y, border.right);
            }
        } else {
            visibility.is_visible = false;
        }

        mesh.insert_vert_data(verts);
        mesh.insert_tile_data(uvs, fg_colors, bg_colors);
    }
}

pub(crate) fn update_tiles(
    mut q_term: Query<(&Terminal, &mut TileData, &UvMapping, &Mesh2dHandle), 
        Or<(Changed<Terminal>,Changed<TerminalLayout>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (term, mut tile_data, mapping, mesh) in &mut q_term {
        let mesh = meshes.get_mut(&mesh.0).expect("Error retrieving terminal mesh");

        let (mut uvs, mut fgcol, mut bgcol) = mesh.get_tile_data();

        let tile_count = term.size().len();

        uvs.clear();
        fgcol.clear();
        bgcol.clear();

        uvs.reserve(tile_count * 4);
        fgcol.reserve(tile_count * 4);
        bgcol.reserve(tile_count * 4);
        
        for tile in term.iter() {
            let glyph_uv = mapping.uvs_from_glyph(tile.glyph);

            uvs.extend(glyph_uv);

            fgcol
                .extend(std::iter::repeat(tile.fg_color.as_linear_rgba_f32()).take(4));
            bgcol
                .extend(std::iter::repeat(tile.bg_color.as_linear_rgba_f32()).take(4));
        }

        mesh.insert_tile_data(uvs, fgcol, bgcol);
    }
}
