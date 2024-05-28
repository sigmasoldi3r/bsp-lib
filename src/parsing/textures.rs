use std::io::{Read, Seek, SeekFrom};

use bytemuck::try_cast_slice;

use crate::{
    header::{BspHeader, BspLumpPointer, LUMP_TEXINFO, LUMP_TEXTURES},
    lumps::{
        tex_info::{BspTexInfoLump, TexInfo},
        textures::{BspMipTex, BspMipTexOffset, BspTextureHeader, BspTexturesLump},
    },
};

use super::{
    extract_struct, seek_and_extract, seek_ptr, BspParseError, LumpExtractor, PtrLumpReader,
};

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
impl LumpExtractor<BspTexturesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_TEXTURES.0]
    }
}

impl PtrLumpReader for BspTexInfoLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let tex_info_entries: &[TexInfo] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspTexInfoLump(tex_info_entries.to_owned()))
    }
}
impl LumpExtractor<BspTexInfoLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_TEXINFO.0]
    }
}
