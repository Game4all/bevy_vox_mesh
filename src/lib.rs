use bevy::prelude::*;

mod loader;

pub struct VoxMeshPlugin;

impl Plugin for VoxMeshPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset_loader(loader::VoxLoader::default());
    }
}
