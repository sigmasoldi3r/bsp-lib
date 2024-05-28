use std::io::{Read, Seek};

use crate::{
    header::{BspHeader, BspLumpPointer, LUMP_ENTITIES},
    lumps::entities::{BspEntitiesLump, BspEntity},
};

use super::{seek_and_extract, BspParseError, LumpExtractor, PtrLumpReader};

peg::parser! {
  grammar entity_descriptor() for str {
      pub rule entities() -> Vec<BspEntity>
          = e:entity()+ _
          { e }

      pub rule entity() -> BspEntity
          = _ "{" _ kv:kv_pairs() "}" _
          { BspEntity(kv) }

      rule kv_pairs() -> Vec<(String, String)>
          = kv_pair()*

      rule kv_pair() -> (String, String)
          =  "\"" key:string() "\"" _ "\"" value:string() "\"" _
          { (key.into(), value.into()) }

      rule string() -> String
          = value:$([^'"']+) { value.into() }

      rule _() = [' '|'\t'|'\r'|'\n']*
  }
}

impl PtrLumpReader for BspEntitiesLump {
    fn read_from_ptr<T>(read: &mut T, ptr: &BspLumpPointer) -> Result<Self, BspParseError>
    where
        T: Seek + Read,
    {
        let buffer = seek_and_extract(read, ptr)?;
        let raw = String::from_utf8(buffer).map_err(BspParseError::BadStringValue)?;
        let raw = &raw[0..raw.len() - 1];
        let entities =
            entity_descriptor::entities(raw).map_err(BspParseError::EntityLumpParseError)?;
        Ok(BspEntitiesLump(entities))
    }
}
impl LumpExtractor<BspEntitiesLump> for BspHeader {
    fn get_pointer(&self) -> BspLumpPointer {
        self.lump[LUMP_ENTITIES.0]
    }
}
