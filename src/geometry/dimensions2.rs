use std::fmt::Debug;
use std::ops::{Add, Sub};
use crate::geometry::{AdditiveGroup, Bounds2};

///
/// A width, height tuple that is always positive
///
#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions2<T: AdditiveGroup> {
    ///
    /// The width
    ///
    width: T,

    ///
    /// The height
    ///
    height: T,
}

impl<T: AdditiveGroup> Dimensions2<T> {
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

    ///
    /// Casts the dimensions into another type
    ///
    pub fn cast<U: AdditiveGroup + Clone + Copy + Debug + Sub, F: Fn(T) -> U>(&self, f: F) -> Dimensions2<U>{
        Dimensions2 {
            width: f(self.width),
            height: f(self.height),
        }
    }

    ///
    /// Converts the dimensions into a bounding rectangle
    ///
    pub fn into_bounds(self) -> Bounds2<T> {
        Bounds2::new(AdditiveGroup::IDENTITY, self.width, AdditiveGroup::IDENTITY, self.height)
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

    #[test]
    pub fn test_into_bounds() {
        let dimensions = Dimensions2 {
            width: 2,
            height: 4,
        }.into_bounds();
        let expected = Bounds2::new(0, 2, 0, 4);
        assert_eq!(expected, dimensions);
    }

    #[test]
    pub fn test_cast() {
        let original = Dimensions2::<i32>::new(2, 4);
        let casted: Dimensions2<f32> = original.cast(|i| i as f32);
        let recasted = casted.cast(|f| f as i32);
        assert_eq!(original, recasted);
    }
}