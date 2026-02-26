use std::collections::HashMap;
use std::path::PathBuf;
use log::{debug, error};
use serde::de::DeserializeOwned;
use crate::configuration::load_configuration;
use crate::resource::Error::DuplicateName;
use crate::validation::ValidateOwned;

///
/// A resource ID type
///
pub type ResourceId = usize;

///
/// A map of resource names to ID's
///
pub struct ResourceIdMap {
    map: HashMap<String, ResourceId>,
}

impl ResourceIdMap {
    
    ///
    /// Creates a new resource ID map
    /// 
    fn new() -> ResourceIdMap {
        ResourceIdMap {
            map: HashMap::new()
        }
    }
    
    ///
    /// Insert an ID into the resource ID map
    /// 
    fn insert(&mut self, name: String, id: ResourceId) -> Option<ResourceId> {
        self.map.insert(name, id)
    }

    ///
    /// The amount of ID's in the resource ID map
    /// 
    pub fn len(&self) -> usize {
        self.map.len()
    }
    
    ///
    /// Gets a resource ID by name
    /// 
    pub fn get_by_name(&self, name: &str) -> Option<ResourceId> {
        self.map.get(name).copied()
    }
    
    ///
    /// Gets a required resource ID by name
    /// 
    pub fn get_required_by_name(&self, name: &str) -> Result<ResourceId, Error> {
        self.get_by_name(name).ok_or_else(|| Error::NotFoundForName(String::from(name)))
    }
    
    ///
    /// Whether the resource ID map contains a name
    /// 
    pub fn contains_name(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
}

///
/// Maps resources on ID's and names
///
pub struct ResourceMap<T> {
    ///
    /// Resources by name
    ///
    by_name: ResourceIdMap,

    ///
    /// Resources by ID
    ///
    by_id: Vec<T>,
}

impl<T> ResourceMap<T> {
    ///
    /// Creates an empty resource map
    ///
    pub fn new() -> Self {
        ResourceMap {
            by_name: ResourceIdMap::new(),
            by_id: Vec::new(),
        }
    }

    ///
    /// Returns the ID map
    /// 
    pub fn get_resource_id_map(&self) -> &ResourceIdMap {
        &self.by_name
    }
    
    ///
    /// Inserts a new resource with a unique name
    ///
    pub fn insert_if_not_present(&mut self, name: String, resource: T) -> Result<ResourceId, Error> {
        if self.by_name.contains_name(&name) {
            Err(DuplicateName(name))
        } else {
            let id = self.by_id.len();
            self.by_id.push(resource);
            self.by_name.insert(name, id);
            Ok(id)
        }
    }

    ///
    /// Inserts a new resource with a unique name, overwriting a previous entry if present
    ///
    pub fn insert(&mut self, name: String, resource: T) -> ResourceId {
        let len = self.by_id.len();
        let id = self.by_name.get_by_name(&name).unwrap_or(len);
        if id == len {
            let id = self.by_id.len();
            self.by_id.push(resource);
            self.by_name.insert(name, id);
        } else {
            self.by_id[id] = resource;
        }
        id
    }

    ///
    /// Gets a resource by name
    ///
    pub fn get_id_by_name(&self, name: &str) -> Option<usize> {
        self.by_name.get_by_name(name)
    }

    ///
    /// Gets a resource by ID
    ///
    pub fn get_required_id_by_name(&self, name: &str) -> Result<ResourceId, Error> {
        self.get_id_by_name(name).ok_or_else(|| Error::NotFoundForName(String::from(name)))
    }

    ///
    /// Gets a resource by ID
    ///
    pub fn get_by_id(&self, id: ResourceId) -> Option<&T> {
        self.by_id.get(id)
    }

    ///
    /// Gets a resource by ID
    ///
    pub fn get_required_by_id(&self, id: ResourceId) -> Result<&T, Error> {
        self.by_id.get(id).ok_or_else(|| Error::NotFoundForId(id))
    }

    ///
    /// Gets a resource by name
    ///
    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        self.by_name.get_by_name(name).and_then(|id| self.by_id.get(id))
    }

    ///
    /// Gets a resource by name or an error
    ///
    pub fn get_required_by_name(&self, name: &str) -> Result<&T, Error> {
        self.get_by_name(name).ok_or_else(|| Error::NotFoundForName(String::from(name)))
    }

    ///
    /// Gets a resource by name or an error
    ///
    pub fn get_required_entry_by_name(&self, name: &str) -> Result<(ResourceId, &T), Error> {
        let id = self.get_required_id_by_name(name)?;
        Ok((id, &self.by_id[id]))
    }
    
    ///
    /// The number of resources
    ///
    pub fn len(&self) -> usize {
        self.by_id.len()
    }
}

