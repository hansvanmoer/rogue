use serde::{Deserialize, Serialize};
use crate::immutable_state::ImmutableState;
use crate::metrics::NonZeroDimensions3;
use crate::resource::{Error as ResourceError};
use crate::texture::TextureSet;
use crate::tile_set::{TileSet};
use crate::validation::{non_empty_string, validate_field, validation_failed, Error as ValidationError, ValidateOwned};

///
/// The local map as loaded in memory
///
pub struct LocalMap<'a> {
    ///
    /// The map bounds
    ///
    bounds: Bounds,

    ///
    /// The tile set
    ///
    tile_set: &'a TileSet,

    ///
    /// The texture set for the tiles
    ///
    texture_set: &'a TextureSet<'a>,

    ///
    /// The tiles
    ///
    tiles: Vec<Tile>,
}

impl<'a> LocalMap<'a> {
    ///
    /// Creates a new local map using the first (presumably empty) tile in the tile set as the base tile
    ///
    pub fn new(size: &NonZeroDimensions3<i32>, tile_set_name: &str, state: &'a ImmutableState) -> Result<LocalMap<'a>, Error> {
        let bounds = Bounds::new(size);
        let tile_set = state.tile_sets().get_required_by_name(tile_set_name).map_err(|e| Error::TileSetNotFound(e))?;
        let texture_set = state.texture_sets().get_required_by_name(tile_set_name).map_err(|e| Error::TextureSetNotFound(e))?;

        // note that tile sets can not be constructed empty, so there is always a tile with index 0
        let tiles= vec![Tile {type_index: 0}; bounds.index_bound()];
        Ok(LocalMap {
            bounds,
            tile_set,
            texture_set,
            tiles,
        })
    }

    ///
    /// Creates a new local map
    ///
    pub fn from_data(data: &LocalMapData, state: &'a ImmutableState) -> Result<LocalMap<'a>, Error>{
        let bounds = Bounds::new(&data.size);
        let tile_set = state.tile_sets().get_required_by_name(&data.tile_set).map_err(|e| Error::TileSetNotFound(e))?;
        let texture_set = state.texture_sets().get_required_by_name(&data.tile_set).map_err(|e| Error::TextureSetNotFound(e))?;
        let tiles = Self::validate_tiles(&data.tiles, &bounds, tile_set)?;
        Ok(LocalMap {
            bounds,
            tile_set,
            texture_set,
            tiles,
        })
    }

    ///
    /// Validates the tiles
    ///
    fn validate_tiles(input: &Vec<usize>, bounds: &Bounds, tile_set: &TileSet) -> Result<Vec<Tile>, Error> {
        let mut output = Vec::with_capacity(input.len());
        for index in 0..input.len() {
            let tile = input[index];
            if tile_set.is_valid_index(tile) {
                output.push(Tile {
                    type_index: tile,
                });
            } else {
                let (x, y, z) = bounds.to_map_coords(index);
                return Err(Error::InvalidTile(x, y, z))
            }
        }
        Ok(output)
    }
}

///
/// The bounds of the local map
///
struct Bounds {
    ///
    /// The x bound coordinate
    ///
    x: usize,

    ///
    /// The y bound coordinate
    ///
    y: usize,

    ///
    /// The z bound coordinate
    ///
    z: usize,
}

impl Bounds {

    ///
    /// Constructs a new bounds instance
    ///
    pub fn new(size: &NonZeroDimensions3<i32>) -> Bounds {
        Bounds {
            x: size.get_width().clone().try_into().unwrap(),
            y: size.get_height().clone().try_into().unwrap(),
            z: size.get_depth().clone().try_into().unwrap(),
        }
    }

    ///
    /// Converts map coordinates to tile array index
    ///
    pub fn to_tile_index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.x + z * (self.x * self.y)
    }

    ///
    /// Converts an index to map coordinates
    ///
    pub fn to_map_coords(&self, index: usize) -> (usize, usize, usize) {
        (index % self.x, (index / self.x) % self.y, index / (self.x * self.y))
    }

    ///
    /// Returns the upper bound of the tile array
    ///
    pub fn index_bound(&self) -> usize {
        self.x * self.y * self.z
    }
}

///
/// A tile
///
#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
    ///
    /// The tile type index
    ///
    type_index: usize,
}

///
/// The view of the local map
///
pub struct View {
    x: f32,
    y: f32,
    z: usize,
    zoom: f32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0,
            zoom: 1.0,
        }
    }
}

///
/// Errors while loading and using the local map
///
pub enum Error {
    ///
    /// The tile set was not found
    ///
    TileSetNotFound(ResourceError),

    ///
    /// The tile texture set was not found
    ///
    TextureSetNotFound(ResourceError),

    ///
    /// An invalid tile
    ///
    InvalidTile(usize, usize, usize),
}

///
/// Opaque map state that contains no references to the rest of the game state
/// To be used to store and load game data
///
#[derive(Debug, PartialEq)]
pub struct LocalMapData {
    ///
    /// The size
    ///
    size: NonZeroDimensions3<i32>,

    ///
    /// The tile set ID
    ///
    tile_set: String,

    ///
    /// The tiles
    ///
    tiles: Vec<usize>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct LocalMapConfig {
    ///
    /// The size
    ///
    size: LocalMapSizeConfig,

    ///
    /// The tile set ID
    ///
    tile_set: String,

    ///
    /// The tiles
    ///
    tiles: Vec<usize>,
}

impl ValidateOwned for LocalMapConfig {
    type Output = LocalMapData;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        let size = validate_field("size", self.size.validate_owned())?;
        let tiles = validate_field("tiles", Self::validate_tiles(&self.tiles, &size))?;
        Ok(LocalMapData {
            size,
            tile_set: validate_field("tile_set", non_empty_string(&self.tile_set))?,
            tiles,
        })
    }
}

impl LocalMapConfig {
    ///
    /// Validates the tiles
    ///
    fn validate_tiles(tiles: &Vec<usize>, size: &NonZeroDimensions3<i32>) -> Result<Vec<usize>, ValidationError> {
        if tiles.len() == (size.get_width() * size.get_height() * size.get_depth()).try_into().expect("should not be negative due to type invariant") {
            Ok(tiles.clone())
        } else {
            validation_failed("invalid number of tiles")
        }
    }
}

///
/// Local map size configuration model
///
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct LocalMapSizeConfig {
    ///
    /// The local map width
    ///
    width: i32,

    ///
    /// The local map height
    ///
    height: i32,

    ///
    /// The local map depth
    ///
    depth: i32,
}

impl ValidateOwned for LocalMapSizeConfig {
    type Output = NonZeroDimensions3<i32>;

    fn validate_owned(&self) -> Result<NonZeroDimensions3<i32>, crate::validation::Error> {
        NonZeroDimensions3::new(self.width, self.height, self.depth)
            .map_err(|_| crate::validation::Error::from_str("invalid map size"))
    }
}