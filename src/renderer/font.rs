use bevy::{
    asset::HandleId,
    prelude::{
        info, Assets, Commands, Component, Entity, Handle, Image, IntoSystemConfigs, Plugin, Query,
        Res, ResMut, Resource, Update,
    },
    reflect::Reflect,
    render::texture::{ImageSampler, ImageType},
    utils::HashMap,
};

use std::borrow::Borrow;

use crate::TerminalMaterial;

use super::TerminalChangeFont;

/// Helper component for changing the terminal's font
///
/// You can add this to a terminal entity to change fonts.
///
/// You can also change fonts by assigning a new image
/// handle directly to the `TerminalMaterial`.
///
/// # Example
///
/// ```rust no_run
/// use bevy_ascii_terminal::{prelude::*, TerminalFont};
/// use bevy::prelude::*;
///
/// fn change_font(
///     mut commands: Commands,
///     q_term: Query<Entity, With<Terminal>>,
///     server: Res<AssetServer>,
/// ) {
///     for e in q_term.iter() {
///         // Change to a built in font
///         commands.entity(e).insert(TerminalFont::Pastiche8x8);
///
///         // Change to a custom font
///         let my_font = server.load("myfont.png");
///         commands.entity(e).insert(TerminalFont::Custom(my_font));
///     }
/// }
///
/// ```
#[derive(Debug, Clone, Component, Eq, PartialEq, Hash, Reflect, Default)]
pub enum TerminalFont {
    JtCurses12x12,
    Pastiche8x8,
    #[default]
    Px4378x8,
    Taffer10x10,
    ZxEvolution8x8,
    TaritusCurses8x12,
    /// Change to a custom font texture
    Custom(Handle<Image>),
}

impl TerminalFont {
    pub const fn file_name(&self) -> &'static str {
        match self {
            TerminalFont::JtCurses12x12 => "jt_curses_12x12.png",
            TerminalFont::Pastiche8x8 => "pastiche_8x8.png",
            TerminalFont::Px4378x8 => "px437_8x8.png",
            TerminalFont::Taffer10x10 => "taffer_10x10.png",
            TerminalFont::ZxEvolution8x8 => "zx_evolution_8x8.png",
            TerminalFont::TaritusCurses8x12 => "taritus_curses_8x12.png",
            TerminalFont::Custom(_) => "custom",
        }
    }
}

/// Load a built in font [`Image`] from it's name
macro_rules! include_font {
    ($font:expr, $path:literal) => {{
        let bytes = include_bytes!(concat!("builtin/", $path));
        let mut image = Image::from_buffer(
            bytes,
            ImageType::Extension("png"),
            bevy::render::texture::CompressedImageFormats::NONE,
            false,
        )
        .unwrap();
        image.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::nearest_descriptor());
        ($font, image)
    }};
}

fn add_font_resource(
    font: (TerminalFont, Image),
    images: &mut Assets<Image>,
    font_map: &mut HashMap<TerminalFont, Handle<Image>>,
) -> Handle<Image> {
    let handle = images.set(font.0.clone(), font.1);
    font_map.insert(font.0, handle.clone());
    handle
}

#[derive(Resource)]
/// An internal resource for tracking built in font handles.
pub(crate) struct BuiltInFontHandles {
    map: HashMap<TerminalFont, Handle<Image>>,
}

impl BuiltInFontHandles {
    /// Retrieve a built-in font handle via it's enum variant.
    pub(crate) fn get(&self, font: impl Borrow<TerminalFont>) -> &Handle<Image> {
        let font = font.borrow();
        self.map
            .get(font)
            .unwrap_or_else(|| panic!("Error retrieving built in font: {:#?} not found", font))
    }
}

impl From<TerminalFont> for HandleId {
    fn from(font: TerminalFont) -> Self {
        font.file_name().into()
    }
}

fn terminal_renderer_change_font(
    built_in_fonts: Res<BuiltInFontHandles>,
    mut q_change: Query<(Entity, &mut Handle<TerminalMaterial>, &TerminalFont)>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
) {
    for (e, mut mat, font) in q_change.iter_mut() {
        let handle = match font {
            TerminalFont::Custom(handle) => handle,
            _ => built_in_fonts.get(font),
        };

        // The requested font might still be loading, this is why we remove
        // the TerminalFont component rather than using change detection
        if images.get(handle).is_none() {
            return;
        }

        info!("Changing material");
        *mat = materials.add(handle.clone().into());
        commands.entity(e).remove::<TerminalFont>();
    }
}

pub(crate) struct TerminalFontPlugin;

impl Plugin for TerminalFontPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut fonts = BuiltInFontHandles {
            map: HashMap::default(),
        };
        let font_map = &mut fonts.map;

        let mut images = app
            .world
            .get_resource_mut::<Assets<Image>>()
            .unwrap_or_else(|| {
                panic!(
                    "Error retrieving image resource - ensure \
                    DefaultPlugins are added before TerminalPlugin \
                    during app initialization"
                )
            });

        let font = include_font!(TerminalFont::JtCurses12x12, "jt_curses_12x12.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!(TerminalFont::Pastiche8x8, "pastiche_8x8.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!(TerminalFont::Px4378x8, "px437_8x8.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!(TerminalFont::Taffer10x10, "taffer_10x10.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!(TerminalFont::ZxEvolution8x8, "zx_evolution_8x8.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!(TerminalFont::TaritusCurses8x12, "taritus_curses_8x12.png");
        add_font_resource(font, &mut images, font_map);

        app.insert_resource(fonts);

        app.add_systems(
            Update,
            terminal_renderer_change_font
                //.after(TERMINAL_INIT)
                .in_set(TerminalChangeFont),
        );
    }
}
