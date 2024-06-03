use std::ops::Index;

use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspFace {
    pub i_plane: u16,
    pub n_plane_side: u16,
    pub i_first_edge: u32,
    pub n_edges: u16,
    pub i_texture_info: u16,
    pub n_styles: [u8; 4],
    pub n_lightmap_offset: i32,
}

/// # Faces
///
/// The face lump contains the surfaces of the scene. Once again an array of
/// structs:
///
/// ```c
/// typedef struct _BSPFACE {
///     uint16_t iPlane;          // Plane the face is parallel to
///     uint16_t nPlaneSide;      // Set if different normals orientation
///     uint32_t iFirstEdge;      // Index of the first surfedge
///     uint16_t nEdges;          // Number of consecutive surfedges
///     uint16_t iTextureInfo;    // Index of the texture info structure
///     uint8_t nStyles[4];       // Specify lighting styles
///     int32_t nLightmapOffset;  // Offset into raw lightmap data
///                               // If < 0, no lightmap was baked for this face
/// } BSPFACE;
/// ```
///
/// The first number of this data structure is an index into the planes lump
/// giving a plane which is parallel to this face (meaning they share the same
/// normal). The second value may be seen as a boolean. If nPlaneSide equals 0,
/// then the normal vector of this face equals the one of the parallel plane
/// exactly. Otherwise, the normal of the plane has to be multiplied by -1 to
/// point in the right direction. Afterwards, we have an index into the
/// surfedges lump, as well as the count of consecutive surfedges from that
/// position. Furthermore, there is an index into the texture info lump, which
/// is used to find the BSPTEXINFO structure needed to calculate the texture
/// coordinates for this face. Afterwards, there are four bytes giving some
/// lighting information (partly used by the renderer to hide sky surfaces).
/// Finally, we have an offset in bytes giving the beginning of the binary
/// lightmap data of this face in the lighting lump.
#[derive(Debug)]
pub struct BspFacesLump(pub Vec<BspFace>);

impl Index<usize> for BspFacesLump {
    type Output = BspFace;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
