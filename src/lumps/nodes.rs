use bytemuck::{Pod, Zeroable};

///// Each node has exactly two children, which can be either another node or a leaf.
/// A child node has two further children, and so on until all branches of the tree
/// are terminated with leaves, which have no children. Each node also references
/// a plane in the plane array.
///
/// When determining the player's viewpoint, the engine is trying to find which leaf
/// the viewpoint falls inside. It first compares the coordinates of the point with
/// the plane referenced in the headnode (Node 0). If the point is in front of the
/// plane, it then moves to the first child of the node; otherwise, it moves to the
/// second child. If the child is a leaf, then it has completed its task. If it is
/// another node, it then performs the same check against the plane referenced in
/// this node, and follows the children as before. It therefore traverses the BSP
/// tree until it finds which leaf the viewpoint lies in. The leaves, then,
/// completely divide up the map volume into a set of non-overlapping, convex
/// volumes defined by the planes of their parent nodes.
///
/// For more information on how the BSP tree is constructed, see the article
/// "BSP for dummies".
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspNode {
  pub plane_index: i32,
  pub children_indices: [i32; 2],
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
#[derive(Debug)]
pub struct BspNodesLump(pub Vec<BspNode>);
