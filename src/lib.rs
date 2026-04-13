pub mod ascii;
pub mod border;
pub mod color;
mod deprecated;
pub mod padding;
pub mod pivot;
pub mod render;
pub(crate) mod rexpaint;
pub mod strings;
pub mod terminal;
pub mod tile;
pub mod transform;

pub use ascii::Glyph;
use bevy::{
    app::{Plugin, PostUpdate},
    prelude::IntoScheduleConfigs,
};
#[allow(deprecated)]
pub use border::TerminalBorder;
pub use padding::{BoxStyle, Padding};
pub use pivot::Pivot;
pub use render::{TerminalCamera, TerminalFont, TerminalMeshPivot, TerminalMeshWorldScaling};
pub use strings::{
    TerminalString, TerminalStringBuilder, Token, TokenIterator, wrap_line_count, wrap_string,
    wrap_tagged_line_count, wrap_tagged_string,
};
pub use terminal::Terminal;
pub use tile::Tile;
use transform::TerminalSystemsUpdateTransform;
pub use transform::{SetTerminalGridPosition, SetTerminalLayerPosition, TerminalTransform};

#[allow(deprecated)]
pub use deprecated::{GridPoint, GridRect, GridSize, PivotedPoint};

pub struct TerminalPlugins;

impl Plugin for TerminalPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TerminalMeshWorldScaling::default());
        app.add_plugins((
            transform::TerminalTransformPlugin, // 'PostUpdate' systems
            render::TerminalUvMappingPlugin,
            render::TerminalMaterialPlugin,
            render::TerminalFontPlugin,   // 'PostUpdate' systems
            render::TerminalCameraPlugin, // 'First` systems
            render::TerminalMeshPlugin,   // 'PostUpdate' systems
        ));
        app.configure_sets(
            PostUpdate,
            TerminalSystemsUpdateTransform.before(render::TerminalSystemsUpdateMesh),
        );
    }
}
