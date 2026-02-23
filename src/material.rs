use log::debug;
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::configuration::{Error as ConfigurationError, load_configuration};
use crate::resource::{Error as ResourceError, ResourceMap};
use crate::validation::{Error as ValidationError, non_empty_string, positive_f32, strictly_positive_f32, validate_field, validate_vec_field, ValidateOwned, validate_optional_vec_field};

///
/// Material properties are used for a simplistic physics simulation
/// allowing us to set fire to buildings, collapse them, freeze water, and so on
///
#[derive(Debug, PartialEq)]
pub struct MaterialDescriptor {
    ///
    /// The unique name of the material
    ///
    name: String,

    ///
    /// The density in kg per cubic meter (kg / m^3)
    /// As a simplification, this is assumed to be constant
    ///
    density: f32,

    ///
    /// The heat capacity in Joule per Kelvin (J / K)
    /// As a simplification, this is assumed to be constant
    ///
    heat_capacity: f32,

    ///
    /// Thermal conductivity in Watt per meter Kelvin (W / (m K))
    /// As a simplification, this is assumed to be constant
    ///
    thermal_conductivity: f32,

    ///
    /// The auto ignition temperature in Kelvin (K)
    /// As a simplification, ignition happens instantly
    ///
    auto_ignition_temperature: f32,

    ///
    /// The melting temperature in Kelvin (K)
    /// As a simplification, melting happens instantly and regardless of pressure
    ///
    melting_temperature: f32,

    ///
    /// The boiling temperature in Kelvin (K)
    /// As a simplification, boiling happens instantly and regardless of pressure
    ///
    boiling_temperature: f32,

    ///
    /// Ultimate tensile strength in Pascal
    ///
    ultimate_tensile_strength: f32,

    ///
    /// The shear strength factor
    ///
    shear_strength_factor: f32,

    ///
    /// Compressive strength in Pascal
    ///
    compressive_strength: f32,
}

impl MaterialDescriptor {
    ///
    /// Loads all materials from the configuration file
    ///
    pub fn from_folder_path<P: AsRef<Path>>(path: P) -> Result<ResourceMap<MaterialDescriptor>, Error> {
        let mut path = path.as_ref().to_path_buf();
        debug!("Loading materials from folder {:?}", path);
        let mut resources = ResourceMap::new();
        Self::from_file_recursive(&mut path, "materials.yaml", &mut resources)?;
        Ok(resources)
    }

    ///
    /// Loads materials from files
    ///
    fn from_file_recursive(path: &mut PathBuf, file_name: &str, resources: &mut ResourceMap<MaterialDescriptor>) -> Result<(), Error> {
        path.push(file_name);
        debug!("Loading materials from file {:?}", path);
        let config: MaterialsConfig = load_configuration(&path)?;
        path.pop();
        let mut descriptor = config.validate_owned()?;
        for material in descriptor.materials.drain(..) {
        resources.insert_if_not_present(material.name.clone(), material)?;
        }
        for included_file in descriptor.files {
            Self::from_file_recursive(path, &included_file, resources)?;
        }
        Ok(())
    }
}

///
/// The configuration model for the material
///
#[derive(Debug, Deserialize, PartialEq)]
struct MaterialConfig {
    ///
    /// The unique name of the material
    ///
    name: String,

    ///
    /// The density in kg per cubic meter (kg / m^3)
    /// As a simplification, this is assumed to be constant
    ///
    density: f32,

    ///
    /// The heat capacity in Joule per Kelvin (J / K)
    /// As a simplification, this is assumed to be constant
    ///
    heat_capacity: f32,

    ///
    /// Thermal conductivity in Watt per meter Kelvin (W / (m K))
    /// As a simplification, this is assumed to be constant
    ///
    thermal_conductivity: f32,

    ///
    /// The auto ignition temperature in Kelvin (K)
    /// As a simplification, ignition happens instantly
    ///
    auto_ignition_temperature: f32,

    ///
    /// The melting temperature in Kelvin (K)
    /// As a simplification, melting happens instantly and regardless of pressure
    ///
    melting_temperature: f32,

