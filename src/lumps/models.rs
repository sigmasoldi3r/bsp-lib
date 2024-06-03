use std::ops::Index;

use bytemuck::{Pod, Zeroable};

use crate::{header::MAX_MAP_HULLS, math::Vector3D};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspModel {
    pub n_mins: [f32; 3],
    pub n_maxs: [f32; 3],
    pub v_origin: Vector3D,
    pub i_head_nodes: [i32; MAX_MAP_HULLS.0],
    pub n_vis_leafs: i32,
    pub i_first_face: i32,
    pub n_faces: i32,
}

/// # Models
///
/// Array of structs:
///
/// ```c
/// #define MAX_MAP_HULLS 4
///
/// typedef struct _BSPMODEL {
///     float nMins[3], nMaxs[3];          // Defines bounding box
///     VECTOR3D vOrigin;                  // Coordinates to move the
///                                        // coordinate system
///     int32_t iHeadnodes[MAX_MAP_HULLS]; // Index into nodes array
///     int32_t nVisLeafs;                 // ???
///     int32_t iFirstFace, nFaces;        // Index and count into faces
/// } BSPMODEL;
/// ```
///
/// A model is kind of a mini BSP tree. Its size is determined by the bounding
/// box spanned by the first two members of this struct. The major difference
/// between a model and the BSP tree holding the scene is that the models use a
/// local coordinate system for their vertices and just state its origin in
/// world coordinates. During rendering, the coordinate system is translated to
/// the origin of the model (glTranslate()) and moved back after the model's BSP
/// tree has been traversed. Furthermore, there are 4 indexes into node arrays.
/// The first one has proved to index the root node of the mini BSP tree used
/// for rendering. The other three indexes could probably be used for collision
/// detection, meaning they point into the clipnodes, but I am not sure about
/// this. The meaning of the next value is also somehow unclear to me. Finally,
/// there are direct indexes into the faces array, not taking the redirecting by
/// the marksurfaces.
#[derive(Debug)]

pub struct BspModelsLump(pub Vec<BspModel>);

impl Index<usize> for BspModelsLump {
    type Output = BspModel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
