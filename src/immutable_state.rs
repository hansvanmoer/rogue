use std::fmt::Display;
use std::path::PathBuf;
use log::{debug, info};
use regex::Regex;
use serde::Deserialize;
use crate::configuration::{Error as ConfigurationError, load_configuration};
use crate::environment::Environment;
use crate::localization::{Error as LocalizationError, load_labels_for_language};
use crate::material::{Error as MaterialError, MaterialDescriptor};
use crate::resource::{Error as ResourceError, ResourceMap};
use crate::settings::Settings;
use crate::system::SubSystems;
use crate::texture::{Error as TextureError, Texture, TextureSet};
use crate::tile_set::{Error as TileSetError, TileSet};
use crate::validation::{Error as ValidationError, matches_pattern_and_capture, non_empty_string, validate_field, validate_field_with, ValidateOwned};

///
/// The immutable game state
///
pub struct ImmutableState<'a> {
    ///
    /// The module descriptor
    ///
    descriptor: ModuleDescriptor,

    ///
    /// The module labels
    ///
    labels: ResourceMap<String>,

    ///
    /// Materials
    ///
    materials: ResourceMap<MaterialDescriptor>,

    ///
    /// The module textures
    ///
    textures: ResourceMap<Texture<'a>>,

    ///
    /// The module textures set
    ///
    texture_sets: ResourceMap<TextureSet<'a>>,

    ///
    /// The module tile sets
    ///
    tile_sets: ResourceMap<TileSet>,
}

impl<'a> ImmutableState<'a> {
    ///
    /// Loads all immutable resources into the state
    ///
    pub fn new(environment: &Environment, settings: &Settings, sub_systems: &'a SubSystems) -> Result<ImmutableState<'a>, Error> {
        let mut path = Self::find_module_path(environment, settings.get_module_name())
            .ok_or_else(|| Error::ModuleNotFound(String::from(settings.get_module_name())))?;

        info!("Loading module from path {}...", path.display());
        path.push("module.yaml");
        let descriptor = load_configuration::<&PathBuf, ModuleDescriptorConfig>(&path)?
            .validate_owned()?;

        info!("Module {} with version {} loaded", descriptor.name, descriptor.version);

        path.pop();
        path.push(&descriptor.labels_folder);
        debug!("Loading labels from path {}...", path.as_path().display());
        let labels = load_labels_for_language(&mut path, settings)?;
        debug!("Labels loaded: {}", labels.len());
        path.pop();

        path.push(&descriptor.textures_folder);
        debug!("Loading textures from path {}...", path.as_path().display());
        let textures = Texture::from_folder_path(sub_systems.texture_creator(), &path)?;
        path.pop();
        path.push(&descriptor.texture_sets_folder);
        let texture_sets = TextureSet::from_folder_path(sub_systems.texture_creator(), &mut path)?;
        path.pop();
        debug!("Textures loaded: {}", textures.len());

        path.push("materials");
        debug!("Loading materials from path {}...", path.as_path().display());
        let materials = MaterialDescriptor::from_folder_path(&mut path)?;
        debug!("Materials loaded: {}", materials.len());
        path.pop();

        path.push(&descriptor.tile_sets_folder);
        debug!("Loading tile sets from path {}...", path.as_path().display());
        let tile_sets = TileSet::from_folder_path(&mut path, &texture_sets)?;
        path.pop();

        debug!("All resources loaded.");

        Ok(ImmutableState {
            descriptor,
            labels,
            materials,
            textures,
            texture_sets,
            tile_sets,
        })
    }

    ///
    /// find the module path
    ///
    fn find_module_path(environment: &Environment, module_name: &str) -> Option<PathBuf> {
        let mut path = environment.create_data_path();
        path.push("modules");
        path.push(module_name);
        if path.is_dir() {
            Some(path)
        } else {
            path = environment.create_user_data_path();
            path.push("modules");
            path.push(module_name);
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        }
    }
    
    ///
    /// Returns the texture map
    /// 
    pub fn textures(&self) -> &ResourceMap<Texture<'a>> {
        &self.textures
    }
    
    ///
    /// Returns the texture set
    /// 
    pub fn texture_sets(&self) -> &ResourceMap<TextureSet<'a>> {
        &self.texture_sets
    }
}

///
/// The module descriptor
///
struct ModuleDescriptor {
    ///
    /// The module descriptor
    ///
    name: String,

    ///
    /// The module version
    ///
    version: Version,

    ///
    /// The label folder
    ///
    labels_folder: String,

