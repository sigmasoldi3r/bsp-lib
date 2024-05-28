use std::fs::File;

use crate::bsp::Bsp;


const TEST_MAP: &'static str = "maps/crossfire.bsp";

#[test]
fn test_face_relation_functions() {
  let mut file = File::open(TEST_MAP).unwrap();
  let bsp = Bsp::parse(&mut file).unwrap();
  let edges = bsp.faces.0[0].edges(&bsp);
  println!("Edges = {:?}", edges);
  let vertices = edges[1].vertices(&bsp);
  println!("Edges[0]/Vertices = {:?}", vertices);
}
