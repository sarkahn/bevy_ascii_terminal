//! A terminal font is a texture of glyphs layed out in a sprite sheet.
//!
//! By default the terminal expects a [code page 437](https://dwarffortresswiki.org/Tileset_repository)
//! texture with 16x16 characters. New font textures can be placed in the `assets/textures`
//! directory and they will be loaded when the application runs.
//!
//! The terminal can handle non-square fonts but only if using [TileScaling::Pixels](super::TileScaling::Pixels).
//! TileScaling is a component on the terminal that can be changed during construction or at runtime.
//!
//! The terminal font can be changed by modifying the [TerminalFont] component on the terminal entity.
//! Font data can be read from the [TerminalFonts] resource.
//!
//! Texture sprites are mapped to glyphs via [GlyphMapping](super::glyph_mapping::GlyphMapping).
//!
//! ## Included Fonts
//! The terminal comes with several built in fonts:
//! - jt_curses_12x12.png
//! - pastiche_8x8.png
//! - px437_8x8.png
//! - taffer_10x10.png
//! - zx_evolution_8x8.png
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
//!     font.change_font("taffer_10x10.png");
//! }
//! ```

use bevy::render::texture::ImageType;
use bevy::{asset::LoadState, prelude::*};
use std::{collections::HashMap, path::PathBuf};

// TODO: Temp workaround for get_handle_path bug in bevy 0.5.0
// https://github.com/bevyengine/bevy/pull/2310
use std::fs;

use super::plugin::TerminalAssetLoadState;

/// Terminal component that determines which texture is rendered by the terminal.
///
/// Also contains various functions to inspect details of the font.
#[derive(Component)]
pub struct TerminalFont {
    /// The file name (including extension) of the texture to render
    name: String,
    /// The color on the texture that should be treated as the background
    clip_color: Color,
    tex_handle: Handle<Image>,
    pixels_per_unit: UVec2,
    //tile_count: UVec2,
}
impl Default for TerminalFont {
    fn default() -> Self {
        Self {
            name: String::from(DEFAULT_FONT_NAME),
            clip_color: Color::BLACK,
            tex_handle: Default::default(),
            pixels_per_unit: Default::default(),
            //tile_count: Default::default(),
        }
    }
}

impl TerminalFont {
    /// The file name of the font, including extension.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Change the terminal's font.
    ///
    /// The name provided must be the full file name of the font,
    /// including extension.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    /// use bevy_ascii_terminal::renderer::TerminalFont;
    /// use bevy::prelude::*;
    ///
    /// fn change_font_system(
    ///     mut q_font: Query<&mut TerminalFont>,
    /// ) {
    ///     let mut font = q_font.single_mut().unwrap();
    ///     font.change_font("taffer_10x10.png");
    /// }
    /// ```
    pub fn change_font(&mut self, font_name: &str) {
        self.name = String::from(font_name);
    }

    /// The clip color of the font texture.
    ///
    /// Clip color determines which part of the texture is regarded as
    /// "background color".
    pub fn clip_color(&self) -> Color {
        self.clip_color
    }

    /// Change the clip color of the font's texture.
    ///
    /// Clip color determines which part of the texture is regarded as
    /// "background color".
    pub fn change_clip_color(&mut self, color: Color) {
        self.clip_color = color;
    }

    /// How many pixels for a single character.
    pub fn pixels_per_unit(&self) -> UVec2 {
        self.pixels_per_unit
    }

    /// Handle to the underlying font texture.
    pub(crate) fn texture_handle(&self) -> &Handle<Image> {
        &self.tex_handle
    }

    /// Construct a font from a texture. This assumes the texture has already been
    /// added to `Assets<Texture>`
    pub(crate) fn from_texture(
        name: &str,
        tex_handle: Handle<Image>,
        textures: &Assets<Image>,
    ) -> Self {
        let image = textures.get(tex_handle.clone_weak()).unwrap();
        let size = &image.texture_descriptor.size;

        let tex_size = UVec2::new(size.width,size.height);

        let tile_count = UVec2::new(16, 16);
        let pixels_per_tile = tex_size / tile_count;

        TerminalFont {
            name: String::from(name),
            pixels_per_unit: pixels_per_tile,
            tex_handle,
            //tile_count,
            ..Default::default()
        }
    }
}

/// Resource used to store and retrieve terminal fonts.
///
/// Fonts should not be added from user code - to add a font, place the font texture
/// in the `assets/textures` directory.
#[derive(Default)]
pub struct TerminalFonts {
    map: HashMap<String, TerminalFont>,
}

impl TerminalFonts {
    pub(crate) fn add(&mut self, font: TerminalFont) {
        self.map.insert(font.name.clone(), font);
    }

