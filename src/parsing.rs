use std::{
    io::{self, Read, Seek},
    num::TryFromIntError, string::FromUtf8Error,
};

use bytemuck::from_bytes;
use peg::{error::ParseError, str::LineCol};

use crate::{
    bsp::Bsp,
    header::{
        BspHeader, BspLumpPointer, LUMP_CLIPNODES, LUMP_EDGES, LUMP_ENTITIES, LUMP_FACES,
        LUMP_LEAVES, LUMP_LIGHTING, LUMP_MARKSURFACES, LUMP_MODELS, LUMP_NODES, LUMP_PLANES,
        LUMP_SURFEDGES, LUMP_TEXINFO, LUMP_TEXTURES, LUMP_VERTICES, LUMP_VISIBILITY,
    },
    lumps::entities::{self, BspEntitiesLump, BspEntity},
};

#[derive(Debug)]
pub enum BspParseError {
    InvalidVersion { valid: Vec<i32>, found: i32 },
    BadPointerValue(TryFromIntError),
    BadStringValue(FromUtf8Error),
    EntityLumpParseError(ParseError<LineCol>),
    GenericError(io::Error),
}

pub fn extract(read: &mut dyn Read, buffer: &mut Vec<u8>) -> Result<(), BspParseError> {
    read.read_exact(buffer).map_err(BspParseError::GenericError)
}

pub fn seek_ptr(read: &mut dyn Seek, ptr: &BspLumpPointer) -> Result<u64, BspParseError> {
    read.seek(io::SeekFrom::Start(
        ptr.n_offset
            .try_into()
            .map_err(BspParseError::BadPointerValue)?,
    ))
    .map_err(BspParseError::GenericError)
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

impl BspEntitiesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
    {
        seek_ptr(read, ptr)?;
        let mut buffer = vec![
            0u8;
            (ptr.n_length-1)
                .try_into()
                .map_err(BspParseError::BadPointerValue)?
        ];
        extract(read, &mut buffer)?;
        let raw = String::from_utf8(buffer).map_err(BspParseError::BadStringValue)?;
        // if let Err(err) = entity_descriptor::entities(raw.as_str()).map_err(BspParseError::EntityLumpParseError) {
        //     if let BspParseError::EntityLumpParseError(err) = err {
        //         let o = err.location.offset;
        //         let len = raw.len();
        //         let s = &raw[0..1];
        //         let ex = err.expected;
        //         println!("\n{o}/{len}\n---\n{s}\n---\n{ex}\n---");
        //     }
        // }
        let entities = entity_descriptor::entities(raw.as_str()).map_err(BspParseError::EntityLumpParseError)?;
        Ok(BspEntitiesLump(entities))
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
        println!("{:?}", entities);
        todo!()
        // Ok(Box::new(Bsp {
        //     entities: BspEntitiesLump::read(&mut read, &entities_ptr)?,
        // }))
    }
}
