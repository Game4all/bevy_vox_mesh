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
//!   let second_mesh = asset_loader.load("my_voxel_model.vox#Model1"); 
//!}
//!```


use bevy::prelude::*;

mod loader;
mod mesh;
mod mesher;

/// The core of this plugin.
/// Allows loading .vox files as usable meshes.
pub struct VoxMeshPlugin {
    /// Whether to flip the UVs vertically when meshing the models.
    /// You may want to change this to `false` if you aren't using Vulkan as a graphical backend for bevy.
    /// Defaults to `true`
    pub flip_uvs_vertically: bool,
}

impl Default for VoxMeshPlugin {
    fn default() -> Self {
        Self {
            flip_uvs_vertically: true, //UVs should be flipped vertically by default as the main backend of WGPU is Vulkan
        }
    }
}

impl Plugin for VoxMeshPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset_loader(loader::VoxLoader {
            flip_uvs_vertically: self.flip_uvs_vertically,
        });
    }
}
