use std::{io::{Read, Seek}, mem::size_of};

use bytemuck::try_cast_slice;

use crate::{
    header::BspLumpPointer,
    lumps::{
        nodes::BspNodesLump,
        planes::{BspPlane, BspPlanesLump},
        vertices::BspVerticesLump,
    },
};

use super::{seek_and_extract, BspParseError, PtrLumpReader};

impl PtrLumpReader for BspPlanesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let planes: &[BspPlane] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspPlanesLump(planes.to_owned()))
    }
}

impl PtrLumpReader for BspVerticesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        // let buffer = [0u8; MAX_MAP_VERTS.0];
        // TODO: WHAT THE HECK MAN? How do I read this?
        Ok(BspVerticesLump(vec![]))
    }
}

impl PtrLumpReader for BspNodesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        todo!()
    }
}
