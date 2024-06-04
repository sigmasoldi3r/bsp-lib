use std::fs::File;

use crate::bsp::Bsp;

const TEST_MAP: &'static str = "maps/crossfire.bsp";

#[test]
fn test_face_relation_functions() {
    let mut file = File::open(TEST_MAP).unwrap();
    let bsp = Bsp::parse(&mut file).unwrap();
    let mut vertices_total = vec![];
    for face in &bsp.faces.0 {
        let vertices = face.vertices(&bsp);
        for vertex in vertices {
            vertices_total.push(vertex);
        }
    }
    println!("n = {}", vertices_total.len());
}
