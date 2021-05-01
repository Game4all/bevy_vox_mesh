use building_blocks::mesh::{OrientedCubeFace, UnorientedQuad};

/// Helper struct to organize mesh data for bevy.
#[derive(Default)]
pub(crate) struct VoxMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[u8; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl VoxMesh {
    pub(crate) fn add_quad(
        &mut self,
        face: &OrientedCubeFace,
        quad: &UnorientedQuad,
        palette_index: u32,
        palette: &[[u8; 4]],
    ) {
        let start_index = self.positions.len() as u32;

        //todo: maybe use u8's instead of f32's for position and normal attributes since magica voxel max size per model per dimension is 256.

        self.positions
            .extend_from_slice(&face.quad_mesh_positions(quad));

        self.normals.extend_from_slice(&face.quad_mesh_normals());

        self.colors
            .extend_from_slice(&[palette[palette_index as usize]; 4]);

        //todo: make this configurable.
        self.uvs
            .extend_from_slice(&face.simple_tex_coords(true, &quad));

        self.indices
            .extend_from_slice(&face.quad_mesh_indices(start_index));
    }
}
