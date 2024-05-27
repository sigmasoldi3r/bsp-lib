pub enum LumpType {
  Entities = 0,
  Planes = 1,
  Textures = 2,
  Vertices = 3,
  Visibility = 4,
  Nodes = 5,
  TexInfo = 6,
  Faces = 7,
  Lightning = 8,
  ClipNodes = 9,
  Leaves = 10,
  MarkSurfaces = 11,
  Edges = 12,
  SurfEdges = 13,
  Models = 14,
  Header = 15
}
pub const HEADER_LUMPS: usize = 15;

pub struct BspLumpPointer {
  n_offset: i32,
  n_length: i32
}

pub struct MapMaxValue(pub usize);
pub const MAX_MAP_HULLS: MapMaxValue = MapMaxValue(4);
pub const MAX_MAP_MODELS: MapMaxValue = MapMaxValue(400);
pub const MAX_MAP_BRUSHES: MapMaxValue = MapMaxValue(4096);
pub const MAX_MAP_ENTITIES: MapMaxValue = MapMaxValue(1024);
pub const MAX_MAP_ENTSTRING: MapMaxValue = MapMaxValue(128*1024);
pub const MAX_MAP_PLANES: MapMaxValue = MapMaxValue(32767);
pub const MAX_MAP_NODES: MapMaxValue = MapMaxValue(32767);
pub const MAX_MAP_CLIPNODES: MapMaxValue = MapMaxValue(32767);
pub const MAX_MAP_LEAFS: MapMaxValue = MapMaxValue(8192);
pub const MAX_MAP_VERTS: MapMaxValue = MapMaxValue(65535);
pub const MAX_MAP_FACES: MapMaxValue = MapMaxValue(65535);
pub const MAX_MAP_MARKSURFACES: MapMaxValue = MapMaxValue(65535);
pub const MAX_MAP_TEXINFO: MapMaxValue = MapMaxValue(8192);
pub const MAX_MAP_EDGES: MapMaxValue = MapMaxValue(256000);
pub const MAX_MAP_SURFEDGES: MapMaxValue = MapMaxValue(512000);
pub const MAX_MAP_TEXTURES: MapMaxValue = MapMaxValue(512);
pub const MAX_MAP_MIPTEX: MapMaxValue = MapMaxValue(0x200000);
pub const MAX_MAP_LIGHTING: MapMaxValue = MapMaxValue(0x200000);
pub const MAX_MAP_VISIBILITY: MapMaxValue = MapMaxValue(0x200000);

/// # Header
///
/// Like almost every file, a BSP file also starts with a specific file header 
/// which is constructed as follows:
///
/// ```c
/// #define LUMP_ENTITIES      0
/// #define LUMP_PLANES        1
/// #define LUMP_TEXTURES      2
/// #define LUMP_VERTICES      3
/// #define LUMP_VISIBILITY    4
/// #define LUMP_NODES         5
/// #define LUMP_TEXINFO       6
/// #define LUMP_FACES         7
/// #define LUMP_LIGHTING      8
/// #define LUMP_CLIPNODES     9
/// #define LUMP_LEAVES       10
/// #define LUMP_MARKSURFACES 11
/// #define LUMP_EDGES        12
/// #define LUMP_SURFEDGES    13
/// #define LUMP_MODELS       14
/// #define HEADER_LUMPS      15
/// ```
///
/// ```c
/// typedef struct _BSPHEADER {
///     int32_t nVersion;           // Must be 30 for a valid HL BSP file
///     BSPLUMP lump[HEADER_LUMPS]; // Stores the directory of lumps
/// } BSPHEADER;
/// ```
///
/// The file header begins with a 32-bit integer containing the file version of 
/// the BSP file (the magic number). This should be 30 for a valid BSP file used 
/// by the Half-Life Engine. Subsequently, there is an array of entries for the 
/// so-called lumps. A lump is more or less a section of the file containing a 
/// specific type of data. The lump entries in the file header address these 
/// lumps, accessed by the 15 predefined indexes. A lump entry struct is defined 
/// as follows:
///
/// ```c
/// typedef struct _BSPLUMP {
///     int32_t nOffset; // File offset to data
///     int32_t nLength; // Length of data
/// } BSPLUMP;
/// ```
///
/// To read the different lumps from the given BSP file, every lump entry file 
/// states the beginning of each lump as an offset relative to the beginning of 
/// the file. Additionally, the lump entry also gives the length of the 
/// addressed lump in bytes. The Half-Life BSP compilers also define several 
/// constants for the maximum size of each lump, as they use static, global 
/// arrays to hold the data. The hlbsp project uses malloc() to allocate the 
/// required memory for each lump depending on their actual size.
///
/// ```c
/// #define MAX_MAP_HULLS        4
///
/// #define MAX_MAP_MODELS       400
/// #define MAX_MAP_BRUSHES      4096
/// #define MAX_MAP_ENTITIES     1024
/// #define MAX_MAP_ENTSTRING    (128*1024)
///
/// #define MAX_MAP_PLANES       32767
/// #define MAX_MAP_NODES        32767
/// #define MAX_MAP_CLIPNODES    32767
/// #define MAX_MAP_LEAFS        8192
/// #define MAX_MAP_VERTS        65535
/// #define MAX_MAP_FACES        65535
/// #define MAX_MAP_MARKSURFACES 65535
/// #define MAX_MAP_TEXINFO      8192
/// #define MAX_MAP_EDGES        256000
/// #define MAX_MAP_SURFEDGES    512000
/// #define MAX_MAP_TEXTURES     512
/// #define MAX_MAP_MIPTEX       0x200000
/// #define MAX_MAP_LIGHTING     0x200000
/// #define MAX_MAP_VISIBILITY   0x200000
///
/// #define MAX_MAP_PORTALS     65536
/// ```
pub struct BspHeader {
  pub n_version: i32,
  pub lump: [BspLumpPointer; HEADER_LUMPS]
}
