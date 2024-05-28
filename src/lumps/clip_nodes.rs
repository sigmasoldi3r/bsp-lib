use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspClipNode {
  pub i_plane: i32,
  pub i_children: [i16; 2]
}

/// # Clipnodes
///
/// This lump contains the so-called clipnodes, which build a second BSP tree 
/// used only for collision detection.
///
/// ```c
/// typedef struct _BSPCLIPNODE {
///     int32_t iPlane;       // Index into planes
///     int16_t iChildren[2]; // negative numbers are contents
/// } BSPCLIPNODE;
/// ```
///
/// This structure is a reduced form of the BSPNODE struct from the nodes lump. 
/// Also, the BSP tree built by the clipnodes is simpler than the one described 
/// by the BSPNODEs to accelerate collision calculations.
#[derive(Debug)]
pub struct BspClipNodesLump(pub Vec<BspClipNode>);
