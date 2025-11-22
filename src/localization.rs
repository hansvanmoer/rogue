use std::collections::HashMap;
use std::path::{Path, PathBuf};
use log::{debug, error};
use crate::configuration::{Error as ConfigurationError, load_configuration};
use crate::resource::ResourceMap;
use crate::settings::Settings;

///
/// Loads localized
///
pub fn load_labels_for_language(path: &mut PathBuf, settings: &Settings) -> Result<ResourceMap<String>, Error> {
    find_label_file(path, settings)?;
    load_labels(&path).map(|l| {
        path.pop();
        l
    })
}

///
/// Finds a label file
///
fn find_label_file(path: &mut PathBuf, settings: &Settings) -> Result<(), Error> {
    path.push(settings.get_language_id());
    path.with_added_extension("yaml");
    if path.is_file() {
        debug!("Found labels file {}", path.display());
        Ok(())
    } else {
        path.pop();
        path.push("default.yaml");
        if path.is_file() {
            debug!("Found labels file {}", path.display());
            Ok(())
        } else {
            path.pop();
            error!("Found no labels in folder {}", path.display());
            Err(Error::FileForLanguageNotFound)
        }
    }
}

///
/// Loads labels into an existing resource map
///
pub fn load_labels_into<P: AsRef<Path>>(path: P, map: &mut ResourceMap<String>) -> Result<(), Error> {
    let mut model: HashMap<String, String> = load_configuration(path)?;
    model.drain().for_each(|(k, v)| {
        map.insert(k, v);
    });
    Ok(())
}

///
/// Loads labels
///
pub fn load_labels<P: AsRef<Path>>(path: P) -> Result<ResourceMap<String>, Error> {
    let mut labels = ResourceMap::new();
    load_labels_into(path, &mut labels)?;
    Ok(labels)
}

///
/// Errors that can occur when labels are loaded
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// A configuration error
    ///
    Configuration(ConfigurationError),

    ///
    /// The file for the specified language was not found
    ///
    FileForLanguageNotFound
}

impl From<ConfigurationError> for Error {
    fn from(e: ConfigurationError) -> Self {
        Error::Configuration(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_load_labels() {
        let path = std::env::current_dir().unwrap();
        let mut map = ResourceMap::new();
        load_labels_into(&path.join("data-test").join("localization1.yaml"), &mut map).unwrap();
        load_labels_into(&path.join("data-test").join("localization2.yaml"), &mut map).unwrap();
        let id1 = map.get_id_by_name("label1").unwrap();
        assert_eq!(Some(&String::from("value1")), map.get_by_id(id1));
        let id2 = map.get_id_by_name("label2").unwrap();
        assert_eq!(Some(&String::from("value3")), map.get_by_id(id2));
        assert_eq!(2, map.len());
    }
}