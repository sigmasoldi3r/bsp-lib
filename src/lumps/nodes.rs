pub struct BspNode {
  pub i_plane: i32,
  pub i_children: [i16; 2],
  pub n_mins: [i16; 3],
  pub n_maxs: [i16; 3],
  pub first_face: u16,
  pub n_faces: u16
}

/// # Nodes
///
/// This lump is simple again and contains an array of binary structures, the 
/// nodes, which are a major part of the BSP tree.
///
/// Every BSPNODE structure represents a node in the BSP tree and every node 
/// equals more or less a division step of the BSP algorithm. Therefore, each 
/// node has an index (iPlane) referring to a plane in the plane lump which 
/// divides the node into its two child nodes. The child nodes are also stored 
/// as indexes. Contrary to the plane index, the node index for the child is 
/// signed. If the index is larger than 0, the index indicates a child node. If 
/// it is equal to or smaller than zero (no valid array index), the bitwise 
/// inversed value of the index gives an index into the leaves lump. 
/// Additionally, two points (nMins, nMaxs) span the bounding box (AABB, axis 
/// aligned bounding box) delimiting the space of the node. Finally, firstFace 
/// indexes into the face lump and specifies the first of nFaces surfaces 
/// contained in this node.
pub struct BspNodesLump(pub Vec<BspNode>);