    /// Retrieve a font by name.
    ///
    /// The name provided must be the full file name of the font, including extension.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::*;
    /// use bevy_ascii_terminal::renderer::TerminalFonts;
    /// use bevy::prelude::*;
    ///
    /// fn read_font_data(
    ///     fonts: Res<TerminalFonts>,
    /// ) {
    ///     let font = fonts.get("pastiche_8x8.png");
    ///     info!("Pastich glyphs are {} pixels high", font.pixels_per_unit());
    /// }
    /// ```
    pub fn get(&self, font_name: &str) -> &TerminalFont {
        &self.map[font_name]
    }
}

/// Plugin for systems and resources related to terminal font rendering.
pub(crate) struct TerminalFontPlugin;
impl Plugin for TerminalFontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingTerminalTextures>()
            .init_resource::<TerminalFonts>()
            .add_system_set(
                SystemSet::on_enter(TerminalAssetLoadState::AssetsLoading)
                    .with_system(terminal_load_fonts.system()),
            )
            .add_system_set(
                SystemSet::on_update(TerminalAssetLoadState::AssetsLoading)
                    .with_system(terminal_check_loading_fonts.system()),
            );
    }
}

#[derive(Default)]
struct LoadingTerminalTextures(Option<Vec<HandleUntyped>>);

fn terminal_load_fonts(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingTerminalTextures>,
    mut textures: ResMut<Assets<Image>>,
    mut fonts: ResMut<TerminalFonts>,
) {
    loading.0 = match asset_server.load_folder("textures") {
        Ok(fonts) => Some(fonts),
        Err(_) => Some(Vec::new()),
    };

    load_built_in_fonts(&mut fonts, &mut textures);
}

fn load_built_in_fonts(fonts: &mut TerminalFonts, textures: &mut ResMut<Assets<Image>>) {
    for font_data in BUILT_IN_FONTS {
        let tex = Image::from_buffer(font_data.bytes, ImageType::Extension("png")).unwrap();
        let handle = textures.add(tex);

        let font = TerminalFont::from_texture(font_data.name, handle, textures);
        fonts.add(font);
    }
}

fn terminal_check_loading_fonts(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingTerminalTextures>,
    textures: ResMut<Assets<Image>>,
    mut state: ResMut<State<TerminalAssetLoadState>>,
    mut fonts: ResMut<TerminalFonts>,
) {
    let loaded = loading.0.as_ref();

    if loaded.is_none() {
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(loaded.unwrap().iter().map(|h| h.id))
    {
        // TODO: Temporary workaround for get_handle_path bug in bevy 0.5.0. Replace with get_handle_path in next bevy version
        // https://github.com/bevyengine/bevy/pull/2310
        let dir = fs::read_dir("assets/textures");
        if let Ok(dir) = dir {
            let paths: Vec<PathBuf> = dir.map(|entry| entry.unwrap().path()).collect();

            // Add any user fonts from the "assets/textures" directory
            for (handle, path) in loaded
                .unwrap()
                .iter()
                .map(|h| h.clone().typed())
                .zip(paths.iter())
            {
                let name = path.file_name().unwrap().to_str().unwrap();

                let font = TerminalFont::from_texture(name, handle, &textures);

                fonts.add(font);
            }
        }

        state
            .set(TerminalAssetLoadState::AssetsDoneLoading)
            .unwrap();
    }
}

pub const DEFAULT_FONT_NAME: &str = "px437_8x8.png";

macro_rules! BUILT_IN_FONT_PATH {
    () => {
        "../../embedded/"
    };
}

macro_rules! include_font {
    ($font_name:expr) => {
        include_bytes!(concat!(BUILT_IN_FONT_PATH!(), $font_name))
    };
}

pub struct BuiltInFontData<'a> {
    pub name: &'a str,
    pub bytes: &'a [u8],
}

pub const BUILT_IN_FONTS: &[BuiltInFontData] = &[
    BuiltInFontData {
        name: "jt_curses_12x12.png",
        bytes: include_font!("jt_curses_12x12.png"),
    },
    BuiltInFontData {
        name: "pastiche_8x8.png",
        bytes: include_font!("pastiche_8x8.png"),
    },
    BuiltInFontData {
        name: "px437_8x8.png",
        bytes: include_font!("px437_8x8.png"),
    },
    BuiltInFontData {
        name: "taffer_10x10.png",
        bytes: include_font!("taffer_10x10.png"),
    },
    BuiltInFontData {
        name: "zx_evolution_8x8.png",
        bytes: include_font!("zx_evolution_8x8.png"),
    },
];
