use std::fmt::Debug;
use std::ops::Sub;
use crate::geometry::AdditiveGroup;
use crate::geometry::bounds3::Bounds3;

///
/// A width, height tuple that is always positive
///
#[derive(Debug, PartialEq, Clone)]
pub struct Dimensions3<T: AdditiveGroup> {
    ///
    /// The width
    ///
    width: T,

    ///
    /// The height
    ///
    height: T,

    ///
    /// The depth
    ///
    depth: T,
}

impl<T: AdditiveGroup> Dimensions3<T> {
    ///
    /// Constructs a new dimensions instance
    ///
    pub fn new(width: T, height: T, depth: T) -> Self {
        Dimensions3 {
            width: T::abs(width),
            height: T::abs(height),
            depth: T::abs(depth),
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

    ///
    /// Casts the dimensions into another type
    ///
    pub fn cast<U: AdditiveGroup + Clone + Copy + Debug + Sub, F: Fn(T) -> U>(&self, f: F) -> Dimensions3<U>{
        Dimensions3 {
            width: f(self.width),
            height: f(self.height),
            depth: f(self.depth),
        }
    }

    ///
    /// Converts the dimensions into a bounding rectangle
    ///
    pub fn into_bounds(self) -> Bounds3<T> {
        Bounds3::new(AdditiveGroup::IDENTITY, self.width, AdditiveGroup::IDENTITY, self.height, AdditiveGroup::IDENTITY, self.depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_dimensions3_new() {
        let expected = Dimensions3 {
            width: 2,
            height: 4,
            depth: 6,
        };
        assert_eq!(expected, Dimensions3::<i32>::new(-2, -4, -6));
        assert_eq!(expected, Dimensions3::<i32>::new(2, 4, 6));
        assert_eq!(&2, expected.get_width());
        assert_eq!(&4, expected.get_height());
        assert_eq!(&6, expected.get_depth());
    }

    #[test]
    pub fn test_into_bounds() {
        let dimensions = Dimensions3 {
            width: 2,
            height: 4,
            depth: 6,
        }.into_bounds();
        let expected = Bounds3::new(0, 2, 0, 4, 0, 6);
        assert_eq!(expected, dimensions);
    }
}