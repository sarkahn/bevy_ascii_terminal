pub mod pipeline;
mod entity;
mod terminal_renderer;


use bevy::prelude::*;

use pipeline::{setup_pipeline, TerminalRendererPipeline};
use terminal_renderer::{terminal_mesh_data_update, terminal_mesh_update, 
    update_terminal_mesh_verts, terminal_mesh_data_resize};


#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
struct TerminalStage;

struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TerminalRendererPipeline>();
        app.add_startup_system(setup_pipeline.system());
        app.add_system(terminal_mesh_data_resize.system());
        app.add_system(update_terminal_mesh_verts.system());
        app.add_system(terminal_mesh_data_update.system());
        app.add_system(terminal_mesh_update.system());

    }
}