    ///
    /// The boiling temperature in Kelvin (K)
    /// As a simplification, boiling happens instantly and regardless of pressure
    ///
    boiling_temperature: f32,

    ///
    /// Ultimate tensile strength in Pascal
    ///
    ultimate_tensile_strength: f32,

    ///
    /// The shear strength factor
    ///
    shear_strength_factor: f32,

    ///
    /// Compressive strength in Pascal
    ///
    compressive_strength: f32,
}

impl ValidateOwned for MaterialConfig {
    type Output = MaterialDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(MaterialDescriptor{
            name: validate_field("name", non_empty_string(&self.name))?,
            density: validate_field("density", strictly_positive_f32(&self.density))?,
            heat_capacity: validate_field("heat_capacity", strictly_positive_f32(&self.heat_capacity))?,
            thermal_conductivity: validate_field("thermal_conductivity", positive_f32(&self.thermal_conductivity))?,
            auto_ignition_temperature: validate_field("auto_ignition_temperature", positive_f32(&self.auto_ignition_temperature))?,
            melting_temperature: validate_field("melting_temperature", positive_f32(&self.melting_temperature))?,
            boiling_temperature: validate_field("boiling_temperature", positive_f32(&self.boiling_temperature))?,
            ultimate_tensile_strength: validate_field("ultimate_tensile_strength", strictly_positive_f32(&self.ultimate_tensile_strength))?,
            shear_strength_factor: validate_field("shear_strength_factor", strictly_positive_f32(&self.shear_strength_factor))?,
            compressive_strength: validate_field("compressive_strength", strictly_positive_f32(&self.compressive_strength))?,
        })
    }
}

///
/// A descriptor for a list of material descriptors and associated files
///
#[derive(Debug, PartialEq)]
pub struct MaterialsDescriptor {
    ///
    /// Other files to load
    ///
    files: Vec<String>,

    ///
    /// Material descriptors
    ///
    materials: Vec<MaterialDescriptor>,
}

///
/// A configuration model for a list of material descriptors and associated files
///
#[derive(Debug, Deserialize, PartialEq)]
pub struct MaterialsConfig {
    ///
    /// Other files to load
    ///
    files: Option<Vec<String>>,

    ///
    /// Material descriptors
    ///
    materials: Option<Vec<MaterialConfig>>,
}

impl ValidateOwned for MaterialsConfig {
    type Output = MaterialsDescriptor;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(MaterialsDescriptor{
            files: validate_optional_vec_field("files", &self.files, non_empty_string)?,
            materials: validate_optional_vec_field("materials", &self.materials, MaterialConfig::validate_owned)?,
        })
    }
}

///
/// Errors that can occur when loading materials from the configuration file
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// An IO error occurred
    ///
    IO(String),

    ///
    /// A configuration error occurred
    ///
    Configuration(ConfigurationError),

    ///
    /// A resource error occurred.
    ///
    Resource(ResourceError),

    ///
    /// A validation error occurred
    ///
    Validation(ValidationError)
}

impl From<ConfigurationError> for Error {
    fn from(error: ConfigurationError) -> Self {
        Error::Configuration(error)
    }
}

impl From<ValidationError> for Error {
    fn from(error: ValidationError) -> Self {
        Error::Validation(error)
    }
}

impl From<ResourceError> for Error {
    fn from(error: ResourceError) -> Self {
        Error::Resource(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_load_config() {
        let path = std::env::current_dir().unwrap();
        let resources = MaterialDescriptor::from_folder_path(&path.join("data-test").join("materials")).unwrap();
        assert_eq!(resources.len(), 1);
        let wood = resources.get_required_by_name("wood").unwrap();
        let expected = MaterialDescriptor {
            name: String::from("wood"),
            density: 400.0,
            heat_capacity: 0.0017,
            thermal_conductivity: 0.12,
            auto_ignition_temperature: 527.0,
            melting_temperature: 100000.0,
            boiling_temperature: 100000.0,
            ultimate_tensile_strength: 40.0,
            shear_strength_factor: 9.0,
            compressive_strength: 40.0,
        };
        assert_eq!(&expected, wood);
    }
}