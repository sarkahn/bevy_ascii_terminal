use std::{collections::HashMap, path::PathBuf};

use bevy::{asset::LoadState, prelude::*};

// TODO: Temp workaround for get_handle_path bug in bevy 0.5.0
use std::fs;

use super::{
    plugin::{DEFAULT_FONT_HANDLE, DEFAULT_FONT_NAME},
    AppState,
};

#[derive(Debug)]
pub struct TerminalFontData {
    pub handle: Handle<Texture>,
    pub tile_count: UVec2,
    pub tile_size: UVec2,
    pub texture_size: UVec2,
}

impl TerminalFontData {
    pub fn from_texture(handle: Handle<Texture>, tex: &Texture) -> Self {
        let tex_size = UVec2::new(tex.size.width, tex.size.height);
        let tile_count = UVec2::new(16, 16);
        TerminalFontData {
            handle,
            tile_count,
            tile_size: tex_size / tile_count,
            texture_size: tex_size,
        }
    }
}

#[derive(Default)]
pub struct TerminalFonts {
    map: HashMap<String, TerminalFontData>,
}

impl TerminalFonts {
    pub fn add(&mut self, name: &str, data: TerminalFontData) {
        self.map.insert(name.to_string(), data);
    }

    pub fn get(&self, font_name: &str) -> &TerminalFontData {
        &self.map[font_name]
    }
}

#[derive(Default)]
pub struct LoadingTerminalTextures(Option<Vec<HandleUntyped>>);

pub(crate) fn terminal_load_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingTerminalTextures>,
) {
    //info!("Loading terminal textures");
    loading.0 = match asset_server.load_folder("textures") {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}

pub(crate) fn check_terminal_assets_loading(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingTerminalTextures>,
    textures: Res<Assets<Texture>>,
    mut state: ResMut<State<AppState>>,
    mut fonts: ResMut<TerminalFonts>,
) {
    let loaded = loading.0.as_ref();

    if loaded.is_none() {
        // Add default font
        let handle: Handle<Texture> = DEFAULT_FONT_HANDLE.typed();
        let tex = textures.get(handle.clone()).unwrap();
        fonts.add(
            DEFAULT_FONT_NAME,
            TerminalFontData::from_texture(handle, tex),
        );

        state.set(AppState::AssetsDoneLoading).unwrap();
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(loaded.unwrap().iter().map(|h| h.id))
    {
        // Add default font
        let handle: Handle<Texture> = DEFAULT_FONT_HANDLE.typed();
        let tex = textures.get(handle.clone()).unwrap();
        fonts.add(
            DEFAULT_FONT_NAME,
            TerminalFontData::from_texture(handle, tex),
        );

        // TODO: Temporary workaround for get_handle_path bug in bevy 0.5.0. Replace in next bevy version
        let dir = fs::read_dir("assets/textures").expect("Error reading textures directory");
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
            fonts.add(name, TerminalFontData::from_texture(handle, tex));
        }

        state.set(AppState::AssetsDoneLoading).unwrap();
    }
}
