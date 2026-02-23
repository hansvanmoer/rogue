use std::path::PathBuf;
use serde::Deserialize;
use crate::configuration::{Error as ConfigurationError};
use crate::metrics::NonZeroDimensions3;
use crate::resource::{Error as ResourceError, LoadableResource, ResourceDescriptor, ResourceMap};
use crate::validation::{non_empty_string, validate_field, validate_optional_vec_field, Error as ValidationError, ValidateOwned};

pub struct BuildingComponent {}

impl BuildingComponent {
    ///
    /// Loads the building components
    ///
    pub fn from_folder_path(path: &mut PathBuf) -> Result<ResourceMap<Self>, Error> {
        ComponentsDescriptor::from_folder_path(&(), path)
    }
}

impl LoadableResource<ComponentsDescriptor, (), Error> for BuildingComponent {
    fn load(resources: &mut ResourceMap<Self>, _state: &(), descriptor: ComponentsDescriptor) -> Result<(), Error> {
        Ok(())
    }
}

pub struct BuildingStyle {}

impl BuildingStyle {
    ///
    /// Loads the building styles
    ///
    pub fn from_folder_path(path: &mut PathBuf) -> Result<ResourceMap<Self>, Error> {
        StylesDescriptor::from_folder_path(&(), path)
    }
}

impl LoadableResource<StylesDescriptor, (), Error> for BuildingStyle {
    fn load(resources: &mut ResourceMap<Self>, _state: &(), descriptor: StylesDescriptor) -> Result<(), Error> {
        Ok(())
    }
}

pub struct BuildingTemplate {}

impl BuildingTemplate {
    ///
    /// Loads the building styles
    ///
    pub fn from_folder_path(path: &mut PathBuf) -> Result<ResourceMap<Self>, Error> {
        TemplatesDescriptor::from_folder_path(&(), path)
    }
}

impl LoadableResource<TemplatesDescriptor, (), Error> for BuildingTemplate {
    fn load(resources: &mut ResourceMap<Self>, _state: &(), descriptor: TemplatesDescriptor) -> Result<(), Error> {
        Ok(())
    }
}

///
/// Errors that can occur when loading the building templates
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// A configuration error occurred
    ///
    Configuration(ConfigurationError),
    ///
    /// An IO error occurred
    ///
    IO(String),

    ///
    /// A resource error occurred
    ///
    Resource(ResourceError),

    ///
    /// A validation error occurred
    ///
    Validation(ValidationError),
}

impl From<ConfigurationError> for Error {
    fn from(error: ConfigurationError) -> Self {
        Self::Configuration(error)
    }
}

impl From<ResourceError> for Error {
    fn from(error: ResourceError) -> Self {
        Self::Resource(error)
    }
}

impl From<ValidationError> for Error {
    fn from(error: ValidationError) -> Self {
        Self::Validation(error)
    }
}

///
/// The component kind
///
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
enum ComponentKind {
    ///
    /// The component is a wall
    ///
    Wall,
}

///
/// Building descriptor configuration
///
#[derive(Debug, PartialEq)]
struct ComponentsDescriptor {
    ///
    /// Subfolders to scan
    ///
    folders: Vec<String>,

    ///
    /// Descriptor files to load
    ///
    files: Vec<String>,

    ///
    /// Building components
    ///
    components: Vec<ComponentDescriptor>,
}

impl ResourceDescriptor for ComponentsDescriptor {
    type Error = Error;
    type Configuration = ComponentsConfig;
    type State = ();
    type Resource = BuildingComponent;

    fn resource_type_name() -> &'static str {
        "building component"
    }

    fn main_descriptor_file_name() -> &'static str {
        "building_components"
    }

    fn get_files(&self) -> &Vec<String> {
        &self.files
    }
    fn get_folders(&self) -> &Vec<String>{
        &self.folders
    }
}

///
/// Building descriptor configuration
///
#[derive(Debug, PartialEq)]
struct TemplatesDescriptor {
    ///
    /// Subfolders to scan
    ///
    folders: Vec<String>,

    ///
    /// Descriptor files to load
    ///
    files: Vec<String>,

    ///
    /// Building templates
    ///
    templates: Vec<TemplateDescriptor>,
}

impl ResourceDescriptor for TemplatesDescriptor {
    type Error = Error;
    type Configuration = TemplatesConfig;
    type State = ();
    type Resource = BuildingTemplate;

    fn resource_type_name() -> &'static str {
        "building template"
    }

    fn main_descriptor_file_name() -> &'static str {
        "building_templates"
    }

    fn get_files(&self) -> &Vec<String> {
        &self.files
    }
    fn get_folders(&self) -> &Vec<String>{
        &self.folders
    }
}

///
/// Building descriptor configuration
///
#[derive(Debug, PartialEq)]
struct StylesDescriptor {
    ///
    /// Subfolders to scan
    ///
    folders: Vec<String>,

