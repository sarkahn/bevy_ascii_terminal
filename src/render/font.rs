use std::{collections::HashMap, path::PathBuf};

use bevy::{asset::LoadState, prelude::*, reflect::TypeUuid, render::texture::ImageType};

// TODO: Temp workaround for get_handle_path bug in bevy 0.5.0
use std::fs;

use super::plugin::AppState;

macro_rules! DEFAULT_FONT_PATH {
    () => {
        "../../assets/textures/"
    };
}

macro_rules! include_font {
    ($font_name:expr) => {
        include_bytes!(concat!(DEFAULT_FONT_PATH!(), $font_name))
    };
}

pub struct TerminalFontBuiltIn<'a> {
    pub name: &'a str,
    pub(crate) bytes: &'a [u8],
}

pub const FONT_JT_CURSES_12X12: TerminalFontBuiltIn = TerminalFontBuiltIn {
    name: "jt_curses_12x12.png",
    bytes: include_font!("jt_curses_12x12.png"),
};
pub const FONT_PASTICHE_8X8: TerminalFontBuiltIn = TerminalFontBuiltIn {
    name: "pastiche_8x8.png",
    bytes: include_font!("pastiche_8x8.png"),
};
pub const FONT_PX437_8X8: TerminalFontBuiltIn = TerminalFontBuiltIn {
    name: "px437_8x8.png",
    bytes: include_font!("px437_8x8.png"),
};
pub const FONT_TAFFER_10X10: TerminalFontBuiltIn = TerminalFontBuiltIn {
    name: "taffer_10x10.png",
    bytes: include_font!("taffer_10x10.png"),
};

pub const FONT_ZX_EVOLUTION_8X8: TerminalFontBuiltIn = TerminalFontBuiltIn {
    name: "zx_evolution_8x8.png",
    bytes: include_font!("zx_evolution_8x8.png"),
};

pub(crate) const DEFAULT_FONT: TerminalFontBuiltIn = FONT_PX437_8X8;
pub(crate) const DEFAULT_FONT_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Texture::TYPE_UUID, 11121232112011521357);

#[derive(Debug)]
pub struct TerminalFontData {
    pub tile_count: UVec2,
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

#[derive(Default)]
pub(crate) struct TerminalFonts {
    map: HashMap<String, (Handle<Texture>, TerminalFontData)>,
}

impl TerminalFonts {
    pub fn add(&mut self, name: &str, handle: Handle<Texture>, data: TerminalFontData) {
        self.map.insert(name.to_string(), (handle, data));
    }

    pub fn get(&self, font_name: &str) -> &(Handle<Texture>, TerminalFontData) {
        &self.map[font_name]
    }
}

#[derive(Default)]
pub(crate) struct LoadingTerminalTextures(Option<Vec<HandleUntyped>>);

pub(crate) fn terminal_load_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingTerminalTextures>,
) {
    loading.0 = match asset_server.load_folder("textures") {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}

fn load_default_font(
    fonts: &mut ResMut<TerminalFonts>,
    textures: &mut ResMut<Assets<Texture>>,
    font: &TerminalFontBuiltIn,
) {
    let tex = Texture::from_buffer(font.bytes, ImageType::Extension("png")).unwrap();
    let data = TerminalFontData::from_texture(&tex);
    let handle = textures.add(tex);
    fonts.add(font.name, handle, data);
}

fn load_default_fonts(
    mut textures: &mut ResMut<Assets<Texture>>,
    mut fonts: &mut ResMut<TerminalFonts>,
) {
    load_default_font(&mut fonts, &mut textures, &FONT_JT_CURSES_12X12);
    load_default_font(&mut fonts, &mut textures, &FONT_PASTICHE_8X8);
    load_default_font(&mut fonts, &mut textures, &FONT_PX437_8X8);
    load_default_font(&mut fonts, &mut textures, &FONT_TAFFER_10X10);
    load_default_font(&mut fonts, &mut textures, &FONT_ZX_EVOLUTION_8X8);
}

pub(crate) fn check_terminal_assets_loading(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingTerminalTextures>,
    mut textures: ResMut<Assets<Texture>>,
    mut state: ResMut<State<AppState>>,
    mut fonts: ResMut<TerminalFonts>,
) {
    let loaded = loading.0.as_ref();

    if loaded.is_none() {
        load_default_fonts(&mut textures, &mut fonts);
        state.set(AppState::AssetsDoneLoading).unwrap();
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(loaded.unwrap().iter().map(|h| h.id))
    {
        load_default_fonts(&mut textures, &mut fonts);

        // TODO: Temporary workaround for get_handle_path bug in bevy 0.5.0. Replace with AssetServer in next bevy version
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
