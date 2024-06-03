use std::fs::File;

use crate::{bsp::Bsp, lumps::vertices};


const TEST_MAP: &'static str = "maps/crossfire.bsp";

#[test]
fn test_face_relation_functions() {
  let mut file = File::open(TEST_MAP).unwrap();
  let bsp = Bsp::parse(&mut file).unwrap();
  for face in &bsp.faces.0 {
    if let Some(edges) = face.edges(&bsp) {
      for edge in edges {
        let _ = edge.vertices(&bsp);
      }
    }
  }
}
