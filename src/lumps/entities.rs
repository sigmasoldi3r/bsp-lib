#[derive(Debug)]
pub struct BspEntity(pub Vec<(String, String)>);

pub const MAX_KEY: usize = 32;
pub const MAX_VALUE: usize = 1024;

/// # Entities
///
/// The entity lump is basically a pure ASCII text section. It consists of the 
/// string representations of all entities, which are copied directly from the 
/// input file to the output BSP file by the compiler. An entity might look like 
/// this:
///
/// ```text
/// {
/// "origin" "0 0 -64"
/// "angles" "0 0 0"
/// "classname" "info_player_start"
/// }
/// ```
///
/// Every entity begins and ends with curly brackets. In between there are the 
/// attributes of the entity, one in each line, which are pairs of strings 
/// enclosed by quotes. The first string is the name of the attribute (the key), 
/// the second one its value. The attribute "classname" is mandatory for every 
/// entity specifying its type and therefore, how it is interpreted by the 
/// engine.
///
/// The map compilers also define two constants for the maximum length of key and 
/// value:
///
/// ```c
/// #define MAX_KEY     32
/// #define MAX_VALUE   1024
/// ```
#[derive(Debug)]
pub struct BspEntitiesLump(pub Vec<BspEntity>);
