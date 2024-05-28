use crate::lumps::{
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
};

/// # BSP file data
/// 
/// This structure is a PDO/DTO.
///
/// ## Versions
///
/// This table gives an overview of the different BSP versions used in some games 
/// based on the GoldSrc Engine:
///
/// - The 0.52 alpha build of Half-Life uses the version number 29, like Quake, 
///   but is much more similar to BSP30 than BSP29.
/// - GoldSrc normally uses BSP30, as described on this page.
/// - Half-Life: Blue Shift flips the positions of the plane and entity lumps. 
///   The version number remains the same.
///   - Tip: BSPFix can be used to losslessly convert between standard BSP30 and 
///     Blue Shift BSPs.
/// - Paranoia 2: Savior uses a modified version of BSP30, called BSP31. This 
///   format is deprecated, as BSP30 can be extended to support BSP31's features 
///   without breaking compatibility (using BSPX lumps).
/// - James Bond 007: Nightfire uses a modified version of BSP30, called BSP42.
///
/// > **Confirm:**
/// >
/// > Are the formats used by Counter-Strike Neo, Counter-Strike Nexon: Studio, 
/// > and Cry of Fear different from BSP30?
#[derive(Debug)]
pub struct Bsp {
  pub entities: BspEntitiesLump,
  pub planes: BspPlanesLump,
  pub textures: BspTexturesLump,
  pub vertices: BspVerticesLump,
  pub vis: BspVisLump,
  pub nodes: BspNodesLump,
  pub tex_info: BspTexInfoLump,
  pub faces: BspFacesLump,
  pub light_map: BspLightMapLump,
  pub clip_nodes: BspClipNodesLump,
  pub leaves: BspLeavesLump,
  pub mark_surfaces: BspMarkSurfacesLump,
  pub edges: BspEdgesLump,
  pub surf_edges: BspSurfEdgesLump,
  pub models: BspModelsLump,
}