///
/// A simple resource descriptor
///
pub trait ResourceDescriptor: Sized {
    ///
    /// The error type for resource loading errors
    ///
    type Error: From<crate::configuration::Error> + From<Error> + From<crate::validation::Error>;

    ///
    /// The configuration type
    ///
    type Configuration: DeserializeOwned + ValidateOwned<Output = Self>;

    ///
    /// The state type for any immutable state used to load resources
    ///
    type State;

    ///
    /// The resource type
    ///
    type Resource;

    ///
    /// The resource type name, used for logging and error messages
    ///
    fn resource_type_name() -> &'static str;

    ///
    /// The main descriptor file name
    ///
    fn main_descriptor_file_name() -> &'static str;

    ///
    /// Gets the list of files to load from the descriptor folder
    ///
    fn get_files(&self) -> &Vec<String>;

    ///
    /// Gets the list of folders to load from the descriptor folder
    ///
    fn get_folders(&self) -> &Vec<String>;

    ///
    /// Loads all resources from a folder
    ///
    fn from_folder_path<F: Fn(&mut ResourceMap<Self::Resource>, <Self::Configuration as ValidateOwned>::Output) -> Result<(), Self::Error>>(path: &mut PathBuf, load: &F) -> Result<ResourceMap<Self::Resource>, Self::Error> {
        debug!("Loading {} resources from folder {} recursively", Self::resource_type_name(), path.display());
        let mut resources = ResourceMap::new();
        Self::from_folder_path_recursive(&mut resources, path, load)?;
        debug!("Loaded {} {} resources from folders", resources.len(), Self::resource_type_name());
        Ok(resources)
    }

    ///
    /// Loads all resources from a folder recursively
    ///
    fn from_folder_path_recursive<F: Fn(&mut ResourceMap<Self::Resource>, <Self::Configuration as ValidateOwned>::Output) -> Result<(), Self::Error>>(
        resources: &mut ResourceMap<Self::Resource>,
        path: &mut PathBuf,
        load: &F
    ) -> Result<(), Self::Error> {
        let resource_name= Self::resource_type_name();
        let file_name = Self::main_descriptor_file_name();
        debug!("Loading {} descriptor from folder {}", resource_name, path.display());
        path.push(file_name);
        path.set_extension("yaml");
        let config: Self::Configuration = load_configuration(&path)?;
        path.pop();
        let descriptor = config.validate_owned()?;

        if !descriptor.get_folders().is_empty() {
            debug!("Loading {} resources from {} subfolders...", resource_name, descriptor.get_folders().len());
            for folder in descriptor.get_folders() {
                path.push(folder);
                if path.is_dir() {
                    Self::from_folder_path_recursive(resources, path, load)?;
                    path.pop();
                } else {
                    error!("no such folder: {}", path.display());
                    Err(crate::configuration::Error::IOError(format!("no such folder: {}", path.display())))?;
                }
            }
        }

        debug!("Loading {} resources from folder {}", resource_name, path.display());
        load(resources, descriptor)
    }
}

///
/// Errors that can occur when loading resources
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// A resource with this name already exists
    ///
    DuplicateName(String),

    ///
    /// A resource was not found for a specified name
    ///
    NotFoundForName(String),

    ///
    /// A resource was not found for a specified ID
    ///
    NotFoundForId(ResourceId),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_resources() {
        let mut map = ResourceMap::new();
        assert_eq!(0, map.len());
        assert_eq!(Ok(0), map.insert_if_not_present(String::from("name0"), String::from("resource0")));
        assert_eq!(1, map.len());
        assert_eq!(Err(Error::DuplicateName(String::from("name0"))), map.insert_if_not_present(String::from("name0"), String::from("resource0")));
        assert_eq!(1, map.len());
        assert_eq!(Ok(1), map.insert_if_not_present(String::from("name1"), String::from("old_resource1")));
        assert_eq!(Some(1), map.get_id_by_name("name1"));
        assert_eq!(1, map.insert(String::from("name1"), String::from("resource1")));
        assert_eq!(2, map.len());
        assert_eq!(Some(1), map.get_id_by_name("name1"));
        assert_eq!(2, map.len());
        assert_eq!(2, map.insert(String::from("name2"), String::from("resource2")));
        assert_eq!(Some(0), map.get_id_by_name("name0"));
        assert_eq!(Some(1), map.get_id_by_name("name1"));
        assert_eq!(Some(2), map.get_id_by_name("name2"));
        assert_eq!(None, map.get_id_by_name("name3"));

        let id_map = map.get_resource_id_map();
        assert_eq!(3, id_map.len());
        assert_eq!(Some(0), id_map.get_by_name("name0"));
        assert_eq!(Some(1), id_map.get_by_name("name1"));
        assert_eq!(Some(2), id_map.get_by_name("name2"));
        assert_eq!(None, id_map.get_by_name("name3"));
    }

}