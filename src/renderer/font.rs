//! A terminal font is a texture of glyphs layed out in a sprite sheet.
//!
//! By default the terminal expects a [code page 437](https://dwarffortresswiki.org/Tileset_repository)
//! texture with 16x16 characters. New font textures can be placed in the `assets/textures`
//! directory and they will be loaded when the application runs. The terminal font can be changed by
//! modifying the [TerminalFont] component on the terminal entity. Font data can be read from the
//! [TerminalFonts] resource.
//!
//! Texture sprites are mapped to glyphs via [super::glyph_mapping::GlyphMapping].
//!
//! ## Included Fonts
//! The terminal comes with several built in fonts:
//! - jt_curses_12x12.png
//! - pastiche_8x8.png
//! - px437_8x8.png
//! - taffer_10x10.png
//! - zx_evolution_8x8.png
//!
//! ## Examples
//!
//! ### Changing Fonts
//!
//! ```
//! use bevy_ascii_terminal::*;
//! use bevy_ascii_terminal::renderer::TerminalFont;
//! use bevy::prelude::*;
//!
//! fn change_font(
//!     mut q_font: Query<&mut TerminalFont>,
//! ) {
//!     let mut font = q_font.single_mut().unwrap();
//!     font.file_name = String::from("taffer_10x10.png");
//! }
//! ```
//!
//! ### Reading Font Data
//!
//! ```
//! use bevy_ascii_terminal::*;
//! use bevy_ascii_terminal::renderer::TerminalFonts;
//! use bevy::prelude::*;
//!
//! fn read_font_data(
//!     fonts: Res<TerminalFonts>,
//! ) {
//!     let font_data = fonts.get("pastiche_8x8.png");
//!     info!("Pastich glyphs are {} pixels high", font_data.1.tile_size.y);
//! }
//! ```

use bevy::{asset::LoadState, prelude::*};
use std::{collections::HashMap, path::PathBuf};

// TODO: Temp workaround for get_handle_path bug in bevy 0.5.0
// https://github.com/bevyengine/bevy/pull/2310
use std::fs;

use super::plugin::AppState;

pub const DEFAULT_FONT_NAME: &str = "px437_8x8.png";

pub const BUILT_IN_FONT_NAMES: &[&str] = &[
    "jt_curses_12x12.png",
    "pastiche_8x8.png",
    "px437_8x8.png",
    "taffer_10x10.png",
    "zx_evolution_8x8.png",
];

/// Terminal component that determines which texture is rendered by the terminal.
///
/// # Example
/// ```
/// use bevy_ascii_terminal::*;
/// use bevy_ascii_terminal::renderer::TerminalFont;
/// use bevy::prelude::*;
///
/// fn change_font(
///     mut q_font: Query<&mut TerminalFont>,
/// ) {
///     let mut font = q_font.single_mut().unwrap();
///     font.file_name = String::from("taffer_10x10.png");
/// }
/// ```
pub struct TerminalFont {
    /// The file name (including extension) of the texture to render
    pub file_name: String,
    /// The color on the texture that should be treated as the background
    pub clip_color: Color,
}
impl Default for TerminalFont {
    fn default() -> Self {
        Self {
            file_name: String::from(DEFAULT_FONT_NAME),
            clip_color: Color::BLACK,
        }
    }
}

/// Size data for a terminal font
#[derive(Debug)]
pub struct TerminalFontData {
    /// The number of tiles in the textures
    pub tile_count: UVec2,
    /// Size in pixels of a single sprite in the texture
    pub tile_size: UVec2,
    pub texture_size: UVec2,
}

impl TerminalFontData {
    pub fn from_texture(tex: &Texture) -> Self {
        let tex_size = UVec2::new(tex.size.width, tex.size.height);
        let tile_count = UVec2::new(16, 16);
        TerminalFontData {
            tile_count,
            tile_size: tex_size / tile_count,
            texture_size: tex_size,
        }
    }
}

/// Resource used to store and retrieve font data.
///
/// Fonts should not be added from code - to add a font, place the font texture
/// in the `assets/textures` directory.
#[derive(Default)]
pub struct TerminalFonts {
    map: HashMap<String, (Handle<Texture>, TerminalFontData)>,
}

impl TerminalFonts {
    pub(crate) fn add(&mut self, name: &str, handle: Handle<Texture>, data: TerminalFontData) {
        self.map.insert(name.to_string(), (handle, data));
    }

    pub fn get(&self, font_name: &str) -> &(Handle<Texture>, TerminalFontData) {
        &self.map[font_name]
    }
}

pub(crate) struct TerminalFontPlugin;
impl Plugin for TerminalFontPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<LoadingTerminalTextures>()
            .init_resource::<TerminalFonts>()
            .add_system_set(
                SystemSet::on_enter(AppState::AssetsLoading)
                    .with_system(terminal_load_fonts.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::AssetsLoading)
                    .with_system(terminal_check_loading_fonts.system()),
            );
    }
}

#[derive(Default)]
struct LoadingTerminalTextures(Option<Vec<HandleUntyped>>);

fn terminal_load_fonts(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingTerminalTextures>,
) {
    loading.0 = match asset_server.load_folder("textures") {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}

fn terminal_check_loading_fonts(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingTerminalTextures>,
    textures: Res<Assets<Texture>>,
    mut state: ResMut<State<AppState>>,
    mut fonts: ResMut<TerminalFonts>,
) {
    let loaded = loading.0.as_ref();

    if let LoadState::Loaded =
        asset_server.get_group_load_state(loaded.unwrap().iter().map(|h| h.id))
    {
        // TODO: Temporary workaround for get_handle_path bug in bevy 0.5.0. Replace with get_handle_path in next bevy version
        // https://github.com/bevyengine/bevy/pull/2310
        let dir = fs::read_dir("assets/textures");
        if dir.is_ok() {
            let dir = dir.unwrap();
            let paths: Vec<PathBuf> = dir.map(|entry| entry.unwrap().path()).collect();

            // Add any user fonts from the "assets/textures" directory
            for (handle, path) in loaded
                .unwrap()
                .iter()
                .map(|h| h.clone().typed())
                .zip(paths.iter())
            {
                let name = path.file_name().unwrap().to_str().unwrap();

                let tex = textures.get(handle.clone()).unwrap();
                let data = TerminalFontData::from_texture(tex);

                fonts.add(name, handle, data);
            }
        }

        state.set(AppState::AssetsDoneLoading).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use bevy::prelude::*;

    #[test]
    fn change_font() {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
    }
}
