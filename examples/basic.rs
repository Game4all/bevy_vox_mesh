use bevy::prelude::*;
use bevy_vox_mesh::VoxMeshPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(VoxMeshPlugin)
        .run();
}
