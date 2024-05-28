use std::io::{Read, Seek, SeekFrom};

use crate::{
    header::BspLumpPointer,
    lumps::textures::{BspMipTex, BspMipTexOffset, BspTextureHeader, BspTexturesLump},
};

use super::{extract_struct, seek_ptr, BspParseError, PtrLumpReader};

impl PtrLumpReader for BspTexturesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        seek_ptr(read, ptr)?;
        let header: BspTextureHeader = extract_struct(read)?;
        let count = header
            .n_mip_textures
            .try_into()
            .map_err(BspParseError::BadPointerValue)?;
        let mut mip_tex_offsets: Vec<BspMipTexOffset> = Vec::with_capacity(count);
        for _ in 0..header.n_mip_textures {
            let value: BspMipTexOffset = extract_struct(read)?;
            mip_tex_offsets.push(value);
        }
        let mut textures: Vec<BspMipTex> = Vec::with_capacity(count);
        for index in mip_tex_offsets {
            let offset = index.0 + ptr.n_offset;
            read.seek(SeekFrom::Start(
                offset.try_into().map_err(BspParseError::BadPointerValue)?,
            ))
            .map_err(BspParseError::GenericError)?;
            let mip: BspMipTex = extract_struct(read)?;
            textures.push(mip);
        }
        Ok(BspTexturesLump(textures))
    }
}
