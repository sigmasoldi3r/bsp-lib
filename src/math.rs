use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Vector3D {
  pub x: f32,
  pub y: f32,
  pub z: f32
}