    ///
    /// Descriptor files to load
    ///
    files: Vec<String>,

    ///
    /// Building styles
    ///
    styles: Vec<StyleDescriptor>,
}

impl ResourceDescriptor for StylesDescriptor {
    type Error = Error;
    type Configuration = StylesConfig;
    type State = ();
    type Resource = BuildingStyle;

    fn resource_type_name() -> &'static str {
        "building style"
    }

    fn main_descriptor_file_name() -> &'static str {
        "building_styles"
    }

    fn get_files(&self) -> &Vec<String> {
        &self.files
    }
    fn get_folders(&self) -> &Vec<String>{
        &self.folders
    }
}

///
/// Component configuration
///
#[derive(Debug, PartialEq)]
struct ComponentDescriptor {
    ///
    /// The component name
    ///
    name: String,

    ///
    /// The component kind
    ///
    kind: ComponentKind,

    ///
    /// The horizontal texture name
    ///
    horizontal_texture: String,

    ///
    /// The vertical texture name
    ///
    vertical_texture: String,

    ///
    /// The corner texture name
    ///
    corner_texture: String,
}

///
/// Building template descriptor
///
#[derive(Debug, PartialEq)]
struct TemplateDescriptor {
    ///
    /// Name
    ///
    name: String,

    ///
    /// Variable components
    ///
    variable_components: Vec<VariableComponentDescriptor>,

}

///
/// A variable component in a building template
///
#[derive(Debug, PartialEq)]
struct VariableComponentDescriptor {
    ///
    /// The name of the variable
    ///
    name: String,

    ///
    /// The kind of component
    ///
    kind: ComponentKind,
}

///
/// A template command descriptor
///
#[derive(Debug, PartialEq)]
struct CommandDescriptor {
    ///
    /// The component to place
    ///
    component: String,
    ///
    /// The wall box to construct
    ///
    wall_box: WallBoxDescriptor,
}

///
/// The wall box descriptor
///
#[derive(Debug, PartialEq)]
struct WallBoxDescriptor {
    ///
    /// The x coordinate
    ///
    x: i32,
    ///
    /// The y coordinate
    ///
    y: i32,
    ///
    /// The z coordinate
    ///
    z: i32,

    ///
    /// The size of the box
    ///
    size: NonZeroDimensions3<i32>,
}

///
/// The building style descriptor
///
#[derive(Debug, PartialEq)]
struct StyleDescriptor {
    ///
    /// Variable assignments
    ///
    variable_assignments: Vec<VariableAssignmentDescriptor>,
}

///
/// Variable assignments in styles
///
#[derive(Debug, PartialEq)]
struct VariableAssignmentDescriptor {
    ///
    /// The name of the variable
    ///
    name: String,

    ///
    /// The component name
    ///
    component: String,
}

///
/// Building descriptor configuration
///
#[derive(Debug, PartialEq, Deserialize)]
struct ComponentsConfig {
    ///
    /// Subfolders to scan
    ///
    folders: Option<Vec<String>>,

    ///
    /// Descriptor files to load
    ///
    files: Option<Vec<String>>,

    ///
    /// Building components
    ///
    components: Option<Vec<ComponentConfig>>,
}

impl ValidateOwned for ComponentsConfig {
    type Output = ComponentsDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(Self::Output {
            folders: validate_optional_vec_field("folders", &self.folders, non_empty_string)?,
            files: validate_optional_vec_field("files", &self.files, non_empty_string)?,
            components: validate_optional_vec_field("components", &self.components, ValidateOwned::validate_owned)?,
        })
    }
}

///
/// Building descriptor configuration
///
#[derive(Debug, PartialEq, Deserialize)]
struct TemplatesConfig {
    ///
    /// Subfolders to scan
    ///
    folders: Option<Vec<String>>,

    ///
    /// Descriptor files to load
    ///
    files: Option<Vec<String>>,

    ///
    /// Building templates
    ///
    templates: Option<Vec<TemplateConfig>>,
}

impl ValidateOwned for TemplatesConfig {
    type Output = TemplatesDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(Self::Output {
            folders: validate_optional_vec_field("folders", &self.folders, non_empty_string)?,
            files: validate_optional_vec_field("files", &self.files, non_empty_string)?,
            templates: validate_optional_vec_field("templates", &self.templates, ValidateOwned::validate_owned)?,
        })
    }
}

///
/// Building descriptor configuration
///
#[derive(Debug, PartialEq, Deserialize)]
struct StylesConfig {
    ///
    /// Subfolders to scan
    ///
    folders: Option<Vec<String>>,

    ///
    /// Descriptor files to load
    ///
    files: Option<Vec<String>>,

    ///
    /// Building styles
    ///
    styles: Option<Vec<StyleConfig>>,
}

