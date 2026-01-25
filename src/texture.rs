use image::{GenericImage, ImageError, ImageReader, RgbaImage};
use log::{debug, error};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{TextureCreator, TextureValueError};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::configuration::{Error as ConfigurationError, load_configuration};
use crate::metrics::NonZeroDimensions2;
use crate::resource::{Error as ResourceError, ResourceMap};
use crate::validation::{non_empty_string, validate_field, validate_field_with, validate_vec_field, Error as ValidationError, ValidateOwned};

///
/// A wrapper around an SDL2 texture handle
///
pub struct Texture<'a> {
    handle: sdl2::render::Texture<'a>,
}

impl<'a> Texture<'a> {

    ///
    /// Loads a texture from the path
    ///
    fn from_path(creator: &'a TextureCreator<WindowContext>, path: &PathBuf) -> Result<Texture<'a>, Error> {
        let mut image = ImageReader::open(path)?.decode()?.into_rgba8();
        Self::from_image(creator, &mut image)
    }

    ///
    /// Loads a texture from the path and validates that it has the specified size
    ///
    fn from_path_with_size(creator: &'a TextureCreator<WindowContext>, path: &PathBuf, size: &NonZeroDimensions2<i32>) -> Result<Texture<'a>, Error> {
        let mut image = ImageReader::open(path)?.decode()?.into_rgba8();
        if image.width() != size.get_width().cast_unsigned() || image.height() != size.get_height().cast_unsigned() {
            Err(Error::BadTextureSize)
        } else {
            Self::from_image(creator, &mut image)
        }
    }

    ///
    /// Loads a texture from an image
    ///
    fn from_image(creator: &'a TextureCreator<WindowContext>, image: &mut RgbaImage) -> Result<Texture<'a>, Error> {
        let width = image.width();
        let height = image.height();
        let surface = Surface::from_data(
            image.as_mut(),
            width,
            height,
            width * 4,
            PixelFormatEnum::RGBA8888
        ).map_err(|msg| Error::Sdl(msg))?;
        Ok(Texture {
            handle: creator.create_texture_from_surface(surface)?,
        })
    }

    ///
    /// Loads textures from a descriptor located in the specified folder
    ///
    pub fn from_folder_path<P: AsRef<Path>>(creator: &'a TextureCreator<WindowContext>, path: P) -> Result<ResourceMap<Texture>, Error> {
        debug!("Loading textures...");
        let mut path = path.as_ref().to_path_buf();
        let mut map = ResourceMap::new();
        Self::from_folder_path_recursive(creator, &mut path, &mut map, None)?;
        debug!("Loaded {} textures.", map.len());
        Ok(map)
    }

    ///
    /// Recursively loads all texture from a folder and its subfolders
    ///
    fn from_folder_path_recursive(creator: &'a TextureCreator<WindowContext>,
                                  path: &mut PathBuf,
                                  map: &mut ResourceMap<Texture<'a>>,
                                  size: Option<&NonZeroDimensions2<i32>>) -> Result<(), Error> {
        debug!("Loading textures from folder {}", path.display());
        path.push("textures.yaml");
        let config: TextureFolderConfig = load_configuration(&path)?;
        path.pop();
        let descriptor = config.validate_owned()?;
        debug!("Loading textures from {} folders...", descriptor.folders.len());
        for folder in descriptor.folders {
            path.push(folder);
            if path.is_dir() {
                Self::from_folder_path_recursive(creator, path, map, descriptor.size.as_ref().or(size))?;
                path.pop();
            } else {
                error!("no such folder: {}", path.display());
                return Err(Error::IO(format!("no such folder: {}", path.display())));
            }
        }
        for descriptor in descriptor.textures {
            path.push(descriptor.file);
            debug!("Loading texture {} from {}", descriptor.name, path.display());
            let texture = match &descriptor.size {
                Some(size) => Self::from_path_with_size(creator, &path, size),
                None => Self::from_path(creator, &path)
            }?;
            map.insert_if_not_present(descriptor.name, texture)?;
            path.pop();
        }
        Ok(())
    }
}

///
/// The texture folder descriptor
///
#[derive(Debug, PartialEq)]
struct TextureFolderDescriptor {
    ///
    /// Subfolders to check
    ///
    folders: Vec<String>,

    ///
    /// Forces a check on the textures in this folders if set
    /// Also applies to subfolders
    ///
    size: Option<NonZeroDimensions2<i32>>,

    ///
    /// Textures to load
    ///
    textures: Vec<TextureDescriptor>
}

///
/// A texture descriptor
///
#[derive(Debug, PartialEq)]
struct TextureDescriptor {
    ///
    /// The name of the texture
    ///
    name: String,

    ///
    /// Forces a check on the texture size if set
    ///
    size: Option<NonZeroDimensions2<i32>>,

    ///
    /// The texture image file
    ///
    file: String,
}

///
/// Textures configuration model
///
#[derive(Debug, Deserialize, PartialEq)]
struct TextureFolderConfig {
    ///
    /// Subfolders to check
    ///
    folders: Option<Vec<String>>,

    ///
    /// Forces a check on the textures in this folders if set
    /// Also applies to subfolders
    ///
    size: Option<TextureSizeConfig>,

    ///
    /// Textures to load
    ///
    textures: Option<Vec<TextureConfig>>,
}

impl ValidateOwned for TextureFolderConfig {
    type Output = TextureFolderDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TextureFolderDescriptor {
            folders: self.folders.as_ref()
                .map(|fs| validate_vec_field("folders", fs, non_empty_string))
                .transpose()?
                .unwrap_or_else(Vec::new),
            size: validate_field("size", self.size.as_ref().map(|s| s.validate_owned()).transpose())?,
            textures: self.textures.as_ref()
                .map(|ts| validate_vec_field("textures", ts, |t| t.validate_owned()))
                .transpose()?
                .unwrap_or_else(Vec::new),
        })
    }
}

///
/// Texture configuration model
///
#[derive(Debug, Deserialize, PartialEq)]
struct TextureConfig {
    ///
    /// The name of the texture
    ///
    name: String,

