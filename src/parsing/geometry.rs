use std::io::{Read, Seek};

use bytemuck::try_cast_slice;

use crate::{
    header::{
        BspHeader, BspLumpPointer, LUMP_CLIPNODES, LUMP_EDGES, LUMP_FACES, LUMP_LEAVES,
        LUMP_MARKSURFACES, LUMP_NODES, LUMP_PLANES, LUMP_SURFEDGES, LUMP_VERTICES,
    },
    lumps::{
        clip_nodes::{BspClipNode, BspClipNodesLump},
        faces::{BspFace, BspFacesLump},
        leaves::{BspLeaf, BspLeavesLump},
        nodes::{BspNode, BspNodesLump},
        planes::{BspPlane, BspPlanesLump},
        surfaces::{
            BspEdge, BspEdgesLump, BspMarkSurface, BspMarkSurfacesLump, BspSurfEdge,
            BspSurfEdgesLump,
        },
        vertices::{BspVertex, BspVerticesLump},
    },
};

use super::{seek_and_extract, BspParseError, LumpExtractor, PtrLumpReader};

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
impl LumpExtractor<BspPlanesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_PLANES.0]
    }
}

impl PtrLumpReader for BspVerticesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let vertices: &[BspVertex] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspVerticesLump(vertices.to_owned()))
    }
}
impl LumpExtractor<BspVerticesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_VERTICES.0]
    }
}

impl PtrLumpReader for BspNodesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let nodes: &[BspNode] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspNodesLump(nodes.to_owned()))
    }
}
impl LumpExtractor<BspNodesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_NODES.0]
    }
}

impl PtrLumpReader for BspFacesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let faces: &[BspFace] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspFacesLump(faces.to_owned()))
    }
}
impl LumpExtractor<BspFacesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_FACES.0]
    }
}

impl PtrLumpReader for BspClipNodesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let faces: &[BspClipNode] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspClipNodesLump(faces.to_owned()))
    }
}
impl LumpExtractor<BspClipNodesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_CLIPNODES.0]
    }
}

impl PtrLumpReader for BspLeavesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let leaves: &[BspLeaf] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspLeavesLump(leaves.to_owned()))
    }
}
impl LumpExtractor<BspLeavesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_LEAVES.0]
    }
}

impl PtrLumpReader for BspMarkSurfacesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let mark_surfaces: &[BspMarkSurface] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspMarkSurfacesLump(mark_surfaces.to_owned()))
    }
}
impl LumpExtractor<BspMarkSurfacesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_MARKSURFACES.0]
    }
}

impl PtrLumpReader for BspEdgesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let edges: &[BspEdge] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspEdgesLump(edges.to_owned()))
    }
}
impl LumpExtractor<BspEdgesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_EDGES.0]
    }
}

impl PtrLumpReader for BspSurfEdgesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
        Self: Sized,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let faces: &[BspSurfEdge] =
            try_cast_slice(&buffer).map_err(BspParseError::DeserializationError)?;
        Ok(BspSurfEdgesLump(faces.to_owned()))
    }
}
impl LumpExtractor<BspSurfEdgesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_SURFEDGES.0]
    }
}
