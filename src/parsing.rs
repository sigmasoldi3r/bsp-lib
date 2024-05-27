use crate::bsp::Bsp;


pub enum BspParseError {
  InvalidVersion {
    valid: Vec<i32>,
    found: i32
  }
}

pub fn parse(file: String) -> Result<Box<Bsp>, BspParseError> {

  
  todo!()
}
