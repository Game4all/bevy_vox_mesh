//! A plugin for the bevy engine which allows loading .vox files as usable meshes.
//!
//! ```
//!use bevy::{
//!   prelude::*,
//!    render::{
//!        pipeline::{PipelineDescriptor, RenderPipeline},
//!        shader::{ShaderStage, ShaderStages},
//!    },
//!};
//!use bevy_vox_mesh::VoxMeshPlugin;
//!
//!fn main() {
//!     App::build()
//!        .add_plugins(DefaultPlugins)
//!        .add_plugin(VoxMeshPlugin::default())
//!        .add_startup_system(setup.system())
//!        .run();
//!}
//!
//!fn setup(asset_loader: Res<AssetServer>) {
//!   let mesh = asset_loader.load("my_voxel_model.vox");
//!   // you can select what model to load from a file if it contains multiple models by adding `#Model<model number here>` to the asset path to load.
//!   let second_mesh = asset_loader.load("my_voxel_model.vox#model1");
//!}
//!```

use bevy::prelude::*;

mod loader;
#[doc(inline)]
use loader::VoxLoader;

mod mesh;
mod mesher;

/// The core plugin adding functionality for loading `.vox` files.
///
/// Registers an [`bevy::asset::AssetLoader`] capable of loading modes in `.vox` files as usable [`bevy::render::mesh::Mesh`].
pub struct VoxMeshPlugin {
    flip_uvs_vertically: bool,
}

impl VoxMeshPlugin {
    /// Creates a [`VoxMeshPlugin`] instance with the specified parameters
    ///
    /// # Arguments
    /// * `flip_uvs_vertically` - Sets whether the mesh UVs should be flipped vertically when loading voxel models.
    pub fn with_options(flip_uvs_vertically: bool) -> Self {
        Self {
            flip_uvs_vertically,
        }
    }
}

impl Default for VoxMeshPlugin {
    fn default() -> Self {
        Self::with_options(true) //UVs should be flipped vertically by default as the main backend of WGPU is Vulkan
    }
}

impl Plugin for VoxMeshPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset_loader(VoxLoader {
            flip_uvs_vertically: self.flip_uvs_vertically,
        });
    }
}
