use std::fs::File;

use crate::bsp::Bsp;

#[test]
fn decode_basic() {
    let mut file = File::open("maps/crossfire.bsp").unwrap();
    let bsp = Bsp::parse(&mut file).unwrap();
    println!("Done!")
}
