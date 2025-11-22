use std::cmp::min;
use serde::{Deserialize, Serialize};
use crate::configuration::{Error as ConfigurationError, load_configuration};
use crate::environment::Environment;
use crate::validation::{matches_pattern, positive_integer, validate_field_with, Error as ValidationError, ValidateOwned};

use log::warn;
use regex::Regex;

///
/// The minimum screen width
///
const MIN_SCREEN_WIDTH: i32 = 800;
///
/// The minimum screen height
///
const MIN_SCREEN_HEIGHT: i32 = 600;

///
/// Settings load / save functionality
///
#[derive(Debug, PartialEq)]
pub struct Settings {
    ///
    /// Window width in pixels
    ///
    window_width: u32,
    
    ///
    /// Window height in pixels
    ///
    window_height: u32,
    
    ///
    /// The language ID
    /// 
    language_id: String,

    ///
    /// The module ID
    ///
    module_name: String,
}

impl Settings {

    ///
    /// Constructs a new settings instance
    ///
    pub fn new(environment: &Environment) -> Settings {
        Self::load(environment).unwrap_or_else(|error| {
            println!("settings could not be loaded: {:?}", error);
            warn!("settings could not be loaded: {:?}", error);
            Settings::default()
        })
    }

    ///
    /// Loads a settings object
    ///
    fn load(environment: &Environment) -> Result<Settings, Error> {
        let model: SettingsConfig = load_configuration(environment.create_user_data_path().join("settings.yaml"))?;
        Ok(model.validate_owned()?)
    }

    ///
    /// Returns the window width
    ///
    pub fn get_window_width(&self) -> u32 {
        self.window_width
    }

    ///
    /// Returns the window width
    ///
    pub fn get_window_height(&self) -> u32 {
        self.window_height
    }
    
    ///
    /// Returns the language ID
    /// 
    pub fn get_language_id(&self) -> &str {
        &self.language_id
    }

    ///
    /// Returns the module name
    ///
    pub fn get_module_name(&self) -> &str {
        &self.module_name
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            window_width: 1200,
            window_height: 800,
            language_id: String::from("en"),
            module_name: String::from("default"),
        }
    }
}

///
/// An error type used to create settings
///
#[derive(Debug, PartialEq)]
enum Error {
    ///
    /// Configuration errors
    ///
    Configuration(ConfigurationError),

    ///
    /// Validation errors
    ///
    Validation(ValidationError),
}

impl From<ConfigurationError> for Error {
    fn from(e: ConfigurationError) -> Self {
        Error::Configuration(e)
    }
}

impl From<ValidationError> for Error {
    fn from(e: ValidationError) -> Self {
        Error::Validation(e)
    }
}

///
/// The settings model
///
#[derive(Debug, Deserialize,PartialEq, Serialize)]
pub struct SettingsConfig {
    ///
    /// Window width in pixels
    ///
    window_width: i32,
    
    ///
    /// Window height in pixels
    ///
    window_height: i32,
    
    ///
    /// Language ID
    /// 
    language_id: String,

    ///
    /// The module name
    ///
    module_name: String,
}

impl ValidateOwned for SettingsConfig {
    type Output = Settings;

    fn validate_owned(&self) -> Result<Self::Output, ValidationError> {
        Ok(Settings {
            window_width: min(self.window_width, MIN_SCREEN_WIDTH).cast_unsigned(),
            window_height: min(self.window_height, MIN_SCREEN_HEIGHT).cast_unsigned(),
            language_id: matches_pattern(&self.language_id, Regex::new("^([[:lower:]]+)(_([[:lower:]]+))?$")?)?,
            module_name: matches_pattern(&self.module_name, Regex::new("^[[:ascii:]]+$")?)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_settings_failure() {
        let env = Environment::new_test().unwrap();
        assert_eq!(Settings::default(), Settings::new(&env));
    }

    #[test]
    pub fn load_settings_success() {
        let env = Environment::new_test().unwrap();
        let user_data_path = env.create_user_data_path();
        std::fs::create_dir(&user_data_path).unwrap();
        std::fs::write(user_data_path.join("settings.yaml"), "window_width: 800\nwindow_height: 600\nlanguage_id: en_uk\nmodule_name: default").unwrap();
        assert_eq!(Settings{
            window_width: 800,
            window_height: 600,
            language_id: String::from("en_uk"),
            module_name: String::from("default"),
        }, Settings::new(&env));
    }
}