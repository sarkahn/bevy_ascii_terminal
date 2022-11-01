//! Terminal components

use bevy::{
    prelude::{Bundle, Component, Deref, Handle, UVec2},
    sprite::MaterialMesh2dBundle,
};

use crate::TerminalMaterial;

use super::{
    mesh_data::{TileData, VertData},
    uv_mapping::UvMapping,
};

#[derive(Component, Deref)]
pub struct TerminalSize(pub UVec2);

/// Terminal component specifying how terminal mesh tiles will be scaled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileScaling {
    /// Each tile will take up 1 unit of world space vertically. This is the default setting.
    World,
    /// Scale terminal tiles based on the size of their texture.
    ///
    /// With this setting, 1 pixel == 1 world unit. This matches the expected
    /// defaults for bevy's orthographic camera.
    Pixels,
}

/// Bundle for a rendering a terminal.
/// Has various functions to help with the construction of a terminal.
#[derive(Default, Bundle)]
pub struct TerminalRenderBundle {
    pub render_bundle: MaterialMesh2dBundle<TerminalMaterial>,
    pub uv_mapping: Handle<UvMapping>,
    pub tile_data: TileData,
    pub vert_data: VertData,
}

impl TerminalRenderBundle {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Component)]
pub(crate) struct TerminalBorder;

#[cfg(test)]
mod test {
    // use super::*;
    // #[test]
    // fn boundss() {
    //     let b = bounds(Pivot::TopRight, [1,1], [3,3]);
    //     let [min,max] = b.min_max_i();
    //     println!("{:?}\nMin {}, max {}", b, min, max);

    //     // let b = bounds(Pivot::TopLeft, [0,0], [3,3]);
    //     // let [min,max] = b.min_max_i();
    //     // println!("{:?}\nMin {}, max {}", b, min, max);

    //     // let b = bounds(Pivot::BottomLeft, [0,0], [3,3]);
    //     // let [min,max] = b.min_max_i();
    //     // println!("{:?}\nMin {}, max {}", b, min, max);

    //     // let b = bounds(Pivot::BottomRight, [0,0], [3,3]);
    //     // let [min,max] = b.min_max_i();
    //     // println!("{:?}\nMin {}, max {}", b, min, max);
    // }
    // fn bounds(pivot: Pivot, pos: impl GridPoint, size: impl Size2d) -> GridRect {
    //     let pivot = Vec2::from(pivot);
    //     let bl = -pivot * size.as_vec2();
    //     GridRect::from_bl(bl.as_ivec2() + pos.as_ivec2(), size)
    //     //let offset = size.as_vec2().div(2.0) * pivot;
    //     //GridRect::new(pos.as_ivec2() - offset.as_ivec2(), size)
    // }
}
