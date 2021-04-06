use anyhow::{anyhow, Error};
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};

#[derive(Default)]
pub struct VoxLoader;

use crate::mesher::mesh_model;

impl AssetLoader for VoxLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            load_magica_voxel_file(bytes, load_context)?;
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vox"]
    }
}

fn load_magica_voxel_file<'a>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
) -> Result<(), Error> {
    let file = match dot_vox::load_bytes(bytes) {
        Ok(data) => data,
        Err(error) => return Err(anyhow!(error)),
    };

    let palette: Vec<[u8; 4]> = file
        .palette
        .iter()
        .map(|color| color.to_le_bytes())
        .collect();

    for (index, model) in file.models.iter().enumerate() {
        let mesh = mesh_model(model, &palette);

        match index {
            0 => load_context.set_default_asset(LoadedAsset::new(mesh)),
            _ => {
                load_context.set_labeled_asset(&format!("Model{}", index), LoadedAsset::new(mesh));
            }
        }
    }

    Ok(())
}
