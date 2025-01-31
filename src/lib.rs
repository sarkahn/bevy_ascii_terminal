pub mod ascii;
pub mod border;
//pub mod grid;
pub mod colors;
pub mod render;
pub(crate) mod rex;
pub mod string;
pub mod terminal;
pub mod tile;
pub mod transform;

pub use ascii::Glyph;
use bevy::{
    app::{Plugin, PostUpdate},
    prelude::IntoSystemSetConfigs,
};
pub use border::TerminalBorder;
pub use render::{TerminalCamera, TerminalFont, TerminalMeshPivot, TerminalMeshWorldScaling};
pub use sark_grids::{GridPoint, GridRect, GridSize};
pub use string::StringDecorator;
pub use terminal::Terminal;
pub use tile::Tile;
use transform::TerminalSystemsUpdateTransform;
pub use transform::TerminalTransform;

pub struct TerminalPlugins;

impl Plugin for TerminalPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TerminalMeshWorldScaling::default());
        app.add_plugins((
            transform::TerminalTransformPlugin,
            render::TerminalUvMappingPlugin,
            render::TerminalMaterialPlugin,
            render::TerminalMeshPlugin,
            render::TerminalFontPlugin,
            render::TerminalCameraPlugin,
        ));
        app.configure_sets(
            PostUpdate,
            TerminalSystemsUpdateTransform.before(render::TerminalSystemsUpdateMesh),
        );
    }
}
