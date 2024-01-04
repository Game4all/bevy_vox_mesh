//! A plugin for the bevy engine which allows loading .vox files as usable meshes.
//!
//! ```
//!use bevy::prelude::*;
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

use bevy::{
    app::{App, Plugin, Update},
    asset::AssetApp,
};
use block_mesh::{QuadCoordinateConfig, RIGHT_HANDED_Y_UP_CONFIG, OrientedBlockFace, AxisPermutation, Axis};

mod loader;
mod voxel_scene;
pub use voxel_scene::VoxelSceneBundle;
#[doc(inline)]
use loader::VoxLoader;
mod mesh;
mod voxel;

/// The core plugin adding functionality for loading `.vox` files.
///
/// Registers an [`bevy::asset::AssetLoader`] capable of loading modes in `.vox` files as usable [`bevy::render::mesh::Mesh`].
pub struct VoxMeshPlugin {
    config: QuadCoordinateConfig,
    v_flip_faces: bool,
}

impl VoxMeshPlugin {
    /// Creates a [`VoxMeshPlugin`] instance with the specified parameters
    ///
    /// # Arguments
    /// * `config` - The quad coordinates configuration ([`QuadCoordinateConfig`]) to use when meshing models.
    pub fn with_options(config: QuadCoordinateConfig, v_flip_faces: bool) -> Self {
        Self {
            config,
            v_flip_faces,
        }
    }
}

impl Default for VoxMeshPlugin {
    fn default() -> Self {
        Self::with_options(RIGHT_HANDED_Y_UP_CONFIG, true)
    }
}

impl Plugin for VoxMeshPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_asset::<voxel_scene::VoxelScene>()
        .register_asset_loader(VoxLoader {
            config: self.config.clone(),
            v_flip_face: self.v_flip_faces,
        })
        .add_systems(Update, voxel_scene::spawn_vox_scenes);
    }
}
