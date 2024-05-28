use bytemuck::{Pod, Zeroable};

use crate::math::Vector3D;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspVertex(pub Vector3D);

/// # Vertices
///
/// This lump simply consists of all vertices of the BSP tree. They are stored
/// as a primitive array of triples of floats.
///
/// ```c
/// typedef VECTOR3D BSPVERTEX;
/// ```
///
/// Each of these triples, obviously, represents a point in 3-dimensional space
/// by giving its three coordinates.
#[derive(Debug)]
pub struct BspVerticesLump(pub Vec<BspVertex>);
