use std::{
    io::{self, Read, Seek},
    num::TryFromIntError,
    string::FromUtf8Error,
};

use bytemuck::{from_bytes, PodCastError};
use peg::{error::ParseError, str::LineCol};

use crate::header::BspLumpPointer;

#[derive(Debug)]
pub enum BspParseError {
    InvalidVersion { valid: Vec<i32>, found: i32 },
    BadPointerValue(TryFromIntError),
    BadStringValue(FromUtf8Error),
    EntityLumpParseError(ParseError<LineCol>),
    GenericError(io::Error),
    DeserializationError(PodCastError),
}

pub trait PtrLumpReader {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized;
}

pub fn seek_and_extract<T: Read + Seek>(
    read: &mut T,
    ptr: &BspLumpPointer,
) -> Result<Vec<u8>, BspParseError> {
    seek_ptr(read, ptr)?;
    let mut buffer = vec![
        0u8;
        (ptr.n_length)
            .try_into()
            .map_err(BspParseError::BadPointerValue)?
    ];
    extract(read, &mut buffer)?;
    Ok(buffer)
}

pub fn extract(read: &mut dyn Read, buffer: &mut Vec<u8>) -> Result<(), BspParseError> {
    read.read_exact(buffer).map_err(BspParseError::GenericError)
}

pub fn extract_struct<T: bytemuck::Pod>(read: &mut dyn Read) -> Result<T, BspParseError> {
    let mut buffer = vec![0u8; std::mem::size_of::<T>()];
    extract(read, &mut buffer)?;
    let result: &T = from_bytes(&buffer);
    Ok(result.to_owned())
}

pub fn seek_ptr(read: &mut dyn Seek, ptr: &BspLumpPointer) -> Result<u64, BspParseError> {
    read.seek(io::SeekFrom::Start(
        ptr.n_offset
            .try_into()
            .map_err(BspParseError::BadPointerValue)?,
    ))
    .map_err(BspParseError::GenericError)
}
