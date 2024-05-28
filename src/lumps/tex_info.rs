use bytemuck::{Pod, Zeroable};

use crate::math::Vector3D;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TexInfo {
  pub texture_s: TextureVector,
  pub texture_t: TextureVector,
  pub miptex_index: u32,
  pub texture_flags: u32
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TextureVector {
  vector: Vector3D,
  shift: f32
}

/// # Texinfo
///
/// The texinfo lump contains information about how textures are applied to 
/// surfaces. The lump itself is an array of binary data structures.
///
/// ```c
/// typedef struct _BSPTEXTUREINFO {
///     VECTOR3D vS;
///     float fSShift;    // Texture shift in s direction
///     VECTOR3D vT;
///     float fTShift;    // Texture shift in t direction
///     uint32_t iMiptex; // Index into textures array
///     uint32_t nFlags;  // Texture flags
/// } BSPTEXTUREINFO;
/// ```
///
/// This struct is mainly responsible for the calculation of the texture 
/// coordinates (vS, fSShift, vT, fTShift). These values determine the position 
/// of the texture on the surface. The iMiptex integer refers to the textures in 
/// the texture lump and would be the index in an array of BSPMITEX structs. 
/// Finally, there are 4 bytes used for flags. Only one flag is used by the 
/// vanilla engine, being 0x1 for disabling lightmaps and subdivision for the 
/// surface (used by sky and liquids).
#[derive(Debug)]
pub struct BspTexInfoLump(pub Vec<TexInfo>);
