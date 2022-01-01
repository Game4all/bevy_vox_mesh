use block_mesh::{MergeVoxel, Voxel as BlockyVoxel};
use dot_vox::Model;
use ndshape::{Shape, Shape3u32};

// trait implementation rules requires the use of a newtype to allow meshing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Voxel(pub(crate) u8);

pub(crate) const EMPTY_VOXEL: Voxel = Voxel(255);

impl BlockyVoxel for Voxel {
    fn is_empty(&self) -> bool {
        self.0 == EMPTY_VOXEL.0
    }

    fn is_opaque(&self) -> bool {
        self.0 != EMPTY_VOXEL.0
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = Voxel;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}

pub(crate) fn load_from_model(model: &Model) -> (Shape3u32, Vec<Voxel>) {
    let model_shape = Shape3u32::new([model.size.x + 2, model.size.z + 2, model.size.y + 2]);
    let mut data = vec![EMPTY_VOXEL; model_shape.size() as usize];

    model.voxels.iter().for_each(|voxel| {
        let index =
            model_shape.linearize([voxel.x as u32 + 1, voxel.z as u32 + 1, voxel.y as u32 + 1])
                as usize;
        data[index] = Voxel(voxel.i);
    });

    (model_shape, data)
}