    ///
    /// The texture folder
    ///
    textures_folder: String,

    ///
    /// The texture set folder
    ///
    texture_sets_folder: String,

    ///
    /// The tile set folder
    ///
    tile_sets_folder: String,
}

///
/// The module config
///
#[derive(Debug, Deserialize, PartialEq)]
struct ModuleDescriptorConfig {
    ///
    /// The module name
    ///
    name: String,

    ///
    /// The version string
    ///
    version: String,

    ///
    /// The label folders
    ///
    labels_folder: String,

    ///
    /// The texture folder
    ///
    textures_folder: String,

    ///
    /// The texture set folder
    ///
    texture_sets_folder: String,

    ///
    /// The tile set folder
    ///
    tile_sets_folder: String,
}

impl ValidateOwned for ModuleDescriptorConfig {
    type Output = ModuleDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, crate::validation::Error> {
        let name = validate_field_with("name", || matches_pattern_and_capture(&self.name, &Regex::new("\\w+")?)
            .map(|v| v.into_iter()
                .map(String::from)
                .next()
                .unwrap_or_else(|| String::from("default"))
            )
        )?;
        let version = validate_field_with("version", || matches_pattern_and_capture(&self.name, &Regex::new("\\w+")?)
            .map(|v| {
                let mut iter = v
                    .into_iter()
                    .map(|s| s.parse::<u32>())
                    .filter(Result::is_ok)
                    .map(Result::unwrap);
                let major = iter.next().unwrap_or(0);
                let minor = iter.next().unwrap_or(0);
                let bugfix = iter.next().unwrap_or(0);
                Version::new(major, minor, bugfix)
            })
        )?;
        let labels_folder = validate_field("labels_folders", non_empty_string(&self.labels_folder))?;
        let textures_folder = validate_field("textures_folder", non_empty_string(&self.textures_folder))?;
        let texture_sets_folder = validate_field("texture_sets_folder", non_empty_string(&self.texture_sets_folder))?;
        let tile_sets_folder = validate_field("tile_sets_folder", non_empty_string(&self.tile_sets_folder))?;
        Ok(ModuleDescriptor {
            name,
            version,
            labels_folder,
            textures_folder,
            texture_sets_folder,
            tile_sets_folder,
        })
    }
}


///
/// A version tuple
///
#[derive(Debug, PartialEq)]
pub struct Version {
    ///
    /// The major version
    ///
    major: u32,

    ///
    /// The minor version
    ///
    minor: u32,

    ///
    /// The bugfix version
    ///
    bugfix: u32,
}

impl Version {
    ///
    /// Constructs a new version
    ///
    fn new(major: u32, minor: u32, bugfix: u32) -> Version {
        Version {
            major,
            minor,
            bugfix,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.bugfix)
    }
}

///
/// Errors that can occur when loading the immutable state
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// A configuration error occurred
    ///
    Configuration(crate::configuration::Error),

    ///
    /// Localization error
    ///
    Localization(LocalizationError),

    ///
    /// The module was not found
    ///
    ModuleNotFound(String),

    ///
    /// An error occurred while loading materials
    ///
    Material(MaterialError),

    ///
    /// A resource error occurred
    ///
    Resource(crate::resource::Error),

    ///
    /// And SDL error occurred
    ///
    Sdl(String),

    ///
    /// An error occurred while loading textures
    ///
    Texture(TextureError),

    ///
    /// An error occurred while loading tile sets
    ///
    TileSet(TileSetError),

    ///
    /// A validation error occurred
    ///
    Validation(crate::validation::Error),
}

impl From<ConfigurationError> for Error {
    fn from(e: ConfigurationError) -> Self {
        Error::Configuration(e)
    }
}

impl From<ValidationError> for Error {
    fn from(e: crate::validation::Error) -> Self {
        Error::Validation(e)
    }
}

impl From<ResourceError> for Error {
    fn from(e: crate::resource::Error) -> Self {
        Error::Resource(e)
    }
}

impl From<TextureError> for Error {
    fn from(e: TextureError) -> Self {
        Error::Texture(e)
    }
}

impl From<LocalizationError> for Error {
    fn from(e: LocalizationError) -> Self {
        Error::Localization(e)
    }
}

impl From<MaterialError> for Error {
    fn from(e: MaterialError) -> Self {
        Error::Material(e)
    }
}

impl From<TileSetError> for Error {
    fn from(e: TileSetError) -> Self {
        Error::TileSet(e)
    }
}