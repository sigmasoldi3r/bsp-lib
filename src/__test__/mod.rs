mod relations;

use std::fs::File;

use crate::{
    bsp::Bsp,
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
    parsing::decoding::LumpExtractor,
};

const TEST_MAP: &'static str = "maps/crossfire.bsp";

#[test]
fn test_entities() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let entities: BspEntitiesLump = header.extract_lump(&mut file).unwrap();
    println!("entities = {:?}", entities.0[0]);
}

#[test]
fn test_planes() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let planes: BspPlanesLump = header.extract_lump(&mut file).unwrap();
    println!("planes = {:?}", planes.0[0]);
}

#[test]
fn test_textures() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let textures: BspTexturesLump = header.extract_lump(&mut file).unwrap();
    println!("textures = {:?}", textures.0[0]);
}

#[test]
fn test_vertices() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let vertices: BspVerticesLump = header.extract_lump(&mut file).unwrap();
    println!("vertices = {:?}", vertices.0[0]);
}

#[test]
fn test_vis() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let vis: BspVisLump = header.extract_lump(&mut file).unwrap();
    println!("vis = {:?}", vis);
}

#[test]
fn test_nodes() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let nodes: BspNodesLump = header.extract_lump(&mut file).unwrap();
    println!("nodes = {:?}", nodes.0[0]);
}

#[test]
fn test_tex_info() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let tex_info: BspTexInfoLump = header.extract_lump(&mut file).unwrap();
    println!("tex_info = {:?}", tex_info.0[0]);
}

#[test]
fn test_faces() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let faces: BspFacesLump = header.extract_lump(&mut file).unwrap();
    println!("faces = {:?}", faces.0[0]);
}

#[test]
fn test_light_map() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let light_map: BspLightMapLump = header.extract_lump(&mut file).unwrap();
    println!("light_map = {:?}", light_map.0[0]);
}

#[test]
fn test_clip_nodes() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let clip_nodes: BspClipNodesLump = header.extract_lump(&mut file).unwrap();
    println!("clip_nodes = {:?}", clip_nodes.0[0]);
}

#[test]
fn test_leaves() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let leaves: BspLeavesLump = header.extract_lump(&mut file).unwrap();
    println!("leaves = {:?}", leaves.0[0]);
}

#[test]
fn test_mark_surfaces() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let mark_surfaces: BspMarkSurfacesLump = header.extract_lump(&mut file).unwrap();
    println!("mark_surfaces = {:?}", mark_surfaces.0[0]);
}

#[test]
fn test_edges() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let edges: BspEdgesLump = header.extract_lump(&mut file).unwrap();
    println!("edges = {:?}", edges.0[0]);
}

#[test]
fn test_surf_edges() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let surf_edges: BspSurfEdgesLump = header.extract_lump(&mut file).unwrap();
    println!("surf_edges = {:?}", surf_edges.0[0]);
}

#[test]
fn test_models() {
    let mut file = File::open(TEST_MAP).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let models: BspModelsLump = header.extract_lump(&mut file).unwrap();
    println!("models = {:?}", models.0[0]);
}
