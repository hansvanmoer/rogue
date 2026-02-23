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
/// A resource ID and name
///
#[derive(Debug, PartialEq)]
pub struct ResourceRef {
    id: ResourceId,
    name: String,
}

impl ResourceRef {
    ///
    /// Creates a new resource reference
    /// 
    pub fn new(id: ResourceId, name: String) -> ResourceRef {
        ResourceRef { id, name }
    }
}

///
/// Maps resources on ID's and names
///
pub struct ResourceMap<T> {
    ///
    /// Resources by name
    ///
    by_name: HashMap<String, usize>,

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
            by_name: HashMap::new(),
            by_id: Vec::new(),
        }
    }

    ///
    /// Inserts a new resource with a unique name
    ///
    pub fn insert_if_not_present(&mut self, name: String, resource: T) -> Result<ResourceId, Error> {
        if self.by_name.contains_key(&name) {
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
        let id = self.by_name.get(&name).map(|i| *i).unwrap_or(len);
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
        self.by_name.get(name).copied()
    }

    ///
    /// Gets a resource by ID
    ///
    pub fn get_required_id_by_name(&self, name: &str) -> Result<ResourceId, Error> {
        self.get_id_by_name(name).ok_or_else(|| Error::NotFoundForName(String::from(name)))
    }

    ///
    /// Creates a resource reference
    /// 
    pub fn create_resource_ref(&self, name: &str) -> Result<ResourceRef, Error> {
        Ok(ResourceRef::new(self.get_required_id_by_name(name)?, String::from(name)))
    }
    
    ///
    /// Fetches a resource by reference
    /// 
    pub fn get_required_by_ref(&self, resource_ref: &ResourceRef) -> Result<&T, Error> {
        self.get_by_id(resource_ref.id).ok_or_else(|| Error::NotFoundForId(resource_ref.id))
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
        self.by_name.get(name).and_then(|id| self.by_id.get(*id))
    }

    ///
    /// Gets a resource by name or an error
    ///
    pub fn get_required_by_name(&self, name: &str) -> Result<&T, Error> {
        self.get_by_name(name).ok_or_else(|| Error::NotFoundForName(String::from(name)))
    }

    ///
    /// The number of resources
    ///
    pub fn len(&self) -> usize {
        self.by_id.len()
    }
}

impl<T> From<ResourceBuffer<T>> for ResourceMap<T> {
    fn from(mut buffer: ResourceBuffer<T>) -> Self {
        let len = buffer.names.len();
        let mut by_name = HashMap::with_capacity(len);
        let mut id = 0;
        buffer.names.drain(0..len).for_each(|name| {
            by_name.insert(name, id);
            id += 1;
        });
        ResourceMap {
            by_name,
            by_id: buffer.resources,
        }
    }
}

///
/// Stores resources by ID and names to reconstruct the resource map when saving
///
pub struct ResourceBuffer<T> {
    ///
    /// Resources in a contiguous buffer
    ///
    resources: Vec<T>,

    ///
    /// The names in a contiguous buffer
    ///
    names: Vec<String>,
}

impl<T> ResourceBuffer<T> {
    ///
    /// Gets a resource by ID
    ///
    pub fn get_by_id(&self, id: ResourceId) -> Option<&T> {
        self.resources.get(id)
    }

    ///
    /// The number of resources
    ///
    pub fn len(&self) -> usize {
        self.resources.len()
    }
}

impl<T> From<ResourceMap<T>> for ResourceBuffer<T> {
    fn from(mut map: ResourceMap<T>) -> Self {
        let mut names = vec![String::new(); map.by_name.len()];
        map.by_name.drain().for_each(|(name, id)| {
            names[id] = name;
        });
        ResourceBuffer {
            resources: map.by_id,
            names,
        }
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
    type Resource: LoadableResource<Self, Self::State, Self::Error>;

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
    fn from_folder_path(state: &Self::State, path: &mut PathBuf) -> Result<ResourceMap<Self::Resource>, Self::Error> {
        debug!("Loading {} resources from folder {} recursively", Self::resource_type_name(), path.display());
        let mut resources = ResourceMap::new();
        Self::from_folder_path_recursive(&mut resources, state, path)?;
        debug!("Loaded {} {} resources from folders", resources.len(), Self::resource_type_name());
        Ok(resources)
    }

    ///
    /// Loads all resources from a folder recursively
    ///
    fn from_folder_path_recursive(
        resources: &mut ResourceMap<Self::Resource>,
        state: &Self::State,
        path: &mut PathBuf,
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
                    Self::from_folder_path_recursive(resources, state, path)?;
                    path.pop();
                } else {
                    error!("no such folder: {}", path.display());
                    Err(crate::configuration::Error::IOError(format!("no such folder: {}", path.display())))?;
                }
            }
        }

        debug!("Loading {} resources from folder {}", resource_name, path.display());
        Self::Resource::load(resources, state, descriptor)
    }
}

///
/// A resource that can be loaded from a descriptor
///
pub trait LoadableResource<D, S, E>: Sized {

    ///
    /// Loads a resource from a descriptor and a given immutable state into a resource map
    ///
    fn load(resources: &mut ResourceMap<Self>, state: &S, descriptor: D) -> Result<(), E>;
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

        let buffer: ResourceBuffer<String> = map.into();
        assert_eq!(3, buffer.len());
        assert_eq!(Some(&String::from("resource0")), buffer.get_by_id(0));
        assert_eq!(Some(&String::from("resource1")), buffer.get_by_id(1));
        assert_eq!(Some(&String::from("resource2")), buffer.get_by_id(2));
        assert_eq!(None, buffer.get_by_id(3));

        let map: ResourceMap<String> = buffer.into();
        assert_eq!(3, map.len());
        assert_eq!(Some(0), map.get_id_by_name("name0"));
        assert_eq!(Some(1), map.get_id_by_name("name1"));
        assert_eq!(Some(2), map.get_id_by_name("name2"));
        assert_eq!(None, map.get_id_by_name("name3"));
    }

}