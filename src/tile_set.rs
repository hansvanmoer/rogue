use std::path::PathBuf;
use serde::Deserialize;
use crate::configuration::{Error as ConfigurationError, load_configuration};
use crate::resource::{Error as ResourceError, ResourceId, ResourceMap};
use crate::texture::{Error as TextureError, SubTextureIndex, TextureSet};
use crate::validation::{Error as ValidationError, non_empty_string, validate_field, validate_vec_field, ValidateOwned};

///
/// A tile set
///
pub struct TileSet {
    ///
    /// The desciptor
    ///
    descriptor: TileSetDescriptor,

    ///
    /// The unpacked tiles for easy lookup
    ///
    tiles: Vec<Tile>,

    ///
    /// The texture set ID for this tile set
    ///
    texture_set_id: ResourceId,
}

///
/// A tile
///
pub struct Tile {
    ///
    /// The texture index associated with this tile
    ///
    texture_index: SubTextureIndex,
    ///
    /// This is a land tile
    ///
    land: bool,

    ///
    /// This is a water tile
    ///
    water: bool,
}

impl TileSet {

    ///
    /// Loads all tile sets in a folder
    ///
    pub fn from_folder_path<'a>(path: &mut PathBuf, texture_sets: &ResourceMap<TextureSet<'a>>) -> Result<ResourceMap<TileSet>, Error> {
        path.push("tile_sets.yaml");
        let config = load_configuration::<&mut PathBuf, TileSetsConfig>(path)?;
        path.pop();
        let descriptor = config.validate_owned()?;
        let mut tile_sets = ResourceMap::new();
        for tile_set_name in descriptor.tile_sets.iter() {
            path.push(tile_set_name);
            path.set_extension("yaml");
            tile_sets.insert(tile_set_name.to_string(), Self::from_descriptor_path(path, texture_sets)?);
            path.pop();
        }
        Ok(tile_sets)
    }

    ///
    /// Constructs a new tile set
    ///
    fn from_descriptor_path<'a>(path: &mut PathBuf, texture_sets: &ResourceMap<TextureSet<'a>>) -> Result<TileSet, Error>{
        let config: TileSetConfig = load_configuration(path)?;
        let descriptor = config.validate_owned()?;
        let texture_set_id = texture_sets.get_required_id_by_name(&descriptor.texture_set)?;
        let tiles = Self::unpack_tiles(&descriptor, texture_sets.get_required_by_id(texture_set_id)?)?;
        Ok(TileSet{
            descriptor,
            tiles,
            texture_set_id,
        })
    }

    ///
    /// Unpacks the tiles for easy indexing
    ///
    fn unpack_tiles(descriptor: &TileSetDescriptor, texture_set: &TextureSet) -> Result<Vec<Tile>, Error> {
        let mut tiles = Vec::new();
        for tile in descriptor.tiles.iter() {
            for variant in tile.variants.iter() {
                let texture_index = texture_set.get_index(&variant.texture)?;
                tiles.push(Tile {
                    texture_index,
                    land: tile.land,
                    water: tile.water,
                })
            }
        }
        Ok(tiles)
    }
}

///
/// A tile set descriptor
///
#[derive(Debug, PartialEq)]
struct TileSetDescriptor {
    ///
    /// The tile set name
    ///
    name: String,

    ///
    /// The texture set ID for the tiles
    ///
    texture_set: String,

    ///
    /// The tiles
    ///
    tiles: Vec<TileDescriptor>,
}

///
/// A tile descriptor
///
#[derive(Debug, PartialEq)]
struct TileDescriptor {
    ///
    /// The tile name
    ///
    name: String,

    ///
    /// This is a land tile
    ///
    land: bool,

    ///
    /// This is a water tile
    ///
    water: bool,

    ///
    /// Variants
    ///
    variants: Vec<TileVariantDescriptor>,
}

///
/// A tile variant descriptor
///
#[derive(Debug, PartialEq)]
struct TileVariantDescriptor {
    ///
    /// The texture name
    ///
    texture: String,
}

///
/// A tile set descriptor
///
#[derive(Debug, Deserialize, PartialEq)]
struct TileSetConfig {
    ///
    /// The tile set name
    ///
    name: String,

    ///
    /// The texture set ID for the tiles
    ///
    texture_set: String,

    ///
    /// The tiles
    ///
    tiles: Vec<TileConfig>,
}

impl ValidateOwned for TileSetConfig {
    type Output = TileSetDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TileSetDescriptor {
            name: validate_field("name", non_empty_string(&self.name))?,
            texture_set: validate_field("texture_set", non_empty_string(&self.texture_set))?,
            tiles: validate_vec_field("tiles", &self.tiles, |t| t.validate_owned())?,
        })
    }
}

///
/// A tile descriptor
///
#[derive(Debug, Deserialize, PartialEq)]
struct TileConfig {
    ///
    /// The tile name
    ///
    name: String,

    ///
    /// Whether this tile is a land tile
    ///
    land: Option<bool>,

    ///
    /// Whether this tile is a water tile
    ///
    water: Option<bool>,

    ///
    /// Variants
    ///
    variants: Vec<TileVariantConfig>,
}

impl ValidateOwned for TileConfig {
    type Output = TileDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TileDescriptor {
            name: validate_field("name", non_empty_string(&self.name))?,
            land: self.land.unwrap_or(false),
            water: self.water.unwrap_or(false),
            variants: validate_vec_field("variants", &self.variants,|v| v.validate_owned())?,
        })
    }
}

///
/// A tile variant descriptor
///
#[derive(Debug, Deserialize, PartialEq)]
struct TileVariantConfig {
    ///
    /// The texture name
    ///
    texture: String,
}

impl ValidateOwned for TileVariantConfig {
    type Output = TileVariantDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TileVariantDescriptor {
            texture: validate_field("texture", non_empty_string(&self.texture))?,
        })
    }
}

///
/// The descriptor for a tile set folder
///
struct TileSetsDescriptor {
    ///
    /// The tile sets
    ///
    tile_sets: Vec<String>,
}

///
/// A configuration for tile sets
///
#[derive(Debug, Deserialize, PartialEq)]
struct TileSetsConfig {
    ///
    /// The tile sets
    ///
    tile_sets: Vec<String>,
}

impl ValidateOwned for TileSetsConfig {
    type Output = TileSetsDescriptor;
    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TileSetsDescriptor {
            tile_sets: validate_vec_field("tile_sets", &self.tile_sets, non_empty_string)?,
        })
    }
}

///
/// An error that can occur loading tile sets
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// A configuration error
    ///
    Configuration(ConfigurationError),

    ///
    /// A resource error
    ///
    Resource(ResourceError),

    ///
    /// A validation error
    ///
    Validation(ValidationError),

    ///
    /// A texture error
    ///
    Texture(TextureError),
}

impl From<ConfigurationError> for Error {
    fn from(e: ConfigurationError) -> Self {
        Error::Configuration(e)
    }
}

impl From<ResourceError> for Error {
    fn from(e: ResourceError) -> Self {
        Error::Resource(e)
    }
}

impl From<ValidationError> for Error {
    fn from(e: ValidationError) -> Self {
        Error::Validation(e)
    }
}

impl From<TextureError> for Error {
    fn from(e: TextureError) -> Self {
        Error::Texture(e)
    }
}