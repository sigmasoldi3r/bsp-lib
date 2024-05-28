pub mod decoding;
pub mod entities;
pub mod geometry;
pub mod misc;
pub mod textures;

use std::io::{Read, Seek};

use bytemuck::from_bytes;

use crate::{
    bsp::Bsp,
    header::{
        BspHeader, LUMP_CLIPNODES, LUMP_EDGES, LUMP_ENTITIES, LUMP_FACES, LUMP_LEAVES,
        LUMP_LIGHTING, LUMP_MARKSURFACES, LUMP_MODELS, LUMP_NODES, LUMP_PLANES, LUMP_SURFEDGES,
        LUMP_TEXINFO, LUMP_TEXTURES, LUMP_VERTICES, LUMP_VISIBILITY,
    },
    lumps::{
        entities::BspEntitiesLump, nodes::BspNodesLump, planes::BspPlanesLump,
        textures::BspTexturesLump, vertices::BspVerticesLump, vis::BspVisLump,
    },
};
use decoding::*;

impl Bsp {
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

    pub fn parse<T>(read: &mut T) -> Result<Box<Self>, BspParseError>
    where
        T: Seek + Read,
    {
        let header = Self::extract_header(read)?;
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
