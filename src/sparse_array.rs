use std::collections::{BTreeSet};

///
/// A sparse array
///
pub struct SparseArray<T> {
    buffer: Vec<Option<T>>,
    free_list: BTreeSet<usize>,
}

impl<T> SparseArray<T> {
    ///
    /// Creates a new sparse array
    ///
    pub fn new() -> Self {
        Self { buffer: Vec::new(), free_list: BTreeSet::new() }
    }

    ///
    /// Inserts a new value into the sparse array
    ///
    pub fn insert(&mut self, value: T) -> usize {
        let len = self.buffer.len();
        let index = self.free_list.pop_first().unwrap_or(len);
        if index == len {
            self.buffer.push(Some(value));
        } else {
            self.buffer[index] = Some(value);
        }
        index
    }

    ///
    /// Fetches the value from the sparse array
    ///
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.buffer.len() {
            self.buffer[index].as_ref()
        } else {
            None
        }
    }
}