use bevy::{
    ecs::prelude::*,
    math::vec3,
    prelude::{Assets, BuildChildren, Color, Handle, Image, Mesh, Vec2, Vec3, Visibility},
    render::render_resource::PrimitiveTopology,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use sark_grids::Size2d;

use crate::{Terminal, TerminalFont, TerminalMaterial};

use super::{
    mesh_data::MeshData, uv_mapping::UvMapping, TerminalBorder, TerminalLayout, TileScaling,
};

pub(crate) fn init_terminal(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(Entity, &mut Mesh2dHandle, &mut TerminalLayout), Added<Terminal>>,
    mut commands: Commands,
) {
    for (term_entity, mut mesh, mut layout) in q.iter_mut() {
        //info!("Initializing terminal mesh");
        // Initialize terminal mesh
        let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        new_mesh.init_mesh_data();

        *mesh = Mesh2dHandle(meshes.add(new_mesh));

        // Initialize border entity and mesh
        let mut border_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        border_mesh.init_mesh_data();
        let border_mesh: MaterialMesh2dBundle<TerminalMaterial> = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(border_mesh)),
            visibility: Visibility::INVISIBLE,
            ..Default::default()
        };

        let border_entity = commands.spawn_bundle((border_mesh, TerminalBorder)).id();

        layout.border_entity = Some(border_entity);

        commands.entity(term_entity).add_child(border_entity);
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn material_change(
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    mut q_term: Query<
        (&Handle<TerminalMaterial>, &mut TerminalLayout),
        Or<(Changed<Handle<TerminalMaterial>>, Changed<TerminalFont>)>,
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
                    layout.tile_size = match layout.scaling {
                        TileScaling::World => {
                            let aspect = font_size.x / font_size.y;
                            Vec2::new(aspect, 1.0)
                        }
                        TileScaling::Pixels => font_size,
                    };
                    //info!("Updating layout ppt. Now {}", layout.pixels_per_tile);
                }
            }
        }
    }
}

