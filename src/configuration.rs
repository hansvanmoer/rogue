use std::fs::File;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

///
/// Loads and parses a configuration file
///
pub fn load_configuration<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T, Error> {
    Ok(serde_yaml::from_reader::<File, T>(File::open(&path)?)?)
}

///
/// Formats and saves a configuration file
///
pub fn save_configuration<P: AsRef<Path>, T: Serialize>(path: P, data: &T) -> Result<(), Error> {
    serde_yaml::to_writer::<File, T>(File::create(&path)?, data)?;
    Ok(())
}

///
/// Errors that occur when reading or writing configuration files
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// An IO error occured while loading or saving configuration
    ///
    IOError(String),

    ///
    /// A parser error occurred while loading the configuration
    ///
    ParseError(String),

    ///
    /// A formatting error occurred while saving the configuration
    ///
    FormatError(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::ParseError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use serde::Deserialize;
    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct TestModel {
        field: String,
    }

    #[test]
    pub fn test_load_configuration_io_error() {
        let path = std::env::current_dir().unwrap().join("doesnotexist.yaml");
        let expected_msg = File::open(&path).unwrap_err().to_string();
        assert_eq!(Err(Error::IOError(expected_msg)), load_configuration::<&PathBuf, TestModel>(&path));
    }

    #[test]
    pub fn test_load_configuration_parse_error() {
        let path = std::env::current_dir().unwrap().join("data-test").join("invalid-configuration.yaml");
        let expected_msg = String::from("invalid type: string \"Blablabla\", expected struct TestModel");
        assert_eq!(Err(Error::ParseError(expected_msg)), load_configuration::<&PathBuf, TestModel>(&path));
    }

    #[test]
    pub fn test_load_configuration() {
        let path = std::env::current_dir().unwrap().join("data-test").join("valid-configuration.yaml");
        assert_eq!(Ok(TestModel{field: String::from("value")}), load_configuration::<&PathBuf, TestModel>(&path));
    }

    #[test]
    pub fn test_save_configuration_io_error() {
        let model = TestModel {field: String::from("value")};
        let path = std::env::current_dir().unwrap().join("target").join("test.yaml");
        assert_eq!(Ok(()), save_configuration(&path, &model));
        assert_eq!(String::from("field: value\n"), std::fs::read_to_string(&path).unwrap());
        std::fs::remove_file(&path).unwrap();
    }

}