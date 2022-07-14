use bevy::{asset::HandleId, prelude::*, render::{texture::{ImageType, ImageSampler}}, utils::HashMap};

use std::str::FromStr;

use strum_macros::{AsRefStr, EnumCount, EnumIter, EnumString};

/// Helper component for changing the terminal's font

/// You can add this to a terminal entity to change fonts.
///
/// You can also change fonts by assigning a new image
/// handle directly to the `TerminalMaterial`.
#[derive(
    Debug, Clone, Component, Eq, PartialEq, Hash, AsRefStr, EnumString, EnumCount, EnumIter,
)]
pub enum TerminalFont {
    #[strum(serialize = "jt_curses_12x12.png")]
    JtCurses12x12,
    #[strum(serialize = "pastiche_8x8.png")]
    Pastiche8x8,
    #[strum(serialize = "px437_8x8.png")]
    Px4378x8,
    #[strum(serialize = "taffer_10x10.png")]
    Taffer10x12,
    #[strum(serialize = "taritus_curses_8x12.png")]
    ZxEvolution8x8,
    #[strum(serialize = "zx_evolution_8x8.png")]
    TaritusCurses8x12,
    /// Change to a custom font texture
    Custom(Handle<Image>),
}

impl Default for TerminalFont {
    fn default() -> Self {
        TerminalFont::Px4378x8
    }
}

/// Load a built in font [`Image`] from it's name
macro_rules! include_font {
    ($font:expr) => {{
        let bytes = include_bytes!(concat!("builtin/", $font));
        let mut image = Image::from_buffer(
            bytes,
            ImageType::Extension("png"),
            bevy::render::texture::CompressedImageFormats::NONE,
            false,
        )
        .unwrap();
        image.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::nearest_descriptor()); 
        (
            TerminalFont::from_str($font).unwrap(),
            image
        )
    }};
}

pub struct TerminalFontPlugin;

impl Plugin for TerminalFontPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut fonts = BuiltInFontHandles {
            map: HashMap::default(),
        };
        let font_map = &mut fonts.map;

        let mut images = app.world.get_resource_mut::<Assets<Image>>()
            .unwrap_or_else(|| panic!("Error retrieving image resource - ensure
            DefaultPlugins are initialized before TerminalPlugin"));

        let font = include_font!("jt_curses_12x12.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!("pastiche_8x8.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!("px437_8x8.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!("taffer_10x10.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!("zx_evolution_8x8.png");
        add_font_resource(font, &mut images, font_map);

        let font = include_font!("taritus_curses_8x12.png");
        add_font_resource(font, &mut images, font_map);
        app.insert_resource(fonts);
    }
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

/// A resource which can be used to retrieve the image handles
/// for the terminal's built-in fonts.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use bevy_ascii_terminal::*;
/// fn change_font_built_in(
/// fonts: Res<BuiltInFontHandles>,
/// mut materials: ResMut<Assets<TerminalMaterial>>,
/// q_mat: Query<&Handle<TerminalMaterial>>,
/// ) {
///     for mat in q_mat.iter() {
///         let mut mat = materials.get_mut(mat).unwrap();
///         let built_in = fonts.get("zx_evolution_8x8.png").unwrap();
///
///         mat.texture = Some(built_in.clone());
///     }
/// }
/// ```
pub struct BuiltInFontHandles {
    pub(crate) map: HashMap<TerminalFont, Handle<Image>>,
}

impl BuiltInFontHandles {
    /// Retrieve a built-in font handle by it's name. Must include ".png" the extension.
    pub fn get(&self, font: &TerminalFont) -> &Handle<Image> {
        self.map
            .get(font)
            .unwrap_or_else(|| panic!("Error retrieving built in font: {:#?} not found", font))
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl From<TerminalFont> for HandleId {
    fn from(font: TerminalFont) -> Self {
        font.as_ref().into()
    }
}
