use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
    },
};
use building_blocks::{
    core::{Extent3i, PointN},
    prelude::Array3x1,
    storage::{Get, GetMut},
};
use building_blocks::{
    mesh::{greedy_quads, GreedyQuadsBuffer, IsOpaque, MergeVoxel},
    storage::IsEmpty,
};

use crate::mesh::VoxMesh;

//required as the builtin magica voxel voxel struct in building_blocks doesn't implement all the required traits

#[derive(Eq, PartialEq, Clone, Copy)]
enum Voxel {
    Full(u8),
    Empty,
}

impl IsEmpty for Voxel {
    fn is_empty(&self) -> bool {
        matches!(self, &Voxel::Empty)
    }
}

impl IsOpaque for Voxel {
    fn is_opaque(&self) -> bool {
        true
    }
}

impl MergeVoxel for Voxel {
    type VoxelValue = Voxel;

    fn voxel_merge_value(&self) -> Self::VoxelValue {
        *self
    }
}

pub(crate) fn mesh_model(model: &dot_vox::Model, palette: &[[u8; 4]]) -> Mesh {
    let extent = Extent3i::from_min_and_shape(
        PointN([0, 0, 0]),
        PointN([
            model.size.x as i32,
            model.size.y as i32,
            model.size.z as i32,
        ]),
    ).padded(1);

    let mut voxels = Array3x1::fill(extent, Voxel::Empty);

    for dot_vox::Voxel { x, y, z, i } in model.voxels.iter() {
        *voxels.get_mut(PointN([*x as i32, *z as i32, *y as i32])) = Voxel::Full(*i);
    }

    let mut greedy_buffer = GreedyQuadsBuffer::new_with_y_up(extent);
    greedy_quads(&voxels, &extent, &mut greedy_buffer);

    let mut mesh = VoxMesh::default();

    for group in greedy_buffer.quad_groups.iter() {
        for quad in group.quads.iter() {
            let index = match voxels.get(quad.minimum) {
                Voxel::Empty => unreachable!(),
                Voxel::Full(x) => x as u32,
            };

            mesh.add_quad(&group.face, quad, index, palette);
        }
    }

    let VoxMesh {
        positions,
        normals,
        colors,
        indices,
    } = mesh;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Uchar4Norm(colors),
    );
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
