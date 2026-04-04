use std::fmt::Debug;
use std::ops::{Add, Sub};
use crate::geometry::AdditiveGroup;

#[derive(Debug, PartialEq)]
pub struct Bounds2<T: AdditiveGroup> {
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
}

impl<T: AdditiveGroup> Bounds2<T> {

    ///
    /// Constructs a new bounding rectangle
    ///
    pub fn new(x1: T, x2: T, y1: T, y2: T) -> Self {
        if x1 <= x2 {
            if y1 <= y2 {
                Bounds2 { min_x: x1, max_x: x2, min_y: y1, max_y: y2 }
            } else {
                Bounds2 { min_x: x1, max_x: x2, min_y: y2, max_y: y1 }
            }
        } else {
            if y1 <= y2 {
                Bounds2 { min_x: x2, max_x: x1, min_y: y1, max_y: y2 }
            } else {
                Bounds2 { min_x: x2, max_x: x1, min_y: y2, max_y: y1 }
            }
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
    /// The min x coordinate
    ///
    pub fn get_min_y(&self) -> T {
        self.min_y
    }

    ///
    /// The min x coordinate
    ///
    pub fn get_max_y(&self) -> T {
        self.max_y
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
    /// Checks whether the given point is within the bounds
    /// 
    pub fn is_within_bounds(&self, x: T, y: T) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    ///
    /// Adds a margin to the bounds
    ///
    pub fn add_margin(&self, margin: T) -> Self {
        Bounds2 {
            min_x: self.min_x - margin,
            max_x: self.max_x + margin,
            min_y: self.min_y - margin,
            max_y: self.max_y + margin,
        }
    }

    ///
    /// Casts the bounds to a new type
    ///
    pub fn cast<U: AdditiveGroup, F: Fn(T) -> U>(&self, f: F) -> Bounds2<U> {
        Bounds2 {
            min_x: f(self.min_x),
            max_x: f(self.max_x),
            min_y: f(self.min_y),
            max_y: f(self.max_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds2_new() {
        let expected = Bounds2 {
            min_x: 1,
            max_x: 2,
            min_y: 3,
            max_y: 4,
        };
        assert_eq!(expected, Bounds2::new(1, 2, 3, 4));
        assert_eq!(expected, Bounds2::new(2, 1, 4, 3));
        assert_eq!(expected, Bounds2::new(1, 2, 4, 3));
        assert_eq!(expected, Bounds2::new(2, 1, 3, 4));
        assert_eq!(1, expected.get_x_difference());
        assert_eq!(1, expected.get_y_difference());
    }

    #[test]
    fn test_bounds2_cast() {
        let expected = Bounds2 {
            min_x: 1,
            max_x: 2,
            min_y: 3,
            max_y: 4,
        };
        let bounds = Bounds2::new(1, 2, 3, 4);
        let casted = bounds.cast(|i| i as f32);
        let casted_back = casted.cast(|f| f as i32);
        assert_eq!(expected, casted_back);
    }
    
    #[test]
    fn test_bounds2_is_within_bounds() {
        let bounds = Bounds2::new(1.0, 2.0, 3.0, 4.0);
        assert!(bounds.is_within_bounds(1.5, 3.5));
        assert!(!bounds.is_within_bounds(0.9, 3.5));
        assert!(!bounds.is_within_bounds(1.5, 4.5));
    }
}