use std::fmt::Debug;
use crate::geometry::AdditiveGroup;

///
/// A width, height tuple that is always positive
///
#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions2<T: Debug + AdditiveGroup> {
    ///
    /// The width
    ///
    width: T,

    ///
    /// The height
    ///
    height: T,
}

impl<T: AdditiveGroup + Debug> Dimensions2<T> {
    ///
    /// Constructs a new dimensions instance
    ///
    pub fn new(width: T, height: T) -> Self {
        Dimensions2 {
            width: T::abs(width),
            height: T::abs(height),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_dimensions2_new() {
        let expected = Dimensions2 {
            width: 2,
            height: 4,
        };
        assert_eq!(expected, Dimensions2::<i32>::new(-2, -4));
        assert_eq!(expected, Dimensions2::<i32>::new(2, 4));
        assert_eq!(&2, expected.get_width());
        assert_eq!(&4, expected.get_height());
    }
}