    ///
    /// Forces a check on the texture size if set
    ///
    size: Option<TextureSizeConfig>,

    ///
    /// The texture image file
    ///
    file: String,
}

impl ValidateOwned for TextureConfig {
    type Output = TextureDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TextureDescriptor {
            name: validate_field("name", non_empty_string(&self.name))?,
            size: validate_field("size", self.size.as_ref().map(|s| s.validate_owned()).transpose())?,
            file: validate_field("file", non_empty_string(&self.file))?
        })
    }
}

///
/// Texture size configuration model
///
#[derive(Debug, Deserialize, PartialEq)]
struct TextureSizeConfig {
    ///
    /// Texture width
    ///
    width: i32,

    ///
    /// Texture height
    ///
    height: i32,
}

impl ValidateOwned for TextureSizeConfig {
    type Output = NonZeroDimensions2<i32>;

    fn validate_owned(&self) -> Result<NonZeroDimensions2<i32>, ValidationError> {
        NonZeroDimensions2::new(self.width, self.height)
            .map_err(|_| ValidationError::from_str("invalid texture size"))
    }
}

///
/// An index type for a subtexture
///
pub type SubTextureIndex = usize;

///
/// A wrapper around an SDL2 texture containing multiple tile_sets
///
pub struct TextureSet<'a> {
    ///
    /// The indexes of the subtextures by name
    ///
    indices_by_name: HashMap<String, usize>,

    ///
    /// The number of columns in the texture set
    /// This is also the number of rows because the texture is always square
    ///
    columns: usize,

    ///
    /// The SDL2 texture handle
    ///
    handle: sdl2::render::Texture<'a>,
}

///
/// A texture composed of multiple subtextures
///
impl<'a> TextureSet<'a> {

