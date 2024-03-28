use bevy::{
    app::PostUpdate,
    asset::{embedded_asset, AssetPath, AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        schedule::SystemSet,
        system::{Commands, Query, Res, ResMut},
    },
    prelude::Plugin,
    reflect::Reflect,
    render::texture::{ImageLoaderSettings, ImageSampler},
};
use enum_ordinalize::Ordinalize;

use super::material::TerminalMaterial;

/// System for tracking camera and cursor data.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalFontSystems;

/// Allows for simple loading of built in terminal fonts.
///
/// A [TerminalFont] component can be set during [TerminalBundle] creation or added
/// to an existing terminal entity to change fonts.
///
/// [TerminalBundle]: crate::TerminalBundle
///
/// ## Example:
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ascii_terminal::*;
///
/// fn setup(mut commands: Commands) {
///     commands.spawn(TerminalBundle::new([10,10]).with_builtin_font(TerminalFont::Rexpaint8x8));
/// }
/// ```
#[derive(Debug, Component, Ordinalize, Reflect, Default, Clone, Copy)]
pub enum TerminalFont {
    #[default]
    Px4378x8,
    ZxEvolution8x8,
    Pastiche8x8,
    Rexpaint8x8,
    Unscii8x8,
    Px4378x16,
    Taffer10x10,
    TaritusCurses8x12,
    JtCurses12x12,
    SazaroteCurses12x12,
}

macro_rules! embed_path {
    () => {
        "embedded://bevy_ascii_terminal/renderer/built_in_fonts/"
    };
}

impl TerminalFont {
    pub const fn asset_path(&self) -> &'static str {
        match self {
            TerminalFont::Unscii8x8 => concat!(embed_path!(), "unscii_8x8.png"),
            TerminalFont::JtCurses12x12 => concat!(embed_path!(), "jt_curses_12x12.png"),
            TerminalFont::Px4378x8 => concat!(embed_path!(), "px437_8x8.png"),
            TerminalFont::Px4378x16 => concat!(embed_path!(), "px437_8x16.png"),
            TerminalFont::Taffer10x10 => concat!(embed_path!(), "taffer_10x10.png"),
            TerminalFont::SazaroteCurses12x12 => {
                concat!(embed_path!(), "sazarote_curses_12x12.png")
            }
            TerminalFont::TaritusCurses8x12 => concat!(embed_path!(), "taritus_curses_8x12.png"),
            TerminalFont::ZxEvolution8x8 => concat!(embed_path!(), "zx_evolution_8x8.png"),
            TerminalFont::Pastiche8x8 => concat!(embed_path!(), "pastiche_8x8.png"),
            TerminalFont::Rexpaint8x8 => concat!(embed_path!(), "rexpaint_8x8.png"),
        }
    }
}

impl From<TerminalFont> for AssetPath<'_> {
    fn from(value: TerminalFont) -> Self {
        value.asset_path().into()
    }
}

pub(crate) struct TerminalFontPlugin;

impl Plugin for TerminalFontPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        embedded_asset!(app, "built_in_fonts/unscii_8x8.png");
        embedded_asset!(app, "built_in_fonts/jt_curses_12x12.png");
        embedded_asset!(app, "built_in_fonts/px437_8x8.png");
        embedded_asset!(app, "built_in_fonts/px437_8x16.png");
        embedded_asset!(app, "built_in_fonts/taffer_10x10.png");
        embedded_asset!(app, "built_in_fonts/sazarote_curses_12x12.png");
        embedded_asset!(app, "built_in_fonts/taritus_curses_8x12.png");
        embedded_asset!(app, "built_in_fonts/zx_evolution_8x8.png");
        embedded_asset!(app, "built_in_fonts/pastiche_8x8.png");
        embedded_asset!(app, "built_in_fonts/rexpaint_8x8.png");

        app.add_systems(PostUpdate, update_font);
    }
}

#[allow(clippy::type_complexity)]
fn update_font(
    mut q_term: Query<(Entity, &mut Handle<TerminalMaterial>, &TerminalFont)>,
    server: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    for (e, mut mat_handle, font) in &mut q_term {
        if mat_handle.id() == Handle::<TerminalMaterial>::default().id() {
            *mat_handle = materials.add(TerminalMaterial {
                texture: Some(server.load_with_settings(
                    font.asset_path(),
                    move |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest()
                    },
                )),
                ..Default::default()
            });
        } else {
            let mat = materials
                .get_mut(mat_handle.clone())
                .expect("Error getting terminal material");
            mat.texture = Some(server.load_with_settings(
                font.asset_path(),
                move |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest()
                },
            ));
        }

        commands.entity(e).remove::<TerminalFont>();
    }
}
