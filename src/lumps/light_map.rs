pub struct BspLightMap(pub u8, pub u8, pub u8);

/// # Lightmap
///
/// This is one of the largest lumps in the BSP file. The lightmap lump stores 
/// all lightmaps used in the entire map. The lightmaps are arrays of triples of 
/// bytes (3 channel color, RGB) and stored continuously.
pub struct BspLightMapLump(pub Vec<BspLightMap>);