    ///
    /// Loads a texture set from a descriptor
    ///
    fn from_descriptor(creator: &'a TextureCreator<WindowContext>, descriptor: &TextureSetDescriptor, path: &mut PathBuf) -> Result<TextureSet<'a>, Error> {
        let width: u32 = descriptor.size.get_width().cast_unsigned();
        let height: u32 = descriptor.size.get_height().cast_unsigned();

        let columns = Self::calculate_columns(descriptor.textures.len());
        let mut image = RgbaImage::new(width * (columns as u32), height * (columns as u32));
        let mut indices_by_name = HashMap::new();
        let mut row: usize = 0;
        let mut col: usize = 0;
        for texture_descriptor in &descriptor.textures {
            path.push(&texture_descriptor.file);
            debug!("Loading subtexture {} from {}", texture_descriptor.name, path.display());
            let sub_image = ImageReader::open(&path)?.decode()?.into_rgba8();
            if sub_image.width() != width || sub_image.height() != height {
                return Err(Error::Sdl(format!("image size does not match texture size: {}", texture_descriptor.file)));
            }
            image.sub_image((col as u32)  * width, (row as u32) * height, width, height).copy_from(&sub_image, 0, 0)?;
            path.pop();
            indices_by_name.insert(texture_descriptor.name.clone(), row);
            if (col + 1) == columns {
                col = 0;
                row += 1;
            } else {
                col += 1
            }
        }
        let surface = Surface::from_data(
            image.as_mut(),
            width * columns as u32,
            height * columns as u32,
            width * 4,
            PixelFormatEnum::RGBA8888
        ).map_err(|msg| Error::Sdl(msg))?;
        let handle = creator.create_texture_from_surface(surface)?;

        Ok(TextureSet {
            indices_by_name,
            columns,
            handle,
        })
    }

    ///
    /// Loads texture sets from a folder
    ///
    pub fn from_folder_path(creator: &'a TextureCreator<WindowContext>, path: &mut PathBuf) -> Result<ResourceMap<TextureSet<'a>>, Error> {
        let mut map = ResourceMap::new();
        Self::from_folder_path_recursive(creator, path, &mut map)?;
        Ok(map)
    }

    ///
    /// Fetches the index of the subtexture
    ///
    pub fn get_index(&self, name: &str) -> Result<SubTextureIndex, Error> {
        self.indices_by_name.get(name).cloned().ok_or_else(|| Error::NotFound(name.to_string()))
    }

    ///
    /// Recursively loads texture sets into the specified map
    ///
    fn from_folder_path_recursive(creator: &'a TextureCreator<WindowContext>, path: &mut PathBuf, map: &mut ResourceMap<TextureSet<'a>>) -> Result<(), Error> {
        debug!("Loading texture sets from folder {}", path.display());
        path.push("texture_sets.yaml");
        let config: TextureSetFolderConfig = load_configuration(&path)?;
        path.pop();
        let descriptor = config.validate_owned()?;
        debug!("Loading texture sets from {} folders...", descriptor.folders.len());
        for folder in descriptor.folders {
            path.push(folder);
            if path.is_dir() {
                Self::from_folder_path_recursive(creator, path, map)?;
                path.pop();
            } else {
                error!("no such folder: {}", path.display());
                return Err(Error::IO(format!("no such folder: {}", path.display())));
            }
        }
        for descriptor in descriptor.texture_sets {
            debug!("Loading texture set {} from {}", descriptor.name, path.display());
            let texture_set = TextureSet::from_descriptor(creator, &descriptor, path)?;
            map.insert_if_not_present(descriptor.name, texture_set)?;
        }
        Ok(())
    }

    ///
    /// Calculate the amount of columns needed for an amount
    ///
    fn calculate_columns(amount: usize) -> usize {
        let mut i = 0;
        loop {
            let result = i * i;
            if result >= amount {
                break result;
            } else {
                i = i + 1;
            }
        }
    }
}


///
/// The texture set folder descriptor
///
struct TextureSetFolderDescriptor {
    ///
    /// Subfolders to check
    ///
    folders: Vec<String>,

    ///
    /// Texture sets to load
    ///
    texture_sets: Vec<TextureSetDescriptor>
}

///
/// A texture set descriptor
///
struct TextureSetDescriptor {
    ///
    /// The name of the texture set
    ///
    name: String,

    ///
    /// The size of each element
    ///
    size: NonZeroDimensions2<i32>,

    ///
    /// The elements of the set
    ///
    textures: Vec<TextureSetElementDescriptor>,
}

///
/// A texture set element descriptor
///
struct TextureSetElementDescriptor {
    ///
    /// The name of the texture
    ///
    name: String,

    ///
    /// The texture image file
    ///
    file: String,
}

///
/// Texture ses configuration model
///
#[derive(Debug, Deserialize, PartialEq)]
struct TextureSetFolderConfig {
    ///
    /// Subfolders to check
    ///
    folders: Option<Vec<String>>,

    ///
    /// Forces a check on the textures in this folders if set
    /// Also applies to subfolders
    ///
    size: Option<TextureSizeConfig>,

    ///
    /// Texture sets to load
    ///
    texture_sets: Option<Vec<TextureSetConfig>>,
}

impl ValidateOwned for TextureSetFolderConfig {
    type Output = TextureSetFolderDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TextureSetFolderDescriptor {
            folders: self.folders.as_ref()
                .map(|fs| validate_vec_field("folders", fs, non_empty_string))
                .transpose()?
                .unwrap_or_else(Vec::new),
            texture_sets: self.texture_sets.as_ref()
                .map(|ts| validate_vec_field("texture_sets", ts, |t| t.validate_owned()))
                .transpose()?
                .unwrap_or_else(Vec::new),
        })
    }
}

