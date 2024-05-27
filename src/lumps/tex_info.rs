use crate::math::Vector3D;

pub struct TexInfo {
  pub v_s: Vector3D,
  pub f_s_shift: f32,
  pub v_t: Vector3D,
  pub f_t_shift: f32,
  pub i_miptex: u32,
  pub n_flags: u32
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
pub struct BspTexInfoLump(pub Vec<TexInfo>);
