use std::fmt::Debug;
use std::ops::{Add, Sub};
use crate::geometry::AdditiveGroup;
use crate::geometry::vector3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Bounds3<T: AdditiveGroup> {
    ///
    /// The left border
    ///
    min_x: T,

    ///
    /// THe right border
    ///
    max_x: T,

    ///
    /// The top border
    ///
    min_y: T,

    ///
    /// The bottom border
    ///
    max_y: T,

    ///
    /// The minimum z coordinate
    ///
    min_z: T,

    ///
    /// The maximum z coordinate
    ///
    max_z: T,
}

impl<T: AdditiveGroup> Bounds3<T> {

    ///
    /// Constructs a new bounding rectangle
    ///
    pub fn new(x1: T, x2: T, y1: T, y2: T, z1: T, z2: T) -> Self {
        let (min_x, max_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (min_y, max_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        let (min_z, max_z) = if z1 <= z2 { (z1, z2) } else { (z2, z1) };

        Bounds3 {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    ///
    /// The min x coordinate
    ///
    pub fn get_min_x(&self) -> T {
        self.min_x
    }

    ///
    /// The min x coordinate
    ///
    pub fn get_max_x(&self) -> T {
        self.max_x
    }

    ///
    /// The min y coordinate
    ///
    pub fn get_min_y(&self) -> T {
        self.min_y
    }

    ///
    /// The min y coordinate
    ///
    pub fn get_max_y(&self) -> T {
        self.max_y
    }

    ///
    /// The min z coordinate
    ///
    pub fn get_min_z(&self) -> T {
        self.min_z
    }

    ///
    /// The min z coordinate
    ///
    pub fn get_max_z(&self) -> T {
        self.max_z
    }

    ///
    /// The difference between the x coordinates
    ///
    pub fn get_x_difference(&self) -> T {
        self.max_x - self.min_x
    }

    ///
    /// The difference between the y coordinates
    ///
    pub fn get_y_difference(&self) -> T {
        self.max_y - self.min_y
    }

    ///
    /// The difference between the z coordinates
    ///
    pub fn get_z_difference(&self) -> T {
        self.max_z - self.min_z
    }

    ///
    /// Checks whether the point is within the bounds
    ///
    pub fn point_is_within(&self, point: &Vector3<T>) -> bool {
        point.get_x() >= self.min_x && point.get_x() <= self.max_x &&
            point.get_y() >= self.min_y && point.get_y() <= self.max_y &&
            point.get_z() >= self.min_z && point.get_z() <= self.max_z
    }

    ///
    /// Casts the bounds to a new type
    ///
    pub fn cast<U: AdditiveGroup, F: Fn(T) -> U>(&self, f: F) -> Bounds3<U> {
        Bounds3 {
            min_x: f(self.min_x),
            max_x: f(self.max_x),
            min_y: f(self.min_y),
            max_y: f(self.max_y),
            min_z: f(self.min_z),
            max_z: f(self.max_z),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds3_new() {
        let expected = Bounds3 {
            min_x: 1,
            max_x: 2,
            min_y: 3,
            max_y: 4,
            min_z: 5,
            max_z: 6,
        };
        assert_eq!(expected, Bounds3::new(1, 2, 3, 4, 5, 6));
        assert_eq!(expected, Bounds3::new(2, 1, 4, 3, 5, 6));
        assert_eq!(expected, Bounds3::new(2, 1, 3, 4, 5, 6));
        assert_eq!(expected, Bounds3::new(1, 2, 4, 3, 6, 5));

        assert_eq!(1, expected.get_x_difference());
        assert_eq!(1, expected.get_y_difference());
    }

    #[test]
    fn test_bounds3_cast() {
        let expected = Bounds3 {
            min_x: 1,
            max_x: 2,
            min_y: 3,
            max_y: 4,
            min_z: 5,
            max_z: 6,
        };
        let bounds = Bounds3::new(1, 2, 3, 4, 5, 6);
        let casted = bounds.cast(|i| i as f32);
        let casted_back = casted.cast(|f| f as i32);
        assert_eq!(expected, casted_back);
    }
}