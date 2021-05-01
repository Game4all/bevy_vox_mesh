use bevy::prelude::*;

mod loader;
mod mesh;
mod mesher;

pub struct VoxMeshPlugin {
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
