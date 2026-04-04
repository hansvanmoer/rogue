use std::fmt::Debug;
use crate::geometry::{AdditiveGroup, Error};

///
/// A width, height tuple that is positive and non zero
///
#[derive(Debug, PartialEq, Clone)]
pub struct NonZeroDimensions2<T: AdditiveGroup> {
    ///
    /// The width
    ///
    width: T,

    ///
    /// The height
    ///
    height: T,
}

impl<T: AdditiveGroup> NonZeroDimensions2<T> {
    ///
    /// Constructs a new dimension object, if possible
    ///
    pub fn new(width: T, height: T) -> Result<Self, Error> {
        if width > T::IDENTITY {
            if height > T::IDENTITY {
                Ok(NonZeroDimensions2 { width, height })
            } else {
                Err(Error::InvalidHeight)
            }
        } else {
            Err(Error::InvalidWidth)
        }
    }

    ///
    /// Gets the
    ///
    pub fn get_width(&self) -> &T {
        &self.width
    }

    pub fn get_height(&self) -> &T {
        &self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_non_zero_dimensions2i() {
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions2::<i32>::new(-2, 4));
        assert_eq!(Err(Error::InvalidWidth), NonZeroDimensions2::<i32>::new(0, 4));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions2::<i32>::new(2, -4));
        assert_eq!(Err(Error::InvalidHeight), NonZeroDimensions2::<i32>::new(2, 0));
        let created = NonZeroDimensions2::<i32>::new(2, 4);
        let expected = NonZeroDimensions2 {
            width: 2,
            height: 4,
        };
        assert_eq!(Ok(expected), created);
        let created = created.unwrap();
        assert_eq!(&2, created.get_width());
        assert_eq!(&4, created.get_height());
    }
}