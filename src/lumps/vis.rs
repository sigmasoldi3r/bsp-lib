// TODO: This might need some work?
pub struct BspVis;

/// # VIS
///
/// The VIS lump contains data, which is irrelevant to the actual BSP tree, but 
/// offers a way to boost the speed of the renderer significantly. Especially 
/// complex maps profit from the use of this data. This lump contains the 
/// so-called Potentially Visible Sets (PVS) (also called VIS lists) in the same 
/// amount of leaves of the tree the user can enter (often referred to as 
/// VisLeaves). The visibility lists are stored as sequences of bitfields, which 
/// are run-length encoded.
/// 
/// > **Important:**
///
/// > The generation of the VIS data is a very time consuming process if a map is 
/// > poorly optimized (several hours) and is also done by a separate compiler. It 
/// > can therefore be skipped when compiling the map, resulting in BSP files with 
/// > no VIS data at all!
pub struct BspVisLump(pub Vec<BspVis>);
