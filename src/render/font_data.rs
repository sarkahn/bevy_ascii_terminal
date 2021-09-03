use std::{collections::HashMap, path::PathBuf};

use bevy::{asset::LoadState, prelude::*};

// TODO: Temp workaround for get_handle_path bug in bevy 0.5.0
use std::fs;

use super::AppState;

#[derive(Debug)]
pub struct TerminalFontData {
    pub name: String,
    pub path: String,
    pub tile_count: UVec2,
    pub tile_size: UVec2,
    pub texture_size: UVec2,
}

#[derive(Default)]
pub struct TerminalFonts {
    map: HashMap<String, TerminalFontData>,
}

impl TerminalFonts {
    pub fn add(&mut self, data: TerminalFontData) {
        self.map.insert(data.name.clone(), data);
    }

    pub fn get(&self, font_name: &str) -> &TerminalFontData {
        &self.map[font_name]
    }
}

#[derive(Default)]
pub struct LoadingTerminalTextures(Vec<HandleUntyped>);

pub(crate) fn terminal_load_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingTerminalTextures>,
) {
    //info!("Loading terminal textures");
    loading.0 = asset_server
        .load_folder("textures")
        .expect("Error loading terminal textures folder");
}

pub(crate) fn check_terminal_assets_loading(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingTerminalTextures>,
    textures: Res<Assets<Texture>>,
    mut state: ResMut<State<AppState>>,
    mut fonts: ResMut<TerminalFonts>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        let textures: Vec<&Texture> = loading.0.iter().map(|h| textures.get(h).unwrap()).collect();
        // TODO: Temporary workaround for get_handle_path bug in bevy 0.5.0. Replace in next bevy version
        let dir = fs::read_dir("assets/textures").expect("Error reading textures directory");
        let paths: Vec<PathBuf> = dir.map(|entry| entry.unwrap().path()).collect();

        for (tex, path) in textures.iter().zip(paths.iter()) {
            let tex_size = UVec2::new(tex.size.width, tex.size.height);
            let tile_count = UVec2::new(16, 16);
            fonts.add(TerminalFontData {
                name: String::from(path.file_name().unwrap().to_str().unwrap()),
                path: String::from(path.to_str().unwrap()),
                tile_count,
                tile_size: tex_size / tile_count,
                texture_size: tex_size,
            });
        }

        state.set(AppState::AssetsDoneLoading).unwrap();
    }
}
