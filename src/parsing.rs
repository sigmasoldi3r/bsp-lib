use std::{
    io::{self, Read, Seek},
    num::TryFromIntError,
    string::FromUtf8Error,
};

use bytemuck::{from_bytes, try_cast_slice, PodCastError};
use peg::{error::ParseError, str::LineCol};

use crate::{
    bsp::Bsp,
    header::{
        BspHeader, BspLumpPointer, LUMP_CLIPNODES, LUMP_EDGES, LUMP_ENTITIES, LUMP_FACES,
        LUMP_LEAVES, LUMP_LIGHTING, LUMP_MARKSURFACES, LUMP_MODELS, LUMP_NODES, LUMP_PLANES,
        LUMP_SURFEDGES, LUMP_TEXINFO, LUMP_TEXTURES, LUMP_VERTICES, LUMP_VISIBILITY,
    },
    lumps::{
        entities::{BspEntitiesLump, BspEntity},
        nodes::BspNodesLump,
        planes::{BspPlane, BspPlanesLump},
        textures::{BspMipTex, BspMipTexOffset, BspTextureHeader, BspTexturesLump},
        vertices::BspVerticesLump,
        vis::BspVisLump,
    },
};

#[derive(Debug)]
pub enum BspParseError {
    InvalidVersion { valid: Vec<i32>, found: i32 },
    BadPointerValue(TryFromIntError),
    BadStringValue(FromUtf8Error),
    EntityLumpParseError(ParseError<LineCol>),
    GenericError(io::Error),
    DeserializationError(PodCastError),
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

pub fn seek_and_extract<T: Read + Seek>(
    read: &mut T,
    ptr: &BspLumpPointer,
) -> Result<Vec<u8>, BspParseError> {
    seek_ptr(read, ptr)?;
    let mut buffer = vec![
        0u8;
        (ptr.n_length - 1)
            .try_into()
            .map_err(BspParseError::BadPointerValue)?
    ];
    extract(read, &mut buffer)?;
    Ok(buffer)
}

/// Extracts an owned instance of the BSP header.
pub fn extract_header(read: &mut dyn Read) -> Result<BspHeader, BspParseError> {
    let mut buffer = vec![0u8; std::mem::size_of::<BspHeader>()];
    extract(read, &mut buffer)?;
    let data: &BspHeader = from_bytes(&buffer);
    if data.n_version != 30 {
        return Err(BspParseError::InvalidVersion {
            valid: vec![30],
            found: data.n_version,
        });
    }
    Ok(data.clone())
}

peg::parser! {
    grammar entity_descriptor() for str {
        pub rule entities() -> Vec<BspEntity>
            = e:entity()+ _
            { e }

        pub rule entity() -> BspEntity
            = _ "{" _ kv:kv_pairs() "}" _
            { BspEntity(kv) }

        rule kv_pairs() -> Vec<(String, String)>
            = kv_pair()*

        rule kv_pair() -> (String, String)
            =  "\"" key:string() "\"" _ "\"" value:string() "\"" _
            { (key.into(), value.into()) }

        rule string() -> String
            = value:$([^'"']+) { value.into() }

        rule _() = [' '|'\t'|'\r'|'\n']*
    }
}

trait PtrLumpReader {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized;
}

impl PtrLumpReader for BspEntitiesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let raw = String::from_utf8(buffer).map_err(BspParseError::BadStringValue)?;
        let entities = entity_descriptor::entities(raw.as_str())
            .map_err(BspParseError::EntityLumpParseError)?;
        Ok(BspEntitiesLump(entities))
    }
}

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
        for i in 0..header.n_mip_textures {
            let value: BspMipTexOffset = extract_struct(read)?;
            mip_tex_offsets.push(value);
        }
        let mut textures: Vec<BspMipTex> = Vec::with_capacity(count);
        for index in mip_tex_offsets {
            let offset = index.0 + ptr.n_offset;
            read.seek(io::SeekFrom::Start(
                offset.try_into().map_err(BspParseError::BadPointerValue)?,
            ))
            .map_err(BspParseError::GenericError)?;
            let mip: BspMipTex = extract_struct(read)?;
            textures.push(mip);
        }
        Ok(BspTexturesLump(textures))
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

impl PtrLumpReader for BspVisLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        Ok(BspVisLump(vec![]))
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

impl Bsp {
    pub fn parse<T>(read: &mut T) -> Result<Box<Self>, BspParseError>
    where
        T: Seek + Read,
    {
        let header = extract_header(read)?;
        let entities_ptr = header.lump[LUMP_ENTITIES.0];
        let planes_ptr = header.lump[LUMP_PLANES.0];
        let textures_ptr = header.lump[LUMP_TEXTURES.0];
        let vertices_ptr = header.lump[LUMP_VERTICES.0];
        let visibility_ptr = header.lump[LUMP_VISIBILITY.0];
        let nodes_ptr = header.lump[LUMP_NODES.0];
        let texinfo_ptr = header.lump[LUMP_TEXINFO.0];
        let faces_ptr = header.lump[LUMP_FACES.0];
        let lighting_ptr = header.lump[LUMP_LIGHTING.0];
        let clipnodes_ptr = header.lump[LUMP_CLIPNODES.0];
        let leaves_ptr = header.lump[LUMP_LEAVES.0];
        let marksurfaces_ptr = header.lump[LUMP_MARKSURFACES.0];
        let edges_ptr = header.lump[LUMP_EDGES.0];
        let surfedges_ptr = header.lump[LUMP_SURFEDGES.0];
        let models_ptr = header.lump[LUMP_MODELS.0];
        let entities = BspEntitiesLump::read_from_ptr(read, &entities_ptr)?;
        let planes = BspPlanesLump::read_from_ptr(read, &planes_ptr)?;
        let textures = BspTexturesLump::read_from_ptr(read, &textures_ptr)?;
        let vertices = BspVerticesLump::read_from_ptr(read, &vertices_ptr)?;
        let vis = BspVisLump::read_from_ptr(read, &visibility_ptr)?;
        println!("{:?}", textures);
        let nodes = BspNodesLump::read_from_ptr(read, &nodes_ptr)?;
        todo!()
        // Ok(Box::new(Bsp {
        //     entities: BspEntitiesLump::read(&mut read, &entities_ptr)?,
        // }))
    }
}
