use bevy::prelude::*;
use bevy_ascii_terminal::{renderer::TerminalMaterial, *};

fn main() {
    App::new()
        .init_resource::<Images>()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (on_image_event, on_material_event, set_image))
        .run();
}

#[derive(Default, Resource)]
pub struct Images {
    a: Handle<Image>,
    b: Handle<Image>,
}

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut images: ResMut<Images>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    images.a = server.load("pastiche_8x8.png");
    images.b = server.load("unscii_8x8.png");
    let mat = materials.add(TerminalMaterial::default());
    commands.spawn(mat);
}

fn on_material_event(mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>) {
    for evt in mat_evt.read() {
        match evt {
            AssetEvent::Added { id } => println!("MATERIAL ADDED"),
            AssetEvent::Modified { id } => println!("MATERIAL MODIFIED"),
            AssetEvent::Removed { id } => println!("MATERIAL REMOVED"),
            AssetEvent::Unused { id } => println!("MATERIAL UNUSED"),
            AssetEvent::LoadedWithDependencies { id } => println!("MATERIAL LOADED"),
        };
    }
}

fn on_image_event(mut img_evt: EventReader<AssetEvent<Image>>) {
    for evt in img_evt.read() {
        match evt {
            AssetEvent::Added { id } => println!("IMAGE ADDED"),
            AssetEvent::Modified { id } => println!("IMAGE MODIFIED"),
            AssetEvent::Removed { id } => println!("IMAGE REMOVED"),
            AssetEvent::Unused { id } => println!("IMAGE UNUSED"),
            AssetEvent::LoadedWithDependencies { id } => println!("IMAGE LOADED"),
        };
    }
}

fn set_image(
    input: Res<ButtonInput<KeyCode>>,
    q_mat: Query<&Handle<TerminalMaterial>>,
    images: Res<Images>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    if input.just_pressed(KeyCode::KeyA) {
        let mat = q_mat.single();
        let mat = materials
            .get_mut(mat.clone())
            .expect("Error getting material");
        println!("Setting image to A");
        mat.texture = Some(images.a.clone());
    }
    if input.just_pressed(KeyCode::KeyB) {
        let mat = q_mat.single();
        let mat = materials
            .get_mut(mat.clone())
            .expect("Error getting material");
        println!("Setting image to B");
        mat.texture = Some(images.b.clone());
    }
}
