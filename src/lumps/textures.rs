use std::ops::Index;

use bytemuck::{Pod, Zeroable};

/// The texture lump is somehow a bit more complex than
/// the other lumps, because it is possible to save
/// textures directly within the BSP file instead of
/// storing them in external WAD files. This lump also
/// starts with a small header:
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspTextureHeader {
    pub n_mip_textures: i32,
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspMipTexOffset(pub i32);

const MAX_TEXTURE_NAME: usize = 16;
const MIP_LEVELS: usize = 4;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct BspMipTex {
    pub sz_name: [u8; MAX_TEXTURE_NAME],
    pub n_width: i32,
    pub n_height: i32,
    pub n_offsets: [i32; MIP_LEVELS],
}
impl BspMipTex {
    pub fn name(&self) -> String {
        let first = self
            .sz_name
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(self.sz_name.len());
        String::from_utf8_lossy(&self.sz_name[..first]).into_owned()
    }
}
impl std::fmt::Debug for BspMipTex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BspMipTex")
            .field("sz_name", &self.name())
            .field("n_width", &self.n_width)
            .field("n_height", &self.n_height)
            .field("n_offsets", &self.n_offsets)
            .finish()
    }
}

/// # Textures
///
/// The texture lump is somewhat more complex than the other lumps, because it
/// is possible to save textures directly within the BSP file instead of storing
/// them in external WAD files. This lump also starts with a small header:
///
/// ```c
/// typedef struct _BSPTEXTUREHEADER {
///     uint32_t nMipTextures; // Number of BSPMIPTEX structures
/// } BSPTEXTUREHEADER;
/// ```
///
/// The header only consists of an unsigned 32-bit integer indicating the number
/// of stored or referenced textures in the texture lump. After the header
/// follows an array of 32-bit offsets pointing to the beginnings of the
/// separate textures.
///
/// ```c
/// typedef int32_t BSPMIPTEXOFFSET;
/// ```
///
/// Every offset gives the distance in bytes from the beginning of the texture
/// lump to one of the beginnings of the BSPMIPTEX structure, which are equal in
/// count to the value given in the texture header.
///
/// ```c
/// #define MAXTEXTURENAME 16
/// #define MIPLEVELS 4
/// typedef struct _BSPMIPTEX {
///     char szName[MAXTEXTURENAME];  // Name of texture
///     uint32_t nWidth, nHeight;     // Extents of the texture
///     uint32_t nOffsets[MIPLEVELS]; // Offsets to texture mipmaps BSPMIPTEX;
/// } BSPMIPTEX;
/// ```
///
/// Each of these structs describes a texture. The name of the texture is a
/// string and may be 16 characters long (including the null-character at the
/// end, char equals an 8-bit signed integer). The name of the texture is needed
/// if the texture has to be found and loaded from an external WAD file.
/// Furthermore, the struct contains the width and height of the texture. The 4
/// offsets at the end can either be zero if the texture is stored in an
/// external WAD file, or point to the beginnings of the binary texture data
/// within the texture lump relative to the beginning of its BSPMIPTEX struct.
#[derive(Debug)]
pub struct BspTexturesLump(pub Vec<BspMipTex>);

impl Index<usize> for BspTexturesLump {
    type Output = BspMipTex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
