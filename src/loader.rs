use anyhow::{anyhow, Error};
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};

use crate::mesher::mesh_model;

/// An asset loader which loads models in .vox files into usable [`bevy::render::mesh::Mesh`]es
#[derive(Default)]
pub struct VoxLoader {
    /// Whether to flip the UVs vertically when meshing the models.
    /// You may want to change this to false if you aren't using Vulkan as a graphical backend for bevy , else this should default to true.
    pub flip_uvs_vertically: bool,
}

impl AssetLoader for VoxLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            self.load_magica_voxel_file(bytes, load_context)?;
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vox"]
    }
}

impl VoxLoader {
    fn load_magica_voxel_file<'a>(
        &self,
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
            let mesh = mesh_model(model, &palette, self.flip_uvs_vertically);

            load_context
                .set_labeled_asset(&format!("model{}", index), LoadedAsset::new(mesh.clone()));
            if index == 0 {
                load_context.set_default_asset(LoadedAsset::new(mesh.clone()));
            }
        }

        Ok(())
    }
}
