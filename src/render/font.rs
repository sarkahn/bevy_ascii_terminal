use bevy::{
    app::PostUpdate,
    asset::{embedded_asset, AssetPath, AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        query::Changed,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Query, Res, ResMut},
    },
    image::{Image, ImageLoaderSettings, ImageSampler},
    prelude::Plugin,
    reflect::Reflect,
    sprite::MeshMaterial2d,
};

use super::material::TerminalMaterial;

/// A component for easy swapping of terminal fonts.
///
/// A custom font can be used by specifying the asset path with [TerminalFont::Custom].
///
/// Note that all [TerminalFont]s will be loaded with [ImageSampler::nearest] filtering.
/// To prevent this you can set the font manually on the [TerminalMaterial].
///
/// ## Example:
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ascii_terminal::*;
///
/// fn setup(mut commands: Commands) {
///    commands.spawn((Terminal, TerminalFont::Custom("assets/MyFont.png")));
/// }
/// ```
#[derive(Debug, Component, Reflect, Default, Clone)]
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
    Custom(String),
}

/// System for updating the [TerminalMaterial] based on the [TerminalFont]. Runs
/// in [PostUpdate].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemsUpdateFont;

macro_rules! embed_path {
    () => {
        "embedded://bevy_ascii_terminal/render/built_in_fonts/"
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
            _ => panic!("Attempting to access embedded asset path for a custom terminal font"),
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

        app.add_systems(PostUpdate, update_font.in_set(TerminalSystemsUpdateFont));
    }
}

#[allow(clippy::type_complexity)]
fn update_font(
    mut q_term: Query<
        (&mut MeshMaterial2d<TerminalMaterial>, &TerminalFont),
        Changed<TerminalFont>,
    >,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    for (mut mat_handle, font) in &mut q_term {
        let image: Handle<Image> = match font {
            TerminalFont::Custom(path) => {
                server.load_with_settings(path, move |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest()
                })
            }
            _ => server.load_with_settings(
                font.asset_path(),
                move |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest()
                },
            ),
        };
        // Dont overwrite the default terminal material which may
        // be shared by many terminals.
        if mat_handle.id() == Handle::<TerminalMaterial>::default().id() {
            let mat = materials.add(TerminalMaterial {
                texture: Some(image),
                ..Default::default()
            });
            *mat_handle = MeshMaterial2d(mat);
        } else {
            let mat = materials
                .get_mut(&*mat_handle.clone())
                .expect("Error getting terminal material");
            mat.texture = Some(image);
        }
    }
}
