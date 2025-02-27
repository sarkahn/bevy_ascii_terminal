use bevy::{
    app::PostUpdate,
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        query::Changed,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Query, Res, ResMut, Resource},
    },
    image::{Image, ImageLoaderSettings, ImageSampler},
    prelude::Plugin,
    reflect::{Enum, Reflect},
    sprite::MeshMaterial2d,
};

use super::material::TerminalMaterial;

/// System for updating the [TerminalMaterial] based on the [TerminalFont]. Runs
/// in [PostUpdate].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemsUpdateFont;

/// A component for easy swapping of terminal fonts.
///
/// A custom font can be used by specifying the asset path with [TerminalFont::Custom].
///
/// Note that all [TerminalFont]s will be loaded with [ImageSampler::nearest] filtering.
/// To prevent this you can set the image handle manually on the [TerminalMaterial].
///
/// ## Example:
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ascii_terminal::*;
///
/// fn setup(mut commands: Commands) {
///    commands.spawn((
///         Terminal::new([10,10]),
///         TerminalFont::Custom("assets/MyFont.png".to_string())
///    ));
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

macro_rules! font_bytes {
    ($name:expr) => {
        include_bytes!(concat!("built_in_fonts/", $name, ".png"))
    };
}

/// Get a font image from a built-in font path.
macro_rules! font_image {
    ($name:expr) => {
        Image::from_buffer(
            font_bytes!($name),
            bevy::image::ImageType::Format(bevy::image::ImageFormat::Png),
            bevy::image::CompressedImageFormats::NONE,
            false,
            ImageSampler::nearest(),
            bevy::asset::RenderAssetUsages::default(),
        )
        .expect("Error loading font image")
    };
}

pub(crate) struct TerminalFontPlugin;

impl Plugin for TerminalFontPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        let v = vec![
            images.add(font_image!("px437_8x8")),
            images.add(font_image!("zx_evolution_8x8")),
            images.add(font_image!("pastiche_8x8")),
            images.add(font_image!("rexpaint_8x8")),
            images.add(font_image!("unscii_8x8")),
            images.add(font_image!("px437_8x16")),
            images.add(font_image!("taffer_10x10")),
            images.add(font_image!("taritus_curses_8x12")),
            images.add(font_image!("jt_curses_12x12")),
            images.add(font_image!("sazarote_curses_12x12")),
        ];
        app.insert_resource(FontHandles { handles: v });
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
    handles: Res<FontHandles>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    for (mut mat_handle, font) in &mut q_term {
        let image: Handle<Image> = match font {
            TerminalFont::Custom(path) => {
                server.load_with_settings(path, move |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest()
                })
            }
            _ => handles.handles[font.variant_index()].clone(),
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

#[derive(Resource, Default)]
struct FontHandles {
    handles: Vec<Handle<Image>>,
}
