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

mod loader;
mod voxel_scene;
pub use voxel_scene::VoxelSceneBundle;
#[doc(inline)]
use loader::VoxSceneLoader;
mod mesh;
mod voxel;

/// The core plugin adding functionality for loading `.vox` files.
///
/// Registers an [`bevy::asset::AssetLoader`] capable of loading modes in `.vox` files as usable [`bevy::render::mesh::Mesh`].
pub struct VoxScenePlugin;

impl Plugin for VoxScenePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_asset::<voxel_scene::VoxelScene>()
        .register_asset_loader(VoxSceneLoader)
        .add_systems(Update, voxel_scene::spawn_vox_scenes);
    }
}
