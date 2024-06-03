use std::ops::Index;

use bytemuck::{Pod, Zeroable};

use crate::math::Vector3D;

#[derive(Clone, Copy, Debug, Zeroable, Pod)]
#[repr(C)]
pub struct BspPlaneType(pub i32);
pub const X: BspPlaneType = BspPlaneType(0);
pub const Y: BspPlaneType = BspPlaneType(1);
pub const Z: BspPlaneType = BspPlaneType(2);
pub const ANY_X: BspPlaneType = BspPlaneType(3);
pub const ANY_Y: BspPlaneType = BspPlaneType(4);
pub const ANY_Z: BspPlaneType = BspPlaneType(5);

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
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct BspPlane {
    pub v_normal: Vector3D,
    pub f_dist: f32,
    pub n_type: BspPlaneType,
}

#[derive(Debug)]
pub struct BspPlanesLump(pub Vec<BspPlane>);

impl Index<usize> for BspPlanesLump {
    type Output = BspPlane;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
