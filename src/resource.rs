use std::collections::HashMap;

use crate::resource::Error::DuplicateName;

///
/// A resource ID type
///
type ResourceId = usize;

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
    pub fn get_by_id(&self, id: ResourceId) -> Option<&T> {
        self.by_id.get(id)
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