use building_blocks::mesh::{OrientedCubeFace, UnorientedQuad};

#[derive(Default)]
/// Helper struct to organize mesh data for bevy.
pub(crate) struct VoxMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[u8; 4]>,
    pub indices: Vec<u32>,
}

impl VoxMesh {
    pub(crate) fn add_quad_(
        &mut self,
        face: &OrientedCubeFace,
        quad: &UnorientedQuad,
        palette_index: u32,
        palette: &Vec<[u8; 4]>,
    ) {
        let start_index = self.positions.len() as u32;

        self.positions
            .extend_from_slice(&face.quad_mesh_positions(quad));

        self.normals.extend_from_slice(&face.quad_mesh_normals());

        self.colors
            .extend_from_slice(&[palette[palette_index as usize]; 4]);

        self.indices
            .extend_from_slice(&face.quad_mesh_indices(start_index));
    }
}
