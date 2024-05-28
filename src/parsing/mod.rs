pub mod decoding;
pub mod entities;
pub mod geometry;
pub mod misc;
pub mod textures;

use std::io::{Read, Seek};

use bytemuck::from_bytes;

use crate::{
    bsp::Bsp,
    header::BspHeader,
    lumps::{
        clip_nodes::BspClipNodesLump,
        entities::BspEntitiesLump,
        faces::BspFacesLump,
        leaves::BspLeavesLump,
        light_map::BspLightMapLump,
        models::BspModelsLump,
        nodes::BspNodesLump,
        planes::BspPlanesLump,
        surfaces::{BspEdgesLump, BspMarkSurfacesLump, BspSurfEdgesLump},
        tex_info::BspTexInfoLump,
        textures::BspTexturesLump,
        vertices::BspVerticesLump,
        vis::BspVisLump,
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

    /// # BSP Bulk Parsing
    ///
    /// Extracts the whole BSP to memory, you can extract granular data by using
    /// `BspHeader::extract_lump` and `Bsp::extract_header` manually.
    pub fn parse<T: Seek + Read>(read: &mut T) -> Result<Box<Self>, BspParseError> {
        let header = Self::extract_header(read)?;
        let entities: BspEntitiesLump = header.extract_lump(read)?;
        let planes: BspPlanesLump = header.extract_lump(read)?;
        let textures: BspTexturesLump = header.extract_lump(read)?;
        let vertices: BspVerticesLump = header.extract_lump(read)?;
        let vis: BspVisLump = header.extract_lump(read)?;
        let nodes: BspNodesLump = header.extract_lump(read)?;
        let tex_info: BspTexInfoLump = header.extract_lump(read)?;
        let faces: BspFacesLump = header.extract_lump(read)?;
        let light_map: BspLightMapLump = header.extract_lump(read)?;
        let clip_nodes: BspClipNodesLump = header.extract_lump(read)?;
        let leaves: BspLeavesLump = header.extract_lump(read)?;
        let mark_surfaces: BspMarkSurfacesLump = header.extract_lump(read)?;
        let edges: BspEdgesLump = header.extract_lump(read)?;
        let surf_edges: BspSurfEdgesLump = header.extract_lump(read)?;
        let models: BspModelsLump = header.extract_lump(read)?;
        Ok(Box::new(Bsp {
            entities,
            planes,
            textures,
            vertices,
            vis,
            nodes,
            tex_info,
            faces,
            light_map,
            clip_nodes,
            leaves,
            mark_surfaces,
            edges,
            surf_edges,
            models,
        }))
    }
}
