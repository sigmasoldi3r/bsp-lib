use crate::math::Vector3D;

pub enum BspPlaneType {
  X = 0,
  Y = 1,
  Z = 2,
  AnyX = 3,
  AnyY = 4,
  AnyZ = 5
}

/// **Each of this structures defines
/// a plane in 3-dimensional space by using
/// the Hesse normal form:** `normal * point - distance = 0`
///
/// Where vNormal is the normalized normal vector of the
/// plane and fDist is the distance of the plane to the
/// origin of the coord system. Additionally, the structure
/// also saves an integer describing the orientation of the
/// plane in space. If nType equals PLANE_X, then the normal
/// of the plane will be parallel to the x axis, meaning the
/// plane is perpendicular to the x axis. If nType equals
/// PLANE_ANYX, then the plane's normal is nearer to the x axis
/// then to any other axis. This information is used by the
/// renderer to speed up some computations. 
pub struct BspPlane {
  pub v_normal: Vector3D,
  pub f_dist: f32,
  pub n_type: BspPlaneType
}

pub struct BspPlanesLump(pub Vec<BspPlane>);