impl ValidateOwned for StylesConfig {
    type Output = StylesDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(Self::Output {
            folders: validate_optional_vec_field("folders", &self.folders, non_empty_string)?,
            files: validate_optional_vec_field("files", &self.files, non_empty_string)?,
            styles: validate_optional_vec_field("styles", &self.styles, ValidateOwned::validate_owned)?,
        })
    }
}

///
/// Component configuration
///
#[derive(Debug, PartialEq, Deserialize)]
struct ComponentConfig {
    ///
    /// The component name
    ///
    name: String,

    ///
    /// The component kind
    ///
    kind: ComponentKind,

    ///
    /// The horizontal texture name
    ///
    horizontal_texture: String,

    ///
    /// The vertical texture name
    ///
    vertical_texture: String,

    ///
    /// The corner texture name
    ///
    corner_texture: String,
}

impl ValidateOwned for ComponentConfig {
    type Output = ComponentDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(Self::Output {
            name: validate_field("name", non_empty_string(&self.name))?,
            kind: self.kind,
            horizontal_texture: validate_field("horizontal_texture", non_empty_string(&self.horizontal_texture))?,
            vertical_texture: validate_field("vertical_texture", non_empty_string(&self.vertical_texture))?,
            corner_texture: validate_field("corner_texture", non_empty_string(&self.corner_texture))?,
        })
    }
}

///
/// Building template configuration
///
#[derive(Debug, PartialEq, Deserialize)]
struct TemplateConfig {
    ///
    /// Name
    ///
    name: String,

    ///
    /// Variable components
    ///
    variable_components: Option<Vec<VariableComponentConfig>>,
}

impl ValidateOwned for TemplateConfig {
    type Output = TemplateDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(TemplateDescriptor {
            name: validate_field("name", non_empty_string(&self.name))?,
            variable_components: validate_optional_vec_field("variable_components", &self.variable_components, ValidateOwned::validate_owned)?,
        })
    }
}

///
/// A variable component in a building template
///
#[derive(Debug, PartialEq, Deserialize)]
struct VariableComponentConfig {
    ///
    /// The name of the variable
    ///
    name: String,

    ///
    /// The kind of component
    ///
    kind: ComponentKind,
}

impl ValidateOwned for VariableComponentConfig {
    type Output = VariableComponentDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(Self::Output {
            name: validate_field("name", non_empty_string(&self.name))?,
            kind: self.kind,
        })
    }
}

///
/// A template command configuration
///
#[derive(Debug, PartialEq, Deserialize)]
struct CommandConfig {
    ///
    /// The component to place
    ///
    component: String,
    ///
    /// The wall box to construct
    ///
    wall_box: WallBoxConfig,
}

impl ValidateOwned for CommandConfig {
    type Output = CommandDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(CommandDescriptor {
            component: validate_field("component", non_empty_string(&self.component))?,
            wall_box: validate_field("wall_box", WallBoxConfig::validate_owned(&self.wall_box))?,
        })
    }
}

///
/// The wall box configuration
///
#[derive(Debug, PartialEq, Deserialize)]
struct WallBoxConfig {
    ///
    /// The first x coordinate
    ///
    x1: i32,

    ///
    /// The second x coordinate
    ///
    x2: i32,

    ///
    /// The first y coordinate
    ///
    y1: i32,

    ///
    /// The second y coordinate
    ///
    y2: i32,

    ///
    /// The first z coordinate
    ///
    z1: i32,

    ///
    /// The second z coordinate
    ///
    z2: i32,
}

impl ValidateOwned for WallBoxConfig {
    type Output = WallBoxDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        let size = NonZeroDimensions3::new(self.x1 - self.x2, self.y1 - self.y2, self.z1 - self.z2).map_err(|_| ValidationError::from_str("invalid wall box bounds"))?;
        let x = self.x1.min(self.x2);
        let y = self.y1.min(self.y2);
        let z = self.z1.min(self.z2);
        Ok(WallBoxDescriptor {
            x,
            y,
            z,
            size,
        })
    }
}

///
/// Configuration for a building style
///
#[derive(Debug, PartialEq, Deserialize)]
struct StyleConfig {
    ///
    /// Variable assignments
    ///
    variable_assignments: Option<Vec<VariableAssignmentConfig>>,
}

impl ValidateOwned for StyleConfig {
    type Output = StyleDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(StyleDescriptor {
            variable_assignments: validate_optional_vec_field("", &self.variable_assignments, ValidateOwned::validate_owned)?,
        })
    }
}

///
/// Variable assignments in styles
///
#[derive(Debug, PartialEq, Deserialize)]
struct VariableAssignmentConfig {
    ///
    /// The name of the variable
    ///
    name: String,

    ///
    /// The component name
    ///
    component: String,
}

impl ValidateOwned for VariableAssignmentConfig {
    type Output = VariableAssignmentDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(VariableAssignmentDescriptor {
            name: validate_field("name", non_empty_string(&self.name))?,
            component: validate_field("component", non_empty_string(&self.component))?,
        })
    }
}