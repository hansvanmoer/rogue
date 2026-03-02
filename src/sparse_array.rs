use std::collections::{BTreeSet, LinkedList};
use std::ops::{Deref, DerefMut};

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
    /// Removes a value from the sparse array
    ///
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.buffer.len() {
            let mut replacement = None;
            std::mem::swap(&mut replacement, &mut self.buffer[index]);
            if replacement.is_some() {
                self.free_list.insert(index);
            }
            replacement
        } else {
            None
        }
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
    
    ///
    /// Fetches a mutable reference to the value from the sparse array
    /// 
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.buffer.len() {
            self.buffer[index].as_mut()
        } else {
            None
        }
    }

    ///
    /// An iterator over the elements of the sparse array
    ///
    pub fn iter(&self) -> Iter<T> {
        Iter { array: self, index: 0 }
    }

    ///
    /// A mutable iterator over the elements of the sparse array
    ///
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { array: self, index: 0 }
    }

    ///
    /// Returns the number of elements in the sparse array
    ///
    pub fn len(&self) -> usize {
        self.buffer.len() - self.free_list.len()
    }
}

///
/// An iterator for the sparse array
///
pub struct Iter<'a, T> {
    array: &'a SparseArray<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.array.buffer.len() {
            match &self.array.buffer[self.index] {
                Some(value) => {
                    self.index += 1;
                    return Some(value)
                },
                None => self.index += 1,
            }
        }
        None
    }
}

pub struct IterMut<'a, T> {
    array: &'a mut SparseArray<T>,
    index: usize,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.array.buffer.len() {
            if self.array.buffer[self.index].is_some() {
                let index = self.index;
                self.index += 1;
                let ptr = std::ptr::from_mut(self.array.buffer[index].as_mut().unwrap());
                /*
                 We need unsafe here because we can't get the compiler
                 to understand that the lifetime of the reference will outlive the function
                 and that we will allow only one iterator to exist at a time.
                */
                return Some(unsafe {
                    &mut *ptr
                });
            } else {
                self.index += 1;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_array() {
        let mut array = SparseArray::new();
        assert_eq!(0, array.insert(1));
        assert_eq!(1, array.insert(2));
        assert_eq!(2, array.insert(3));
        assert_eq!(3, array.insert(4));
        assert_eq!(4, array.len());
        assert_eq!(Some(&1), array.get(0));
        assert_eq!(Some(&2), array.get(1));
        assert_eq!(Some(&3), array.get(2));
        assert_eq!(Some(&4), array.get(3));
        assert_eq!(None, array.get(4));

        assert_eq!(Some(2), array.remove(1));
        assert_eq!(None, array.remove(1));
        assert_eq!(3, array.len());
        assert_eq!(None, array.get(1));

        assert_eq!(Some(3), array.remove(2));
        assert_eq!(2, array.len());
        assert_eq!(1, array.insert(5));
        assert_eq!(3, array.len());

        assert_eq!(vec![1, 5, 4], array.iter().copied().collect::<Vec<_>>())
    }

    #[test]
    fn test_sparse_array_mut() {
        let mut array = SparseArray::new();
        assert_eq!(0, array.insert(1));
        assert_eq!(1, array.insert(2));
        assert_eq!(2, array.insert(3));
        assert_eq!(3, array.insert(4));

        array.remove(1);

        array.iter_mut().for_each(|value| *value *= 2);
        assert_eq!(vec![2, 6, 8], array.iter().copied().collect::<Vec<_>>())
    }
}