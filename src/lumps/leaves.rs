pub struct BspLeafContent(pub i32);
pub const CONTENTS_EMPTY: BspLeafContent = BspLeafContent(-1);
pub const CONTENTS_SOLID: BspLeafContent = BspLeafContent(-2);
pub const CONTENTS_WATER: BspLeafContent = BspLeafContent(-3);
pub const CONTENTS_SLIME: BspLeafContent = BspLeafContent(-4);
pub const CONTENTS_LAVA: BspLeafContent = BspLeafContent(-5);
pub const CONTENTS_SKY: BspLeafContent = BspLeafContent(-6);
pub const CONTENTS_ORIGIN: BspLeafContent = BspLeafContent(-7);
pub const CONTENTS_CLIP: BspLeafContent = BspLeafContent(-8);
pub const CONTENTS_CURRENT_0: BspLeafContent = BspLeafContent(-9);
pub const CONTENTS_CURRENT_90: BspLeafContent = BspLeafContent(-10);
pub const CONTENTS_CURRENT_180: BspLeafContent = BspLeafContent(-11);
pub const CONTENTS_CURRENT_270: BspLeafContent = BspLeafContent(-12);
pub const CONTENTS_CURRENT_UP: BspLeafContent = BspLeafContent(-13);
pub const CONTENTS_CURRENT_DOWN: BspLeafContent = BspLeafContent(-14);
pub const CONTENTS_TRANSLUCENT: BspLeafContent = BspLeafContent(-15);

pub struct BspLeaf {
  pub n_contents: BspLeafContent,
  pub n_vis_offset: i32,
  pub n_mins: [i16; 3],
  pub n_maxs: [i16; 3],
  pub i_fist_mark_surface: u16,
  pub n_mark_surfaces: u16,
  pub n_ambient_levels: [u8; 4]
}

/// # Leaves
///
/// The leaves lump contains the leaves of the BSP tree. Another array of binary 
/// structs:
///
/// ```c
/// #define CONTENTS_EMPTY        -1
/// #define CONTENTS_SOLID        -2
/// #define CONTENTS_WATER        -3
/// #define CONTENTS_SLIME        -4
/// #define CONTENTS_LAVA         -5
/// #define CONTENTS_SKY          -6
/// #define CONTENTS_ORIGIN       -7
/// #define CONTENTS_CLIP         -8
/// #define CONTENTS_CURRENT_0    -9
/// #define CONTENTS_CURRENT_90   -10
/// #define CONTENTS_CURRENT_180  -11
/// #define CONTENTS_CURRENT_270  -12
/// #define CONTENTS_CURRENT_UP   -13
/// #define CONTENTS_CURRENT_DOWN -14
/// #define CONTENTS_TRANSLUCENT  -15
/// ```
///
/// ```c
/// typedef struct _BSPLEAF {
///     int32_t nContents;                         // Contents enumeration
///     int32_t nVisOffset;                        // Offset into visibility lump
///     int16_t nMins[3], nMaxs[3];                // Defines bounding box
///     uint16_t iFirstMarkSurface, nMarkSurfaces; // Index and count into 
///                                                // marksurfaces array
///     uint8_t nAmbientLevels[4];                 // Ambient sound levels
/// } BSPLEAF;
/// ```
///
/// The first entry of this struct is the type of the content of this leaf. It 
/// can be one of the predefined values, found in the compiler source codes, and 
/// is little relevant for the actual rendering process. All the more important 
/// is the next integer containing the offset into the vis lump. It defines the 
/// start of the raw PVS data for this leaf. If this value equals -1, no VIS 
/// lists are available for this leaf, usually if the map has been built without 
/// the VIS compiler. The next two 16-bit integer triples span the bounding box 
/// of this leaf. Furthermore, the struct contains an index pointing into the 
/// array of marksurfaces loaded from the marksurfaces lump as well as the 
/// number of consecutive marksurfaces belonging to this leaf. The marksurfaces 
/// are looped through during the rendering process and point to the actual 
/// faces. The final 4 bytes specify the volume of ambient sounds in Quake, but 
/// are unused in GoldSrc.
pub struct BspLeavesLump(pub Vec<BspLeaf>);
