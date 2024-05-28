use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspMarkSurface(pub u16);

/// # Marksurfaces
///
/// The marksurfaces lump is a simple array of short integers.
///
/// ```c
/// typedef uint16_t BSPMARKSURFACE;
/// ```
///
/// This lump is a simple table for redirecting the marksurfaces indexes in the 
/// leafs to the actual face indexes. A leaf inserts its marksurface indexes into 
/// this array and gets the associated faces contained within this leaf.
#[derive(Debug)]
pub struct BspMarkSurfacesLump(pub Vec<BspMarkSurface>);

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspEdge {
  pub i_vertex: [u16; 2]
}

/// # Edges
///
/// The edges are defined as an array of structs:
///
/// ```c
/// typedef struct _BSPEDGE {
///     uint16_t iVertex[2]; // Indices into vertex array
/// } BSPEDGE;
/// ```
///
/// The edges delimit the face and further refer to the vertices of the face. 
/// Each edge is pointing to the start and end vertex of the edge.
#[derive(Debug)]
pub struct BspEdgesLump(pub Vec<BspEdge>);

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspSurfEdge(pub i32);

/// # Surfedges
///
/// Another array of integers.
///
/// ```c
/// typedef int32_t BSPSURFEDGE;
/// ```
///
/// This lump represents pretty much the same mechanism as the marksurfaces. A 
/// face can insert its surfedge indexes into this array to get the corresponding 
/// edges delimiting the face and further pointing to the vertices, which are 
/// required for rendering. The index can be positive or negative. If the value 
/// of the surfedge is positive, the first vertex of the edge is used as a vertex 
/// for rendering the face; otherwise, the value is multiplied by -1 and the 
/// second vertex of the indexed edge is used.
#[derive(Debug)]
pub struct BspSurfEdgesLump(pub Vec<BspSurfEdge>);
