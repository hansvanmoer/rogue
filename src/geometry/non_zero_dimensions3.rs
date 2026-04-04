use std::fmt::Debug;
use crate::geometry::{AdditiveGroup, Error};

///
/// A width, height tuple that is positive and non zero
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonZeroDimensions3<T: AdditiveGroup> {
    ///
    /// The width along the x axis
    ///
    width: T,

    ///
    /// The height along the y axis
    ///
    height: T,

    ///
    /// The depth along the z axis
    ///
    depth: T,
}

impl<T: AdditiveGroup> NonZeroDimensions3<T> {
    ///
    /// Constructs a new dimension object, if possible
    ///
    pub fn new(width: T, height: T, depth: T) -> Result<Self, Error> {
        if width > T::IDENTITY {
            if height > T::IDENTITY {
                if depth > T::IDENTITY {
                    Ok(NonZeroDimensions3 { width, height, depth })
                } else {
                    Err(Error::InvalidDepth)
                }
            } else {
                Err(Error::InvalidHeight)
            }
        } else {
            Err(Error::InvalidWidth)
        }
    }

    ///
    /// Gets the width
    ///
    pub fn get_width(&self) -> &T {
        &self.width
    }

    ///
    /// Gets the height
    ///
    pub fn get_height(&self) -> &T {
        &self.height
    }

    ///
    /// Gets the depth
    ///
    pub fn get_depth(&self) -> &T {
        &self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_non_zero_dimensions3i() {
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions3::<i32>::new(-2, 4, 3));
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions3::<i32>::new(0, 4, 3));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions3::<i32>::new(2, -4, 3));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions3::<i32>::new(2, 0, 3));
        assert_eq!(Err(Error::InvalidDepth), NonZeroDimensions3::<i32>::new(2, 4, 0));
        assert_eq!(Err(Error::InvalidDepth), NonZeroDimensions3::<i32>::new(2, 4, -3));
        let created = NonZeroDimensions3::<i32>::new(2, 4, 3);
        let expected = NonZeroDimensions3 {
            width: 2,
            height: 4,
            depth: 3,
        };
        assert_eq!(Ok(expected), created);
        let created = created.unwrap();
        assert_eq!(&2, created.get_width());
        assert_eq!(&4, created.get_height());
        assert_eq!(&3, created.get_depth());
    }
}