pub(crate) fn update_layout(
    mut q_term: Query<(&Terminal, &mut TerminalLayout), Changed<Terminal>>,
) {
    for (term, mut layout) in &mut q_term {
        if layout.term_size() != term.size() || layout.border.as_ref() != term.border() {
            layout.update_state(term);
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn layout_changed(
    mut q_term: Query<
        (&Terminal, &TerminalLayout, &UvMapping, &Mesh2dHandle),
        (Changed<TerminalLayout>, Without<TerminalBorder>),
    >,
    mut q_border: Query<(&mut Visibility, &Mesh2dHandle), With<TerminalBorder>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (term, layout, mapping, mesh) in &mut q_term {
        //info!("Updating mesh data from layout change");
        let tile_count = term.size().len();
        let width = term.width();
        let height = term.height();

        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error retrieving terminal mesh");

        // Remove vert data so we can modify it with indices without
        // the borrow checker complaining. Note indices are mutably
        // borrowed, not removed since they are separate from the attribute map.
        let mut vert_data = mesh.get_vert_data();
        vert_data.verts.clear();
        vert_data.indices.clear();
        // 4 verts per tile
        vert_data.verts.reserve(tile_count * 4);
        // 6 indices per tile
        vert_data.indices.reserve(tile_count * 6);

        let tile_size = layout.tile_size;
        let origin = layout.origin();
        let right = Vec3::X * tile_size.x;
        let up = Vec3::Y * tile_size.y;

        for i in 0..tile_count {
            let x = (i % width) as f32;
            let y = (i / width) as f32;

            let xy = vec3(x, y, 0.0) * tile_size.extend(0.0);
            let p = origin.extend(0.0) + xy;

            vert_data
                .verts
                .extend(&[p + up, p, p + right + up, p + right].map(|p| p.to_array()));
            let vi = (i * 4) as u32;
            vert_data
                .indices
                .extend(&[vi, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);
        }

        let verts = vert_data.verts;
        mesh.insert_vert_data(verts);

        //info!("Updating border");
        // Update border only when layout changes
        let (mut visibility, mesh) = q_border.get_mut(layout.border_entity.unwrap()).unwrap();

        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error retrieving the terminal border mesh");

        let mut td = mesh.get_tile_data();
        let mut vd = mesh.get_vert_data();

        vd.verts.clear();
        vd.indices.clear();

        td.uvs.clear();
        td.fg_cols.clear();
        td.bg_cols.clear();

        if let Some(border) = term.border() {
            visibility.is_visible = true;
            let tile_count = ((width + 2) * 2) + (height * 2);

            vd.verts.reserve(tile_count * 4);
            vd.indices.reserve(tile_count * 6);

            td.uvs.reserve(tile_count * 4);
            td.fg_cols.reserve(tile_count * 4);
            td.bg_cols.reserve(tile_count * 4);

            let origin = origin.extend(0.0) - tile_size.extend(0.0);
            let right = Vec3::X * tile_size.x;
            let up = Vec3::Y * tile_size.y;

            let mut tile_at = |x: usize, y: usize, glyph: char, fg: Color, bg: Color| {
                let xy = vec3(x as f32, y as f32, 0.0) * tile_size.extend(0.0);
                let p = origin + xy;

                let vi = vd.verts.len() as u32;
                vd.verts
                    .extend(&[p + up, p, p + right + up, p + right].map(|p| p.to_array()));
                vd.indices
                    .extend(&[vi, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);

                let glyph_uv = mapping.uvs_from_glyph(glyph);
                td.uvs.extend(glyph_uv);
                td.fg_cols
                    .extend(std::iter::repeat(fg.as_linear_rgba_f32()).take(4));
                td.bg_cols
                    .extend(std::iter::repeat(bg.as_linear_rgba_f32()).take(4));
            };

            let top = height + 1;
            let bottom = 0;
            let left = 0;
            let right = width + 1;

            // If border color was not explicitly set, use terminal clear tile
            // colors
            let fg = border.get_fgcol().unwrap_or(term.clear_tile.fg_color);
            let bg = border.get_bgcol().unwrap_or(term.clear_tile.bg_color);

            tile_at(left, bottom, border.bottom_left, fg, bg);
            tile_at(left, top, border.top_left, fg, bg);
            tile_at(right, top, border.top_right, fg, bg);
            tile_at(right, bottom, border.bottom_right, fg, bg);

            if let Some(title) = &border.title {
                let stringlen = title.string.chars().count();

                let title_offset = stringlen as f32 * title.align;
                let align_offset = ((width - 1) as f32) * title.align;
                let beg = 1 + (align_offset - title_offset).floor() as usize;
                let end = beg + stringlen;

                for x in 1..beg {
                    tile_at(x, bottom, border.bottom, fg, bg);
                    tile_at(x, top, border.top, fg, bg);
                }

                for (x, titlechar) in (beg..end).zip(title.string.chars()) {
                    tile_at(x, bottom, border.bottom, fg, bg);
                    tile_at(x, top, titlechar, title.color, bg);
                }

                for x in end..width + 1 {
                    tile_at(x, bottom, border.bottom, fg, bg);
                    tile_at(x, top, border.top, fg, bg);
                }
            } else {
                for x in 1..width + 1 {
                    tile_at(x, bottom, border.bottom, fg, bg);
                    tile_at(x, top, border.top, fg, bg);
                }
            }

            for y in 1..height + 1 {
                tile_at(left, y, border.left, fg, bg);
                tile_at(right, y, border.right, fg, bg);
            }
        } else {
            visibility.is_visible = false;
        }

        let verts = vd.verts;
        mesh.insert_vert_data(verts);
        mesh.insert_tile_data(td);
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_tiles(
    mut q_term: Query<
        (&Terminal, &UvMapping, &Mesh2dHandle),
        Or<(Changed<Terminal>, Changed<TerminalLayout>)>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (term, mapping, mesh) in &mut q_term {
        //info!("Updating tile data from terminal change");
        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error retrieving terminal mesh");

        let mut td = mesh.get_tile_data();

        let tile_count = term.size().len();

        td.uvs.clear();
        td.fg_cols.clear();
        td.bg_cols.clear();

        td.uvs.reserve(tile_count * 4);
        td.fg_cols.reserve(tile_count * 4);
        td.bg_cols.reserve(tile_count * 4);

        for tile in term.iter() {
            let glyph_uv = mapping.uvs_from_glyph(tile.glyph);

            td.uvs.extend(glyph_uv);

            td.fg_cols
                .extend(std::iter::repeat(tile.fg_color.as_linear_rgba_f32()).take(4));
            td.bg_cols
                .extend(std::iter::repeat(tile.bg_color.as_linear_rgba_f32()).take(4));
        }

        mesh.insert_tile_data(td);
    }
}
