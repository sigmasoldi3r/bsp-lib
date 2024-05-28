use std::io::{Read, Seek};

use bytemuck::try_cast_slice;

use crate::{
    header::{BspHeader, BspLumpPointer, LUMP_LIGHTING, LUMP_MODELS, LUMP_VISIBILITY},
    lumps::{light_map::{BspLightMap, BspLightMapLump}, models::{BspModel, BspModelsLump}, vis::BspVisLump},
};

use super::{seek_and_extract, BspParseError, LumpExtractor, PtrLumpReader};

impl PtrLumpReader for BspVisLump {
    fn read_from_ptr<T>(_read: &mut T, _ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        Ok(BspVisLump(vec![]))
    }
}
impl LumpExtractor<BspVisLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_VISIBILITY.0]
    }
}

impl PtrLumpReader for BspLightMapLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let nodes: &[BspLightMap] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspLightMapLump(nodes.to_owned()))
    }
}
impl LumpExtractor<BspLightMapLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_LIGHTING.0]
    }
}

impl PtrLumpReader for BspModelsLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let faces: &[BspModel] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspModelsLump(faces.to_owned()))
    }
}
impl LumpExtractor<BspModelsLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_MODELS.0]
    }
}