///
/// The configuration model for the texture set descriptor
///
#[derive(Debug, Deserialize, PartialEq)]
struct TextureSetConfig {
    ///
    /// The name of the texture set
    ///
    name: String,

    ///
    /// The size of each element
    ///
    size: TextureSizeConfig,

    ///
    /// The elements of the set
    ///
    textures: Vec<TextureSetElementConfig>,
}

impl ValidateOwned for TextureSetConfig {
    type Output = TextureSetDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TextureSetDescriptor {
            name: validate_field("name", non_empty_string(&self.name))?,
            size: validate_field("size", self.size.validate_owned())?,
            textures: validate_vec_field("textures", self.textures.as_ref(), |t| t.validate_owned())?,
        })
    }
}

///
/// A texture descriptor configuration model
///
#[derive(Debug, Deserialize, PartialEq)]
struct TextureSetElementConfig {
    ///
    /// The name of the texture
    ///
    name: String,

    ///
    /// The texture image file
    ///
    file: String,
}

impl ValidateOwned for TextureSetElementConfig {
    type Output = TextureSetElementDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TextureSetElementDescriptor {
            name: validate_field("name", non_empty_string(&self.name))?,
            file: validate_field("file", non_empty_string(&self.file))?
        })
    }
}

///
/// Errors that can occur when loading textures
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// Configuration error
    ///
    Configuration(ConfigurationError),

    ///
    /// Bad texture size
    ///
    BadTextureSize,

    ///
    /// An image read error occurred
    ///
    IO(String),

    ///
    /// A resource error
    ///
    Resource(ResourceError),

    ///
    /// An SDL error occurred
    ///
    Sdl(String),

    ///
    /// A validation error occurred
    ///
    Validation(ValidationError),

    ///
    /// No subtexture was found for this name
    ///
    NotFound(String),
}

impl From<ConfigurationError> for Error {
    fn from(e: ConfigurationError) -> Self {
        Error::Configuration(e)
    }
}

impl From<ImageError> for Error {
    fn from(e: ImageError) -> Error {
        Error::IO(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e.to_string())
    }
}

impl From<TextureValueError> for Error {
    fn from(e: TextureValueError) -> Error {
        Error::Sdl(e.to_string())
    }
}

impl From<ValidationError> for Error {
    fn from(e: ValidationError) -> Error {
        Error::Validation(e)
    }
}

impl From<ResourceError> for Error {
    fn from(e: ResourceError) -> Error {
        Error::Resource(e)
    }
}