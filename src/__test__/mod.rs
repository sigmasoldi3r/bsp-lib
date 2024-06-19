mod relations;

use rstest::*;
use rstest_reuse::{self, *};
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
    parsing::decoding::{BspParseError, LumpExtractor},
};

#[template]
#[rstest]
#[case("maps/crossfire.bsp")]
#[case("maps/frenzy.bsp")]
#[case("maps/snark_pit.bsp")]
#[case("maps/stalkyard.bsp")]
#[case("maps/undertow.bsp")]
#[case("maps/c1a0.bsp")]
fn test_maps(#[case] map: &str) {}

#[apply(test_maps)]
fn test_entities(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let entities: Result<BspEntitiesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(entities.is_ok());
}

#[apply(test_maps)]
fn test_planes(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let planes: Result<BspPlanesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(planes.is_ok());
}

#[apply(test_maps)]
fn test_textures(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let textures: Result<BspTexturesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(textures.is_ok());
}

#[apply(test_maps)]
fn test_vertices(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let vertices: Result<BspVerticesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(vertices.is_ok());
}

#[apply(test_maps)]
fn test_vis(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let vis: Result<BspVisLump, BspParseError> = header.extract_lump(&mut file);
    assert!(vis.is_ok());
}

#[apply(test_maps)]
fn test_nodes(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let nodes: Result<BspNodesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(nodes.is_ok());
}

#[apply(test_maps)]
fn test_tex_info(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let tex_info: Result<BspTexInfoLump, BspParseError> = header.extract_lump(&mut file);
    assert!(tex_info.is_ok());
}

#[apply(test_maps)]
fn test_faces(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let faces: Result<BspFacesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(faces.is_ok());
}

#[apply(test_maps)]
fn test_light_map(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let light_map: Result<BspLightMapLump, BspParseError> = header.extract_lump(&mut file);
    assert!(light_map.is_ok());
}

#[apply(test_maps)]
fn test_clip_nodes(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let clip_nodes: Result<BspClipNodesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(clip_nodes.is_ok());
}

#[apply(test_maps)]
fn test_leaves(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let leaves: Result<BspLeavesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(leaves.is_ok());
}

#[apply(test_maps)]
fn test_mark_surfaces(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let mark_surfaces: Result<BspMarkSurfacesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(mark_surfaces.is_ok());
}

#[apply(test_maps)]
fn test_edges(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let edges: Result<BspEdgesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(edges.is_ok());
}

#[apply(test_maps)]
fn test_surf_edges(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let surf_edges: Result<BspSurfEdgesLump, BspParseError> = header.extract_lump(&mut file);
    assert!(surf_edges.is_ok());
}

#[apply(test_maps)]
fn test_models(#[case] map: &str) {
    let mut file = File::open(map).unwrap();
    let header = Bsp::extract_header(&mut file).unwrap();
    let models: Result<BspModelsLump, BspParseError> = header.extract_lump(&mut file);
    assert!(models.is_ok());
}
