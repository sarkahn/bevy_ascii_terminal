use bevy::{prelude::*, asset::AssetLoader, utils::BoxedFuture};
use ldtk_rust::Project;

pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset_loader(LdtkAssetLoader);
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkAssetLoader;

impl AssetLoader for LdtkAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let p: Project = serde_json::from_slice(bytes)?;
            println!("Level count: {}", p.levels.len());
            
            for l in p.levels {
                if let Some(layers) = l.layer_instances {
                    for layer in layers {
                        println!("Layer {} tilecount {}, autotilecount {}", 
                        layer.identifier, 
                        layer.grid_tiles.len(),
                        layer.auto_layer_tiles.len());
                    }
                }
            }

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}