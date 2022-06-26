use block_mesh::{MergeVoxel, Voxel as BlockyVoxel};
use dot_vox::Model;
use ndshape::RuntimeShape;
use ndshape::Shape;

// trait implementation rules requires the use of a newtype to allow meshing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Voxel(pub(crate) u8);

pub(crate) const EMPTY_VOXEL: Voxel = Voxel(255);

impl BlockyVoxel for Voxel {
    fn get_visibility(&self) -> block_mesh::VoxelVisibility {
        match self.0 {
            255 => block_mesh::VoxelVisibility::Empty,
            _ => block_mesh::VoxelVisibility::Opaque,
        }
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = Voxel;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}

pub(crate) fn load_from_model(model: &Model) -> (RuntimeShape<u32, 3>, Vec<Voxel>) {
    let model_shape =
        RuntimeShape::<u32, 3>::new([model.size.x + 2, model.size.z + 2, model.size.y + 2]);
    let mut data = vec![EMPTY_VOXEL; model_shape.size() as usize];

    model.voxels.iter().for_each(|voxel| {
        let index =
            model_shape.linearize([voxel.x as u32 + 1, voxel.z as u32 + 1, voxel.y as u32 + 1])
                as usize;
        data[index] = Voxel(voxel.i);
    });

    (model_shape, data)
}
