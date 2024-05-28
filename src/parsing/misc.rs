use std::io::{Read, Seek};

use crate::{header::BspLumpPointer, lumps::vis::BspVisLump};

use super::{BspParseError, PtrLumpReader};

impl PtrLumpReader for BspVisLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        Ok(BspVisLump(vec![]))
    }
}
